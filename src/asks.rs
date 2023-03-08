pub struct Asks {
    pub ticks: Vec<(f64, f64)>,
}

impl Asks {
    pub fn new() -> Asks {
        Asks{ 
            ticks: Vec::new(),
        }
    }

    pub fn find_tick(&self, price: f64) -> Option<&(f64, f64)> {
        self.ticks.iter().find(|tick| tick.0 == price)
    }

    pub fn set_asks(&mut self, price: f64, qty: f64){
        if self.ticks.is_empty() {
            self.ticks.push((price, qty));
            return
        }

        match self.ticks.binary_search_by(|&(p, _)| p.partial_cmp(&price).unwrap()) {
            Ok(value) => {
                if qty == 0.0 {
                    self.ticks.remove(value);
                } else if self.ticks[value].0 == price {
                    self.ticks[value] = (price, qty);
                } else {
                    self.ticks.insert(value, (price, qty));
                }
            }
            Err(value) => {
                if qty != 0.0 {
                    self.ticks.insert(value, (price, qty));
                }
            }
        }
    }

    pub fn snapshot_5(&self) -> Vec<&(f64, f64)> {
        self.ticks.iter().take(5).collect()
    }
}