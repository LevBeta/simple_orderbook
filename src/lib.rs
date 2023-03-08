mod bids;
mod asks;

use bids::Bids;
use asks::Asks;

pub struct OrderBook {
    pub bids: Bids,
    pub asks: Asks,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook{ 
            bids: Bids::new(),
            asks: Asks::new()
        }
    }

    pub fn bid(&mut self, price: f64, qty: f64) {
        self.bids.set_bids(price, qty);
    }

    pub fn ask(&mut self, price: f64, qty: f64) {
        self.asks.set_asks(price, qty);
    }

    pub fn get_bid(&self, price: f64) -> Option<&(f64, f64)> {
        self.bids.find_tick(price)
    }

    pub fn get_ask(&self, price: f64) -> Option<&(f64, f64)> {
        self.asks.find_tick(price)
    }
    pub fn snapshot(&self) -> (Vec<&(f64, f64)>, Vec<&(f64, f64)>) {
        let bids_snapshot = self.bids.snapshot_5();
        let asks_snapshot = self.asks.snapshot_5();
        
        (bids_snapshot, asks_snapshot)
    }
}