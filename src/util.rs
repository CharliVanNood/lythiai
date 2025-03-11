pub const EULER: f32 = 2.71828182845904523536028747135266250;

pub fn sigmoid(mut list_in: Vec<f32>) -> Vec<f32> {
    for value_index in 0..list_in.len() {
        list_in[value_index] = 1.0 / (1.0 + exp(-list_in[value_index]))
    }
    list_in
}

pub fn tanh(mut list_in: Vec<f32>) -> Vec<f32> {
    for value_index in 0..list_in.len() {
        let exp_positive = exp(list_in[value_index]);
        let exp_negative = exp(-list_in[value_index]);
        list_in[value_index] = (exp_positive - exp_negative) / (exp_positive + exp_negative)
    }
    list_in
}

pub fn derivative_sigmoid(mut list_in: Vec<f32>) -> Vec<f32> {
    for value_index in 0..list_in.len() {
        list_in[value_index] = list_in[value_index] * (1.0 - list_in[value_index])
    }
    list_in
}

pub fn derivative_tanh(mut list_in: Vec<f32>) -> Vec<f32> {
    for value_index in 0..list_in.len() {
        list_in[value_index] = 1.0 - list_in[value_index] * list_in[value_index]
    }
    list_in
}

pub fn exp(value: f32) -> f32 {
    EULER.powf(value)
}

pub fn dot(list1: Vec<f32>, list2: Vec<f32>) -> f32 {
    let mut result = 0.0;

    if list1.len() != list2.len() {
        println!("The list sizes aren't equal");
        return 0.0;
    }

    for i in 0..list1.len() {
        result += list1[i] * list2[i];
    }

    result
}

pub fn average(list_in: Vec<f32>) -> f32 {
    let list_len = list_in.len() as f32;
    let mut result = 0.0;
    for value in list_in {
        result += value;
    }
    result / list_len
}

/*
Why didn't I use the average function?
Here it would be slower to create a new list of the combined lists, or updating would also be slower
*/
pub fn mse(list1: Vec<f32>, list2: Vec<f32>) -> f32 {
    if list1.len() != list2.len() {
        println!("The list sizes aren't equal");
        return 0.0;
    }

    let list_len = list1.len() as f32;

    let mut result = 0.0;

    for value_index in 0..list_len as usize {
        result += (list1[value_index] - list2[value_index]).powf(2.0);
    }
    result / list_len
}