use rust_neural_network::nn::{Neuron, Layer, Network};

#[test]
fn neuron_compute_output() {
    let neuron = Neuron::new(&[1.0, 0.5], -0.2);
    let inputs = [0.5, 0.25];
    let output = neuron.compute_output(&inputs);
    assert_eq!(output, 0.525);
}

#[test]
fn layer_compute_output() {
    let neurons = vec![
        Neuron::new(&[1.0, 0.5], -0.2),
        Neuron::new(&[-1.0, 0.5], 0.1),
    ];
    let layer = Layer::new(neurons);
    let inputs = [0.5, 0.25];
    let output = layer.compute_output(&inputs);
    assert_eq!(output, vec![0.525, -0.375]);
}

#[test]
fn network_compute_output() {
    let layers = vec![
        Layer::new(vec![
            Neuron::new(&[1.0, 0.5], -0.2),
        ]),
        Layer::new(vec![
            Neuron::new(&[-1.0, 0.5], 0.1),
        ]),
    ];
    let network = Network::new(layers);
    let inputs = [0.5, 0.25];
    let output = network.compute_output(&inputs);
    assert_eq!(output, vec![-0.375]);
}
