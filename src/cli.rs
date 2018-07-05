extern crate termios;

use std::io;
use std::io::{Read, Write};
use self::termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

pub fn read_char() {
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
