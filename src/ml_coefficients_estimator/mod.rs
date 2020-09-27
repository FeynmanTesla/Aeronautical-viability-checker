use std::fs::OpenOptions;
use stl_io::{IndexedMesh, IndexedTriangle, Vertex};

mod drag_coeff_estimator;
mod lift_coeff_estimator;

pub fn process_stl_file(stl_file_path: &str) -> &[f64] {
    let mut file = OpenOptions::new().read(true).open(stl_file_path).unwrap();
    let stl: IndexedMesh = stl_io::read_stl(&mut file).unwrap();
    let mut vertex_coords: Vec<f64> = Vec::new();

    for triangle in stl.faces {
        for vertex in triangle {
            for coord in vertex {
                vertex_coords.push(coord as f64);
            }
        }
    }

    vertex_coords.as_slice()
}

pub fn get_drag_coefficient(processed_stl_file: &[f64]) -> f64 {
    drag_coeff_estimator::predict_drag_coefficient(processed_stl_file)
}

pub fn get_lift_coefficient(processed_stl_file: &[f64]) -> f64 {
    lift_coeff_estimator::predict_lift_coefficient(processed_stl_file)
}