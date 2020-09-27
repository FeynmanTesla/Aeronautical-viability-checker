// TODO: make a machine learning module here that predicts the lift coefficient from a 3D model given by the user (STL (with a .STL extension), COLLADA (with a .DAE extension), etc)

use rusty_machine::learning::nnet::{BCECriterion, NeuralNet};
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::learning::SupModel;
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::linalg::Matrix;

// TODO: remake with tensorflow over rusty_machine

pub fn predict_lift_coefficient(data: &Matrix<f64>) -> f64 {
    let neural_net: NeuralNet<BCECriterion, StochasticGD> = get_neural_network();
    let matrix_result: Matrix<f64> = neural_net.predict(data).unwrap();
    assert_eq!(matrix_result.data().len(), 1);
    matrix_result.data()[0]
}

fn get_lift_predictor_neural_network() -> NeuralNet<BCECriterion, StochasticGD> {
    match load_lift_predictor_neural_network() {
        Some(net) => net,
        _ => make_lift_predictor_neural_network()
    }
}

fn load_lift_predictor_neural_network() -> Option<NeuralNet<BCECriterion, StochasticGD>> {
    //TODO: implement this: saving or training and returning a neural net
}

fn save_lift_predictor_neural_network(model: &NeuralNet<BCECriterion, StochasticGD>) {
    //TODO: implement this: saving or training and returning a neural net
}

fn make_lift_predictor_neural_network() -> NeuralNet<BCECriterion, StochasticGD> {
    const LAYERS: &[usize] = &[128, 64, 32, 16, 8, 4, 2, 1];
    const CRITERION: BCECriterion = BCECriterion::new(Regularization::L2(0.1));
    let mut model: NeuralNet<BCECriterion, StochasticGD> = NeuralNet::new(LAYERS, CRITERION, StochasticGD::default());
    let training_inputs: Matrix<f64> = get_training_inputs();
    let training_outputs: Matrix<f64> = get_training_outputs();
    model.train(&training_inputs, &training_outputs).unwrap();
    let testing_inputs: Matrix<f64> = get_testing_inputs();
    let testing_outputs: Matrix<f64> = get_testing_outputs();
    let testing_predictions: Matrix<f64> = model.predict(&testing_inputs).unwrap();
    save_lift_predictor_neural_network(&model);
    model
}

fn get_training_inputs() -> Matrix<f64> {
    //TODO: implement this
}

fn get_training_outputs() -> Matrix<f64> {
    //TODO: implement this
}

fn get_testing_inputs() -> Matrix<f64> {
    //TODO: implement this
}

fn get_testing_outputs() -> Matrix<f64> {
    //TODO: implement this
}