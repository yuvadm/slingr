extern crate clap;
extern crate colored;
extern crate rustyline;

use colored::*;
use clap::{Arg, App};
// use rustyline::error::ReadlineError;
// use rustyline::Editor;
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("Rustcast")
        .version("0.1")
        .author("Yuval Adam")
        .about("A simple UPnP/DLNA casting player")
        .arg(Arg::with_name("FILE")
            .value_name("FILE")
            .help("Media file to stream")
            .index(1)
            .required(true))
        .get_matches();

    let infile = matches.value_of("FILE").unwrap();

    if Path::new(infile).exists() {
        println!("\n{} {}\n", "Using input file:".green(), infile.green());
    }
    else {
        println!("\n{} {}\n", "Input file does not exist:".red(), infile.red());
        process::exit(1);
    }
}
