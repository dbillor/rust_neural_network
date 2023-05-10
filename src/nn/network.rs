use crate::nn::layer::Layer;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn compute_output(&self, inputs: &[f64]) -> Vec<f64> {
        self.layers.iter().fold(inputs.to_vec(), |input_values, layer| {
            layer.compute_output(&input_values)
        })
    }
}
