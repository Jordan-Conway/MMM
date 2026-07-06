use crate::data::Network;

mod data;

fn main() {
    let network = Network::new();
    println!("{:#?}", network);
}
