pub struct Neuron {
    weights: Vec<f64>,
    bias: f64,
}

impl Neuron {
    pub fn new(weights: &[f64], bias: f64) -> Self {
        Self {
            weights: weights.to_vec(),
            bias,
        }
    }

    pub fn compute_output(&self, inputs: &[f64]) -> f64 {
        let sum = self.weights.iter().zip(inputs).map(|(w, i)| w * i).sum::<f64>();
        sum + self.bias
    }
}
