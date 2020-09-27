use std::borrow::Borrow;
use std::io;

use rusty_machine::linalg::Matrix;

#[path = "../viable/mod.rs"]
mod viable;

#[path = "../viable/thrust/mod.rs"]
mod thrust;

#[path = "../ml_coefficients_estimator/mod.rs"]
mod ml_coefficients_estimator;

pub fn run_cli() {
    let mass_kilograms: f64 = get_f64_user_input("What is the mass of the aircraft (kg)?");
    let max_altitude_metres: i64 = get_i64_user_input("What is the maximum altitude of the aircraft (metres)?");
    let min_velocity_metres_per_second: i64 = get_i64_user_input("What is the minimum velocity of the aircraft (metres/second)?");
    let max_velocity_metres_per_second: i64 = get_i64_user_input("What is the maximum velocity of the aircraft (metres/second)?");
    let frontal_projection_area: f64 = get_f64_user_input("What is the frontal projection area of the aircraft (metres squared)?");
    let wing_area: f64 = get_f64_user_input("What is the wing area of the aircraft (metres squared)?");
    let stl_model_file_path: String = get_string_user_input("What is the filepath of the STL file modelling the aircraft?");

    let processed_stl_file: Matrix<f64> = ml_coefficients_estimator::process_stl_file(stl_model_file_path.borrow());
    let lift_coefficient: f64 = ml_coefficients_estimator::get_lift_coefficient(&processed_stl_file);
    let drag_coefficient: f64 = ml_coefficients_estimator::get_drag_coefficient(&processed_stl_file);

    let altitudes_thrusts_csv_file_path: String = get_string_user_input("What is the path to the altitudes-thrusts CSV file?");
    let altitudes_thrusts: &Vec<(i64, f64)> = &thrust::get_altitudes_thrusts(altitudes_thrusts_csv_file_path.borrow());

    println!("{}", viable::is_viable(mass_kilograms, max_altitude_metres, min_velocity_metres_per_second, max_velocity_metres_per_second, frontal_projection_area, wing_area, lift_coefficient, drag_coefficient, altitudes_thrusts));
}

fn get_string_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn get_f64_user_input(prompt: &str) -> f64 {
    get_string_user_input(prompt).trim().parse::<f64>().unwrap()
}

fn get_i64_user_input(prompt: &str) -> i64 {
    get_string_user_input(prompt).trim().parse::<i64>().unwrap()
}