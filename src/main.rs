extern crate clap;
extern crate colored;

use colored::*;
use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::thread;
use std::sync::mpsc;

mod cli;
mod upnp;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Yuval Adam")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("device")
             .short("d")
             .long("device")
             .value_name("DEVICE")
             .help("Target specific device address"))
        .arg(Arg::with_name("subtitles")
             .short("s")
             .long("subtitles")
             .value_name("SUBTITLES")
             .help("Subtitles file to use with the video"))
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

    let _udp = thread::spawn(move || {
        upnp::discover();
    });

    let (tx, rx) = mpsc::channel();
    let child = thread::spawn(move || {
        let mut controller = cli::Controller::init();
        loop {
            let c = controller.read();
            tx.send(c).unwrap();
            if c == 113 {
                break;
            }
        }
        controller.destroy();
    });

    for received in rx {
        println!("Got char: {}", received);
    }

    println!("Waiting for all thread");
    let _res = child.join();
    println!("Done!");

}
