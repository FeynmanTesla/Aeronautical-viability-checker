use std::fs::{File, OpenOptions};

use randomforest::{RandomForestRegressor, RandomForestRegressorOptions};
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder};
use stl_io::IndexedMesh;

mod drag_coeff_estimator;
mod lift_coeff_estimator;

const PROPORTION_DATASET_TRAINING: f32 = 0.7;

pub fn process_stl_file(stl_file_path: &str) -> Vec<f64> {
    let mut file = OpenOptions::new().read(true).open(stl_file_path).unwrap();
    let stl: IndexedMesh = stl_io::read_stl(&mut file).unwrap();
    let mut vertex_coords: Vec<f64> = Vec::new();

    for vertex in stl.vertices {
        for coord in vertex.iter() {
            vertex_coords.push(*coord as f64);
        }
    }

    vertex_coords
}

pub fn estimate_drag_coefficient(processed_stl_file: &[f64]) -> f64 {
    estimate_coefficient(processed_stl_file, true)
}

pub fn estimate_lift_coefficient(processed_stl_file: &[f64]) -> f64 {
    estimate_coefficient(processed_stl_file, false)
}

fn get_model_file_path(drag_not_lift: bool) -> String {
    String::from(
        if drag_not_lift { drag_coeff_estimator::MODEL_FILE_PATH } else { lift_coeff_estimator::MODEL_FILE_PATH }
    )
}

fn get_random_forest_regressor(drag_not_lift: bool) -> RandomForestRegressor {
    match load_random_forest_regressor(drag_not_lift) {
        Some(regressor) => regressor,
        _ => make_random_forest_regressor(drag_not_lift)
    }
}

fn get_training_data(drag_not_lift: bool) -> TableBuilder {
    let mut table_builder: TableBuilder = TableBuilder::new();
    let num_dataset_values_training: i64 = get_num_dataset_values_training(drag_not_lift);

    let training_testing_inputs = get_training_testing_inputs(drag_not_lift);
    let training_testing_outputs = get_training_testing_outputs(drag_not_lift);
    assert_eq!(training_testing_inputs.len(), training_testing_outputs.len());
    let training_data: Vec<(&Vec<f64>, &f64)> = (&training_testing_inputs[(0 as usize)..(num_dataset_values_training as usize - 1 as usize)]).iter().zip((&training_testing_outputs[(0 as usize)..(num_dataset_values_training as usize - 1 as usize)]).iter()).collect();

    for (xs, y) in training_data {
        table_builder.add_row(xs, *y).unwrap();
    }
    table_builder
}

fn make_random_forest_regressor(drag_not_lift: bool) -> RandomForestRegressor {
    let table_builder: TableBuilder = get_training_data(drag_not_lift);
    let table: Table = table_builder.build().unwrap();

    let regressor: RandomForestRegressor = RandomForestRegressorOptions::new()
        .seed(0)
        .fit(Mse, table);

    test_random_forest_regressor(&regressor, drag_not_lift);
    save_random_forest_regressor(&regressor, drag_not_lift);

    regressor
}

fn get_training_testing_inputs(drag_not_lift: bool) -> Vec<Vec<f64>> {
    if drag_not_lift { drag_coeff_estimator::get_training_testing_inputs() } else { lift_coeff_estimator::get_training_testing_inputs() }
}

fn get_training_testing_outputs(drag_not_lift: bool) -> Vec<f64> {
    if drag_not_lift { drag_coeff_estimator::get_training_testing_outputs() } else { lift_coeff_estimator::get_training_testing_outputs() }
}

fn test_random_forest_regressor(regressor: &RandomForestRegressor, drag_not_lift: bool) {
    let mut sum_squared_diffs: f64 = 0.0;
    let mut number: i64 = 0;

    let num_dataset_values_training: i64 = get_num_dataset_values_training(drag_not_lift);
    let training_testing_inputs: Vec<Vec<f64>> = get_training_testing_inputs(drag_not_lift);
    let training_testing_outputs: Vec<f64> = get_training_testing_outputs(drag_not_lift);
    assert_eq!(training_testing_inputs.len(), training_testing_outputs.len());

    let start_index: usize = num_dataset_values_training as usize;
    let stop_index: usize = training_testing_inputs.len() as usize - 1 as usize;

    let testing_data: Vec<(&Vec<f64>, &f64)> = (&training_testing_inputs[start_index..stop_index]).iter().zip((&training_testing_outputs[start_index..stop_index]).iter()).collect();

    for (xs, y_actual) in testing_data {
        let y_predicted: f64 = regressor.predict(xs.as_slice());
        sum_squared_diffs += (y_predicted - y_actual).powi(2);
        number += 1;
    }

    let mse: f64 = sum_squared_diffs / (number as f64);
    println!("The mean squared error of the drag coefficient estimator over the testing data was {}.", mse);
}

fn load_random_forest_regressor(drag_not_lift: bool) -> Option<RandomForestRegressor> {
    RandomForestRegressor::deserialize(File::open(get_model_file_path(drag_not_lift)).unwrap()).ok()
}

fn save_random_forest_regressor(regressor: &RandomForestRegressor, drag_not_lift: bool) {
    let writer: File = File::create(get_model_file_path(drag_not_lift)).ok().unwrap();
    regressor.serialize(writer).unwrap();
}

pub fn estimate_coefficient(data: &[f64], drag_not_lift: bool) -> f64 {
    get_random_forest_regressor(drag_not_lift).predict(data)
}

fn get_num_dataset_values_training(drag_not_lift: bool) -> i64 {
    let len: usize = if drag_not_lift { drag_coeff_estimator::get_training_testing_inputs().len() } else { lift_coeff_estimator::get_training_testing_inputs().len() };
    (PROPORTION_DATASET_TRAINING * len as f32).round() as i64
}