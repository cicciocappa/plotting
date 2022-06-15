mod matrix {

    use super::Matrix;

    pub fn gaussian_jordan_elimination(left_matrix: &Matrix, right_matrix: &Vec<f64>) -> Vec<f64> {
        let combined = combine_matrices(left_matrix, right_matrix);
        let mut fwd_integration = forward_elimination(&combined);
        let l1 = (fwd_integration.len() - 1) as i32;
        let l2 = (fwd_integration[0].len() - 2) as i32;
        let mut arr = vec![0.0;fwd_integration.len()];
    
        //println!("{l1} {l2}");
        //NOW, FINAL STEP IS BACKWARD SUBSTITUTION WHICH RETURNS THE TERMS NECESSARY FOR POLYNOMIAL REGRESSION
        backward_substitution(&mut fwd_integration, &mut arr, l1, l2).to_vec()
    }
    
    fn combine_matrices(left: &Matrix, right: &Vec<f64>) -> Matrix {
        let rows = right.len();
        let cols = left[0].len();
        let mut return_matrix = Vec::new();
    
        for i in 0..rows {
            return_matrix.push(Vec::new());
    
            for j in 0..=cols {
                if j == cols {
                    return_matrix[i].push(right[i]);
                } else {
                    return_matrix[i].push(left[i][j]);
                }
            }
        }
    
        return_matrix
    }
    
    fn forward_elimination(any_matrix: &Matrix) -> Matrix {
        let rows = any_matrix.len();
        let cols = any_matrix[0].len();
        let mut matrix = Vec::new();
        //return_matrix = any_matrix;
        for i in 0..rows {
            matrix.push(Vec::new());
    
            for j in 0..cols {
                matrix[i].push(any_matrix[i][j]);
            }
        }
    
        for x in 0..rows - 1 {
            let mut z = x;
            while z < rows - 1 {
                let numerator = matrix[z + 1][x];
                let denominator = matrix[x][x];
                let result = numerator / denominator;
    
                for i in 0..cols {
                    matrix[z + 1][i] = matrix[z + 1][i] - (result * matrix[x][i]);
                }
                z += 1;
            }
        }
        matrix
    }
    
    fn backward_substitution<'a>(
        any_matrix: &mut Matrix,
        arr: &'a mut Vec<f64>,
        row: i32,
        col: i32,
    ) -> &'a Vec<f64> {
        if row < 0 || col < 0 {
            return arr;
        } else {
            //println!("{row} {col}");
            let col = col as usize;
            let row = row as usize;
            let rows = any_matrix.len();
            let cols = any_matrix[0].len() - 1;
            let mut current = 0.0;
            let mut counter = 0;
            let mut i = (cols - 1) as i32;
            while i >= col as i32 {
                if i == col as i32 {
                    current = any_matrix[row][cols] / any_matrix[row][i as usize];
                } else {
                    any_matrix[row][cols] -= any_matrix[row][i as usize] * arr[rows - 1 - counter];
                    counter += 1;
                }
                //println!("{i}");
                i -= 1;
               
            }
    
            arr[row] = current;
            return backward_substitution(any_matrix, arr, row as i32 - 1, col as i32 - 1);
        }
    }
}

pub type Matrix = Vec<Vec<f64>>;
#[derive(Debug)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug)]
pub struct PolynomialRegression {
    pub data: Vec<DataPoint>,
    pub degree: usize,
    pub matrix: Matrix,
    pub left_matrix: Matrix,
    pub right_matrix: Vec<f64>,
}

impl PolynomialRegression {
    pub fn new(data_points: Vec<DataPoint>, degree: usize) -> Self {
        PolynomialRegression {
            data: data_points,
            degree,
            matrix: Vec::new(),
            left_matrix: Vec::new(),
            right_matrix: Vec::new(),
        }
    }

    /**
     * Sums up all x coordinates raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */

    fn sum_x(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.x.powi(power)).sum()
    }

    /**
     * sums up all x * y where x is raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sum_x_times_y(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.x.powi(power) * d.y).sum()
    }

    /**
     * Sums up all Y's raised to a power
     * @param anyData
     * @param power
     * @returns {number}
     */
    fn sum_y(&self, power: i32) -> f64 {
        self.data.iter().map(|d| d.y.powi(power)).sum()
    }

    /**
     * generate the left matrix
     */
    fn generate_left_matrix(&mut self) {
        for i in 0..=self.degree {
            self.left_matrix.push(Vec::new());
            for j in 0..=self.degree {
                if i == 0 && j == 0 {
                    self.left_matrix[i].push(self.data.len() as f64);
                } else {
                    let v = self.sum_x((i + j) as i32);
                    self.left_matrix[i].push(v);
                }
            }
        }
    }
    /**
     * generates the right hand matrix
     */
    fn generate_right_matrix(&mut self) {
        /*
        for (let i = 0; i <= this.degree; i++) {
            if (i === 0) {
                this.rightMatrix[i] = this.sum_y(this.data, 1);
            } else {
                this.rightMatrix[i] = this.sum_x_times_y(this.data, i);
            }
        }
        */
        for i in 0..=self.degree {
            if i == 0 {
                self.right_matrix.push(self.sum_y(1) as f64);
            } else {
                self.right_matrix.push(self.sum_x_times_y(i as i32) as f64);
            }
        }
    }
    /**
     * gets the terms for a polynomial
     * @returns {*}
     */
    pub fn get_terms(&mut self) -> Vec<f64> {
        self.generate_left_matrix();
        self.generate_right_matrix();
        matrix::gaussian_jordan_elimination(&self.left_matrix, &self.right_matrix)
    }
    /**
     * Predicts the Y value of a data set based on polynomial coefficients and the value of an independent variable
     * @param terms
     * @param x
     * @returns {number}
     */
    pub fn predict_y(terms: &Vec<f64>, x: f64) -> f64 {
        let mut result = 0.0;
        let mut i = (terms.len() - 1) as i32;
        while i >= 0 {
            if i == 0 {
                result += terms[i as usize];
            } else {
                result += terms[i as usize] * f64::powi(x, i);
            }
            i -= 1;
        }
        result
    }
}