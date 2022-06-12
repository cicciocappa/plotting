#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::plot::{Line, Plot, Points, Value, Values};
use std::io::BufRead;

fn main() {
   let mut options = eframe::NativeOptions::default();
   options.maximized = true;
   eframe::run_native(
      "Cortisolo",
      options,
      Box::new(|_cc| Box::new(MyApp::default())),
   );
}

struct Point {
   x: f32,
   y: u32,
   delta: i32,
}

struct MyApp {
   data_loaded: bool,
   data: Vec<Point>,
   min_limit_fq: u32,
   max_limit_fq: u32,
   min_fq: u32,
   max_fq: u32,
   delta: u32,
}

impl MyApp {
   fn load_data(&mut self, path: String) {
      let mut maxfq = 0;
      let mut minfq = u32::MAX;
      let f = std::fs::File::open(path);
      if f.is_err() {
         return;
      }
      self.data.clear();
      let mut time: f32 = 5.1;
      let f = std::io::BufReader::new(f.unwrap());
      for line in f.lines() {
         if line.is_ok() {
            let v = line.unwrap();
            let fq = v.trim().parse::<u32>();
            if fq.is_ok() {
               let fq = fq.unwrap();
               //println!("{}",fq.unwrap());
               //self.data.push(Point{x:time, y:fq.unwrap()));
               self.data.push(Point {
                  x: time,
                  y: fq,
                  delta: 0,
               });
               if fq < minfq {
                  minfq = fq;
               }
               if fq > maxfq {
                  maxfq = fq;
               }
            }
            time += 5.1;
         }
      }
      println!("{minfq} {maxfq}");
      self.min_limit_fq = minfq;
      self.max_limit_fq = maxfq;
      self.min_fq = minfq;
      self.max_fq = maxfq;
      self.data_loaded = true;
   }
}

impl Default for MyApp {
   fn default() -> Self {
      Self {
         data_loaded: false,
         data: Vec::new(),
         min_fq: 0,
         max_fq: 0,
         delta: 0,
         min_limit_fq: 0,
         max_limit_fq: 100,
      }
   }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::SidePanel::right("controlli").show(ctx, |ui| {
         ui.add_space(10.0);
         //let stroke = egui::Stroke {width:4.0, color:egui::Color32::from_rgb(0xff, 0x00, 0x00)};
         ui.vertical_centered(|ui| {
            let button = egui::Button::new("CSV File"); //.fill(egui::Color32::from_rgb(0x33, 0x55, 0xcc)).stroke(stroke);

            if ui.add_sized([70.0, 25.0], button).clicked() {
               if let Some(path) = rfd::FileDialog::new().pick_file() {
                  let csv = path.display().to_string();
                  self.load_data(csv);
               }
            }
         });
         ui.add_space(10.0);
         ui.heading("Limiti");
         egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("Min:");
            if ui
               .add(
                  egui::Slider::new(&mut self.min_fq, self.min_limit_fq..=self.max_limit_fq)
                     .show_value(false),
               )
               .changed()
            {
               // ricalcolo regressione...
            };
            ui.end_row();
            ui.label("Max:");
            ui.add(
               egui::Slider::new(&mut self.max_fq, self.min_limit_fq..=self.max_limit_fq)
                  .show_value(false),
            );
            ui.end_row();
            ui.label("Delta:");
            ui.add(egui::Slider::new(&mut self.delta, 0..=100).show_value(false));
            ui.end_row();
         });
      });

      egui::CentralPanel::default().show(ctx, |ui| {
         if self.data_loaded {
            let serie = self
               .data
               .iter()
               .filter(|p| p.y >= self.min_fq && p.y <= self.max_fq)
               .map(|p| Value::new(p.x, p.y));
            let punti = Points::new(Values::from_values_iter(serie)).radius(4.0);
            Plot::new("my_plot").show(ui, |plot_ui| plot_ui.points(punti));
         }
      });
   }
}
