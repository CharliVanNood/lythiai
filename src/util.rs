pub const EULER: f32 = 2.71828182845904523536028747135266250;

pub fn sigmoid(mut list_in: Vec<f32>) -> Vec<f32> {
    for value_index in 0..list_in.len() {
        list_in[value_index] = 1.0 / (1.0 + exp(-list_in[value_index]))
    }
    list_in
}

pub fn exp(value: f32) -> f32 {
    EULER.powf(value)
}