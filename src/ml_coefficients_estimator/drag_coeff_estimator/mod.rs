pub(crate) const MODEL_FILE_PATH: &str = "data/models/drag_coefficient_model";

const TRAINING_TESTING_INPUTS: [[f64; 4]; 12] = [
    [0.0, 2.0, 1.0, 0.0],
    [0.0, 2.0, 1.0, 1.0],
    [1.0, 2.0, 1.0, 0.0],
    [2.0, 1.0, 1.0, 0.0],
    [2.0, 0.0, 0.0, 0.0],
    [2.0, 0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [2.0, 1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 1.0],
];

const TRAINING_TESTING_OUTPUTS: [f64; 12] = [
    25.0, 30.0, 46.0, 45.0, 52.0, 23.0, 43.0, 35.0, 38.0, 46.0, 48.0, 52.0
];

pub(crate) fn get_training_testing_inputs() -> Vec<Vec<f64>> {
    Vec::from(TRAINING_TESTING_INPUTS).into_iter().map(|array: [f64; 4]| Vec::from(array)).collect()
}

pub(crate) fn get_training_testing_outputs() -> Vec<f64> {
    Vec::from(TRAINING_TESTING_OUTPUTS)
}

//TODO: get real training and testing data and measure performance