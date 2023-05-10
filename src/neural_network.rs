use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Array2<f64>>
}

impl NeuralNetwork {
    pub fn new(sizes: &[usize]) -> Self {
        let mut layers = Vec::new();
        for i in 0..sizes.len() - 1 {
            layers.push(Array2::zeros((sizes[i + 1], sizes[i])));
        }
        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let neural_network = NeuralNetwork::new(&[3, 2, 1]);
        assert_eq!(2, neural_network.layers.len());
    }
}