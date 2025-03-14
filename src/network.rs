use crate::{relu, util};

pub enum Layer {
    RELU(relu::RELU)
}

pub struct Network {
    loss_function: usize, /* The current loss function is: MSE(0) */
    layers: Vec<Layer> /* A list of layer indexes, 0: RELU */
}
impl Network {
    pub fn init() -> Self {
        Self {
            loss_function: 0,
            layers: Vec::new()
        }
    }

    pub fn add(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    fn calc_loss(&self, prediction: Vec<f32>, reference: Vec<f32>) -> f32 {
        match self.loss_function {
            0 => return util::mse(prediction, reference),
            _ => return util::mse(prediction, reference)
        };
    }

    fn validate(&self, prediction: Vec<f32>, reference: Vec<f32>) -> f32 {
        let results = Vec::new();

        if prediction.len() != reference.len() {
            println!("[VALIDATION] The list sizes aren't equal");
            return 0.0;
        }

        for value_index in 0..prediction.len() {
            
        }

        util::average(results)
    }
}