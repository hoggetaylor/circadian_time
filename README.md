# Circadian Time

This crate adds support for the [circadian
timezone](https://irreducible.software/essays/circadian-time) to chrono.

## Documentation

Documentation can be found on [docs.rs](https://docs.rs/circadian_time)

## Installation

``` shell
$ cargo add circadian_time --version 0.0.1
$ cargo add circadia --version 0.0.1
```

## Usage

``` rust
use circadia::GlobalPosition;
use circadian_time::{Circadian, Positioned};

#[derive(Debug, Clone)]
struct SandyUtah;

impl Positioned for SandyUtah {
    fn position() -> GlobalPosition {
        GlobalPosition::at(40.60710285372043, -111.85515699873065)
    }
}

fn main() {
    let now = Circadian::<SandyUtah>::now();
    println!("{}", now.format("%T"));
}
```
