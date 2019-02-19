#![feature(proc_macro_hygiene)]
use core::snake;

fn main() {
    let points = snake!();
    println!("the developer who compiled this got {} points", points);
}
