use crate::util;

pub struct Network {
    loss_function: usize /* The current loss function is: MSE(0) */
}
impl Network {
    pub fn init() -> Self {
        Self {
            loss_function: 0
        }
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

        0.0
    }
}