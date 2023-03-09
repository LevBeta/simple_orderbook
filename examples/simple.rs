use simple_orderbook;
use rand::Rng;

fn main(){

    //Create a new orderbook
    let mut ob = simple_orderbook::OrderBook::new();

    //Random number generator 
    let mut rng = rand::thread_rng();

    //Add some random bids to the orderbook
    for _ in 0..10 {
        let price: f64 = rng.gen_range(9.0..10.0);
        let size: f64 = rng.gen_range(100.0..200.0);
        ob.bid(price, size);
    }

    //Add some random asks to the orderbook
    for _ in 0..10 {
        let price: f64 = rng.gen_range(10.01..11.0);
        let size: f64 = rng.gen_range(100.0..200.0);
        ob.ask(price, size);
    }

    //Take a orderbook snapshot
    //Only top 5 levels each to bids and asks

    let snap = ob.snapshot();
    println!("Snapshot: {:?}", snap);
}