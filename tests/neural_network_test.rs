use rust_neural_network::neural_network::NeuralNetwork;

#[test]
fn test_neural_network() {
    let mut neural_network = NeuralNetwork::new();
    neural_network.add_layer(3, 2);
    neural_network.add_layer(2, 3);

    let inputs = vec![0.5, 0.5];
    let targets = vec![0.5, 0.5];
    let learning_rate = 0.1;
    neural_network.train(&inputs, &targets, learning_rate);
}
