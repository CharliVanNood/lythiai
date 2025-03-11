mod util;

fn main() {
    let test_list = vec![0.1, 0.3, 0.5, 1.0, 0.9, -0.1, -1.0, 10.0, 1000.0];
    let sigmoid_function = util::sigmoid(test_list);
    for item in sigmoid_function {
        println!("{}", item);
    }
}
