mod util;
mod network;
mod relu;

fn main() {
    let mut network = network::Network::init();
    let relu_layer = relu::RELU::new(3);
    network.add(network::Layer::RELU(relu_layer));
    network.info();
    network.train(vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]], vec![0.0, 1.0, 1.0, 0.0]);
}
