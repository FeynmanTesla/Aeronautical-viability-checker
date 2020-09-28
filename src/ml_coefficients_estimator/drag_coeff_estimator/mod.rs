use std::fs::File;
use randomforest::{RandomForestRegressor, RandomForestRegressorOptions};
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder};

const MODEL_FILE_PATH: &str = "data/drag_coefficient_model";

const TRAINING_TESTING_INPUTS: [&[f64]; 12] = [
    &[0.0, 2.0, 1.0, 0.0][..],
    &[0.0, 2.0, 1.0, 1.0][..],
    &[1.0, 2.0, 1.0, 0.0][..],
    &[2.0, 1.0, 1.0, 0.0][..],
    &[2.0, 0.0, 0.0, 0.0][..],
    &[2.0, 0.0, 0.0, 1.0][..],
    &[1.0, 0.0, 0.0, 1.0][..],
    &[0.0, 1.0, 1.0, 0.0][..],
    &[0.0, 0.0, 0.0, 0.0][..],
    &[2.0, 1.0, 0.0, 0.0][..],
    &[0.0, 1.0, 0.0, 1.0][..],
    &[1.0, 1.0, 1.0, 1.0][..],
];

const TRAINING_TESTING_OUTPUTS: [f64; 12] = [
    25.0, 30.0, 46.0, 45.0, 52.0, 23.0, 43.0, 35.0, 38.0, 46.0, 48.0, 52.0
];

assert_eq!(TRAINING_TESTING_INPUTS.len(), TRAINING_TESTING_OUTPUTS.len());

const PROPORTION_DATASET_TRAINING: f32 = 0.7;
const NUM_DATASET_VALUES_TRAINING: i64 = Math::round(PROPORTION_DATASET_TRAINING * TRAINING_TESTING_INPUTS.len());
const NUM_DATASET_VALUES_TESTING: i64 = (TRAINING_TESTING_INPUTS.len() - NUM_DATASET_VALUES_TRAINING) as i64;

const TRAINING_INPUTS: [&[f64]; NUM_DATASET_VALUES_TRAINING as usize] = TRAINING_TESTING_INPUTS[0..NUM_DATASET_VALUES_TRAINING - 1];
const TESTING_INPUTS: [&[f64]; NUM_DATASET_VALUES_TESTING as usize] = TRAINING_TESTING_INPUTS[NUM_DATASET_VALUES_TRAINING..TRAINING_TESTING_INPUTS.len()];

const TRAINING_OUTPUTS: [&[f64]; NUM_DATASET_VALUES_TRAINING as usize] = TRAINING_TESTING_OUTPUTS[0..NUM_DATASET_VALUES_TRAINING - 1];
const TESTING_OUTPUTS: [&[f64]; NUM_DATASET_VALUES_TESTING as usize] = TRAINING_TESTING_OUTPUTS[NUM_DATASET_VALUES_TRAINING..TRAINING_TESTING_INPUTS.len()];

//TODO: get real training and testing data and measure performance

fn get_random_forest_regressor() -> RandomForestRegressor {
    match load_random_forest_regressor() {
        Some(regressor) => regressor,
        None => make_random_forest_regressor()
    }
}

fn make_random_forest_regressor() -> RandomForestRegressor {
    let mut table_builder: TableBuilder = TableBuilder::new();
    for (xs, y) in TRAINING_INPUTS.iter().zip(TRAINING_OUTPUTS.iter()) {
        table_builder.add_row(xs, *y)?;
    }
    let table: Table = table_builder.build()?;

    let regressor: RandomForestRegressor = RandomForestRegressorOptions::new()
        .seed(0)
        .fit(Mse, table);

    test_random_forest_regressor(&regressor);
    save_random_forest_regressor(&regressor);

    regressor
}

fn test_random_forest_regressor(regressor: &RandomForestRegressor) {
    let mut sum_squared_diffs: f64 = 0.0;
    for (xs, y_actual) in TESTING_INPUTS.iter().zip(TESTING_OUTPUTS.iter()) {
        let y_predicted: f64 = regressor.predict(xs);
        sum_squared_diffs += (y_predicted - y_actual).powi(2);
    }
    let mse: f64 = sum_squared_diffs / TESTING_OUTPUTS.len();
    println!("The mean squared error of the drag coefficient estimator over the testing data was {}.", mse);
}

fn load_random_forest_regressor() -> Option<RandomForestRegressor> {
    RandomForest::deserialize(File::open(MODEL_FILE_PATH))
}

fn save_random_forest_regressor(regressor: &RandomForestRegressor) {
    let writer: File = File::create(MODEL_FILE_PATH)?;
    regressor.serialize(writer).unwrap();
}

pub fn predict_drag_coefficient(data: &[f64]) -> f64 {
    get_random_forest_regressor().predict(data)
}