pub(crate) const MODEL_FILE_PATH: &str = "data/drag_coefficient_model";

pub(crate) const TRAINING_TESTING_INPUTS: Vec<Vec<f64>> = vec![
    vec![0.0, 2.0, 1.0, 0.0],
    vec![0.0, 2.0, 1.0, 1.0],
    vec![1.0, 2.0, 1.0, 0.0],
    vec![2.0, 1.0, 1.0, 0.0],
    vec![2.0, 0.0, 0.0, 0.0],
    vec![2.0, 0.0, 0.0, 1.0],
    vec![1.0, 0.0, 0.0, 1.0],
    vec![0.0, 1.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 0.0],
    vec![2.0, 1.0, 0.0, 0.0],
    vec![0.0, 1.0, 0.0, 1.0],
    vec![1.0, 1.0, 1.0, 1.0],
];

pub(crate) const TRAINING_TESTING_OUTPUTS: Vec<f64> = vec![
    25.0, 30.0, 46.0, 45.0, 52.0, 23.0, 43.0, 35.0, 38.0, 46.0, 48.0, 52.0
];

//TODO: get real training and testing data and measure performance