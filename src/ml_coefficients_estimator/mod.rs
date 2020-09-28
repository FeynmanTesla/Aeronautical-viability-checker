use std::fs::{File, OpenOptions};

use randomforest::{RandomForestRegressor, RandomForestRegressorOptions};
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder};
use stl_io::IndexedMesh;

mod drag_coeff_estimator;
mod lift_coeff_estimator;

const PROPORTION_DATASET_TRAINING: f32 = 0.7;

pub fn process_stl_file(stl_file_path: &str) -> &[f64] {
    let mut file = OpenOptions::new().read(true).open(stl_file_path).unwrap();
    let stl: IndexedMesh = stl_io::read_stl(&mut file).unwrap();
    let mut vertex_coords: Vec<f64> = Vec::new();

    for triangle in stl.faces {
        for vertex in triangle.vertices.iter() {
            for coord in vertex {
                vertex_coords.push(coord as f64);
            }
        }
    }

    vertex_coords.as_slice()
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

fn get_training_data(drag_not_lift: bool) -> Table<'static> {
    let mut table_builder: TableBuilder = TableBuilder::new();
    let num_dataset_values_training: i64 = get_num_dataset_values_training(drag_not_lift);

    let training_testing_inputs = get_training_testing_inputs(drag_not_lift);
    let training_testing_outputs = get_training_testing_outputs(drag_not_lift);
    assert_eq!(training_testing_inputs.len(), training_testing_outputs.len());
    let training_data: Vec<(Vec<f64>, f64)> = training_testing_inputs[0..num_dataset_values_training - 1].iter().zip(training_testing_outputs[0..num_dataset_values_training - 1].iter);

    for (xs, y) in training_data {
        table_builder.add_row(&*xs, y);
    }
    table_builder.build().unwrap()
}

fn make_random_forest_regressor(drag_not_lift: bool) -> RandomForestRegressor {
    let table: Table = get_training_data(drag_not_lift);

    let regressor: RandomForestRegressor = RandomForestRegressorOptions::new()
        .seed(0)
        .fit(Mse, table);

    test_random_forest_regressor(&regressor, drag_not_lift);
    save_random_forest_regressor(&regressor, drag_not_lift);

    regressor
}

fn get_training_testing_inputs(drag_not_lift: bool) -> Vec<Vec<f64>> {
    let outer_vec: Vec<&[f64]> = if drag_not_lift { drag_coeff_estimator::TRAINING_TESTING_INPUTS } else { lift_coeff_estimator::TRAINING_TESTING_INPUTS }.into_vec();
    outer_vec.into_iter().map(|array| array.into_vec()).collect()
}

fn get_training_testing_outputs(drag_not_lift: bool) -> Vec<f64> {
    if drag_not_lift { drag_coeff_estimator::TRAINING_TESTING_OUTPUTS } else { lift_coeff_estimator::TRAINING_TESTING_OUTPUTS }.into_vec()
}

fn get_testing_data(drag_not_lift: bool) -> Vec<(Vec<f64>, f64)> {
    let num_dataset_values_training: i64 = get_num_dataset_values_training(drag_not_lift);
    let training_testing_inputs = get_training_testing_inputs(drag_not_lift);
    let training_testing_outputs = get_training_testing_outputs(drag_not_lift);
    assert_eq!(training_testing_inputs.len(), training_testing_outputs.len());
    training_testing_inputs[num_dataset_values_training..training_testing_inputs.len() - 1].iter().zip(training_testing_outputs[num_dataset_values_training..training_testing_outputs.len() - 1].iter)
}

fn test_random_forest_regressor(regressor: &RandomForestRegressor, drag_not_lift: bool) {
    let mut sum_squared_diffs: f64 = 0.0;
    let mut number: i64 = 0;
    for (xs, y_actual) in get_testing_data(drag_not_lift) {
        let y_predicted: f64 = regressor.predict(&*xs);
        sum_squared_diffs += (y_predicted - y_actual).powi(2);
        number += 1;
    }
    let mse: f64 = sum_squared_diffs / (number as f64);
    println!("The mean squared error of the drag coefficient estimator over the testing data was {}.", mse);
}

fn load_random_forest_regressor(drag_not_lift: bool) -> Option<RandomForestRegressor> {
    RandomForestRegressor::deserialize(File::open(get_model_file_path(drag_not_lift))).ok()
}

fn save_random_forest_regressor(regressor: &RandomForestRegressor, drag_not_lift: bool) {
    let writer: File = File::create(get_model_file_path(drag_not_lift))?;
    regressor.serialize(writer).unwrap();
}

pub fn estimate_coefficient(data: &[f64], drag_not_lift: bool) -> f64 {
    get_random_forest_regressor(drag_not_lift).predict(data)
}

fn get_num_dataset_values_training(drag_not_lift: bool) -> i64 {
    let len: usize = if drag_not_lift { drag_coeff_estimator::TRAINING_TESTING_INPUTS.len() } else { lift_coeff_estimator::TRAINING_TESTING_INPUTS.len() };
    (PROPORTION_DATASET_TRAINING * len as f32).round() as i64
}