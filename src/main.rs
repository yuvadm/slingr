extern crate clap;
extern crate colored;
extern crate termios;

use colored::*;
use clap::{Arg, App};
use std::path::Path;
use std::process;

use std::io;
use std::io::{Read, Write};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn read_char() {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO);  // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    print!("Hit a key! ");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    println!("You have hit: {:?}", buffer);
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to original termios data
}

fn main() {
    let app = App::new("Rustcast")
        .version("0.1")
        .author("Yuval Adam")
        .about("A simple UPnP/DLNA casting player")
        .arg(Arg::with_name("FILE")
            .value_name("FILE")
            .help("Media file to stream")
            .index(1)
            .required(true));

    let matches = app.get_matches();
    let infile = matches.value_of("FILE").unwrap();

    if Path::new(infile).exists() {
        println!("\n{} {}\n", "Using input file:".green(), infile.green());
    }
    else {
        println!("\n{} {}\n", "Input file does not exist:".red(), infile.red());
        process::exit(1);
    }

    read_char();
}
