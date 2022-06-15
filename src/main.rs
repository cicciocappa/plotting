#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod polyregres;

use polyregres::{DataPoint, PolynomialRegression};

use eframe::egui;
use egui::plot::{Line, LineStyle, Plot, Points, VLine, Value, Values};
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

struct Event {
   time: u32,
   label: String,
}

struct MyApp {
   data_loaded: bool,
   data: Vec<Point>,
   regression: Vec<Value>,
   min_limit_fq: u32,
   max_limit_fq: u32,
   min_fq: u32,
   max_fq: u32,
   delta: u32,
   show_data: bool,
   show_regression: bool,
   show_by_event: bool,
   show_interpolated: bool,
   regression_grade: String,
   events: Vec<Event>,
   min: String,
   sec: String,
   name: String,
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
      self.regression.clear();
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
      //println!("{minfq} {maxfq}");
      self.min_limit_fq = minfq;
      self.max_limit_fq = maxfq;
      self.min_fq = minfq;
      self.max_fq = maxfq;

      // calcolo la regressione totale
      self.calculate_regression();

      self.data_loaded = true;
   }

   fn calculate_regression(&mut self) {
      let grado = self.regression_grade.parse::<usize>().unwrap_or(1);
      let data = self
         .data
         .iter()
         .filter(|p| p.y >= self.min_fq && p.y <= self.max_fq)
         .map(|p| DataPoint {
            x: p.x as f64,
            y: p.y as f64,
         })
         .collect();
      let mut poly = PolynomialRegression::new(data, grado);
      let terms = poly.get_terms();
      self.regression = self
         .data
         .iter()
         .map(|p| Value::new(p.x, PolynomialRegression::predict_y(&terms, p.x as f64)))
         .collect();
   }
}

impl Default for MyApp {
   fn default() -> Self {
      Self {
         data_loaded: false,
         data: Vec::new(),
         regression: Vec::new(),
         min_fq: 0,
         max_fq: 0,
         delta: 0,
         min_limit_fq: 0,
         max_limit_fq: 100,
         show_data: true,
         show_regression: false,
         show_by_event: false,
         show_interpolated: false,
         regression_grade: String::from("3"),
         events: Vec::new(),
         min: String::new(),
         sec: String::new(),
         name: String::new(),
        // event_regression: Vec::new(),
        // interp_regression: Vec::new(),
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
         egui::Grid::new("sliders").show(ui, |ui| {
            ui.label("Min:");
            if ui
               .add(
                  egui::Slider::new(&mut self.min_fq, self.min_limit_fq..=self.max_limit_fq)
                     .show_value(false),
               )
               .changed()
            {
               // ricalcolo regressione...
               self.calculate_regression();
            };
            ui.end_row();
            ui.label("Max:");
            if ui
               .add(
                  egui::Slider::new(&mut self.max_fq, self.min_limit_fq..=self.max_limit_fq)
                     .show_value(false),
               )
               .changed()
            {
               // ricalcolo regressione...
               self.calculate_regression();
            };
            ui.end_row();
            ui.label("Delta:");
            ui.add(egui::Slider::new(&mut self.delta, 0..=100).show_value(false));
            ui.end_row();
         });
         ui.add_space(10.0);
         ui.separator();
         ui.heading("Regressione");
         //ui.add_space(10.0);
         ui.checkbox(&mut self.show_data, "Misura");
         ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_regression, "Globale di grado");
            let response = ui.add(egui::TextEdit::singleline(&mut self.regression_grade));
            if response.changed() {
               // println!("{}",self.regression_grade);
               self.regression_grade.retain(|c| c.is_ascii_digit());
               if self.regression_grade.len() > 2 {
                  self.regression_grade.pop();
               }
               self.calculate_regression();
            }
            //if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
            // …
            //}
         });
         ui.checkbox(&mut self.show_by_event, "Per fase");
         ui.checkbox(&mut self.show_interpolated, "Per fase interpolata");
         ui.add_space(10.0);
         ui.separator();
         ui.heading("Eventi");
         ui.vertical_centered(|ui| {
            let button = egui::Button::new("Default"); //.fill(egui::Color32::from_rgb(0x33, 0x55, 0xcc)).stroke(stroke);

            if ui.add_sized([70.0, 25.0], button).clicked() {
               // imposta gli eventi di default
               self.events = vec![
                  Event {
                     label: String::from("12:30 anticorpo"),
                     time: 750,
                  },
                  Event {
                     label: String::from("19:00 bsa"),
                     time: 19 * 60 + 30,
                  },
                  Event {
                     label: String::from("25:30 antigene"),
                     time: 25 * 60 + 30,
                  },
               ]
            }
         });

         let mut del = 1000000;
         for ev in self.events.iter() {
            ui.horizontal(|ui| {
               if ui.button("X").clicked() {
                  del = ev.time;
               };
               ui.label(&ev.label);
            });
         }
         if del != 1000000 {
            self.events.retain(|e| e.time != del);
         }

         ui.horizontal(|ui| {
            // ui.text_edit_singleline(&mut self.min);
            // ui.add_sized([40.0, 20.0], egui::TextEdit::singleline::new(&mut self.min));
            ui.add_sized([40.0, 20.0], egui::TextEdit::singleline(&mut self.min));
            ui.label(":");
            ui.add_sized([40.0, 20.0], egui::TextEdit::singleline(&mut self.sec));
            ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(&mut self.name));
            if ui.button("add").clicked() {
               let min: u32 = self.min.parse().unwrap_or(0);
               let sec: u32 = self.sec.parse().unwrap_or(0);
               let label = format!(
                  "{}:{:02} {}",
                  min,
                  sec,
                  if self.name.is_empty() {
                     String::from("Event")
                  } else {
                     self.name.clone()
                  }
               );
               self.events.push(Event {
                  label,
                  time: min * 60 + sec,
               });
               self.min = String::from("0");
               self.sec = String::from("0");
               self.name = String::new();
            }
         });
      });

      egui::CentralPanel::default().show(ctx, |ui| {
         if self.data_loaded {
            let serie = self
               .data
               .iter()
               .filter(|p| p.y >= self.min_fq && p.y <= self.max_fq)
               .map(|p| Value::new(p.x, p.y));
            let reg = Line::new(Values::from_values_iter(
               self.regression.iter().map(|p| Value::new(p.x, p.y)),
            ))
            .width(4.0);
            let punti = Points::new(Values::from_values_iter(serie)).radius(2.0);
            let p = Plot::new("my_plot").legend(egui::widgets::plot::Legend::default());
            p.show(ui, |plot_ui| {
               if self.show_data {
                  plot_ui.points(punti);
               }
               
               if self.show_regression {
                  plot_ui.line(reg);
               }
               for ev in self.events.iter() {
                  plot_ui.vline(
                     VLine::new(ev.time as f64)
                        .width(3.0)
                        .name(&ev.label)
                        .style(LineStyle::Dashed { length: 3.0 }),
                  );
               }
            });
         }
      });
   }
}
