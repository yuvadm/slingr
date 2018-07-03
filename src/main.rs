extern crate clap;
extern crate colored;
extern crate rustyline;

use colored::*;
use clap::{Arg, App, SubCommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let matches = App::new("Rustcast")
        .version("0.1")
        .author("Yuval Adam")
        .about("A simple UPnP/DLNA casting player")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("Media file to stream")
            .required(true))
        .get_matches();
    //println!("\n{}", "Rustcast v0.1.0".color("blue").bold());
    //println!("\n  usage: rustcast <media.file>\n");
}
