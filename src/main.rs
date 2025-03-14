mod util;
mod network;
mod relu;

fn main() {
    let mut network = network::Network::init();
    let relu_layer = relu::RELU::new();
    network.add(network::Layer::RELU(relu_layer));
}
