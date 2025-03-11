mod util;

fn main() {
    /*let test_list = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let sigmoid_function = util::sigmoid(test_list);
    for item in sigmoid_function {
        println!("{}", item);
    }

    let test_list = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let derivative_sigmoid_function = util::derivative_sigmoid(test_list);
    for item in derivative_sigmoid_function {
        println!("{}", item);
    }*/

    /*let test_list_1 = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let test_list_2 = vec![0.1, 0.3, 0.5, 0.7, 0.9];
    let dot_product = util::dot(test_list_1, test_list_2);
    println!("{}", dot_product);*/

    let test_list = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let sigmoid_function = util::tanh(test_list);
    for item in sigmoid_function {
        println!("{}", item);
    }
}
