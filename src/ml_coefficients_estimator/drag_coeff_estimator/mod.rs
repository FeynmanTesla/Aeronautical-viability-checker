use std::fs::File;
use randomforest::{RandomForestRegressor, RandomForestRegressorOptions};
use randomforest::criterion::Mse;
use randomforest::table::{Table, TableBuilder};

const MODEL_FILE_PATH: &str = "data/drag_coefficient_model";

const TRAINING_INPUTS: [&[f64]; 12] = [
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

const TRAINING_OUTPUTS: [f64; 12] = [
    25.0, 30.0, 46.0, 45.0, 52.0, 23.0, 43.0, 35.0, 38.0, 46.0, 48.0, 52.0
];

//TODO: get real training and testing data and measure performance

fn get_random_forest_regressor() -> RandomForestRegressor {
    match load_random_forest_regressor() {
        Some(regressor) => regressor,
        None => make_random_forest_regressor()
    }
}

fn make_random_forest_regressor() -> RandomForestRegressor {
    //TODO: do this properly
    let mut table_builder: TableBuilder = TableBuilder::new();
    for (xs, y) in TRAINING_INPUTS.iter().zip(TRAINING_OUTPUTS.iter()) {
        table_builder.add_row(xs, *y)?;
    }
    let table: Table = table_builder.build()?;

    let regressor: RandomForestRegressor = RandomForestRegressorOptions::new()
        .seed(0)
        .fit(Mse, table);

    save_random_forest_regressor(&regressor);

    regressor
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