use std::fs::OpenOptions;

use stl_io::IndexedMesh;

use rusty_machine::linalg::Matrix;

mod drag_coeff_estimator;
mod lift_coeff_estimator;

pub fn process_stl_file(stl_file_path: &str) -> Matrix<f64> {
    let mut file = OpenOptions::new().read(true).open(stl_file_path).unwrap();
    let stl: IndexedMesh = stl_io::read_stl(&mut file).unwrap();
    let rows: usize = stl.faces.len(); // num of triangles
    let cols: usize = 9; // 3x vertices per triangle, 3x coords per vertex
    Matrix::new(rows, cols, stl)
}

pub fn get_drag_coefficient(processed_stl_file: &Matrix<f64>) -> f64 {
    drag_coeff_estimator::predict_drag_coefficient(processed_stl_file)
}

pub fn get_lift_coefficient(processed_stl_file: &Matrix<f64>) -> f64 {
    lift_coeff_estimator::predict_lift_coefficient(processed_stl_file)
}