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
