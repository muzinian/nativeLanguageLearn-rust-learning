use std::thread;
use std::time::{Duration, Instant};
fn main() {
    let start = Instant::now();

    thread::sleep(Duration::from_millis(100));
    let first = start.elapsed();
    thread::sleep(Duration::from_millis(200));
    let total = start.elapsed();

    println!("first {} ms", first.as_millis());
    println!("second {} ms", total.as_millis());
}
