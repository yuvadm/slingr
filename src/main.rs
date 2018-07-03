extern crate colored;

use colored::*;

fn main() {
    println!("\n{}", "Rustcast v0.1.0".color("blue").bold());
    println!("\n  usage: rustcast <media.file>\n");
}
