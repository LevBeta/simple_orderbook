# Simple orderbook

Simple orderbook made in Rust

(Very bad code, you can use it as a base for your own, or contribute)

## Usage

```rust

//Create a new orderbook
let mut ob = OrderBook::new();

//Add a bid at 1.0 with 2.5 of size and a ask at 1.5 with 2.2 of size
ob.bid(1.0, 2.5);
ob.ask(1.5, 2.2);

//Gets a orderbook snapshot
let snap = ob.snapshot();

//Update the bid at 1.0 to 2.1 of size
ob.bid(1.0, 2.1);

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
