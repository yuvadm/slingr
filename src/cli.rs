extern crate termios;

use std::io;
use std::io::{Stdout, Stdin, Read, Write};
use self::termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

pub struct Controller {
    termios: Termios,
    stdin: i32,
    stdout: Stdout,
    reader: Stdin,
    buffer: [u8; 1]
}

impl Controller {
    pub fn init() -> Controller {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();

        // make a mutable copy of termios, and drop echo and canonical mode
        let mut new_termios = termios;
        new_termios.c_lflag &= !(ICANON | ECHO);

        tcsetattr(stdin, TCSANOW, &new_termios).unwrap();
        let stdout = io::stdout();
        let reader = io::stdin();
        let buffer = [0; 1];
        Controller {
            termios,
            stdin,
            stdout,
            reader,
            buffer
        }
    }

    pub fn read(&mut self) -> u8 {
        print!("Hit a key! ");
        self.stdout.lock().flush().unwrap();
        self.reader.read_exact(&mut self.buffer).unwrap();
        println!("You have hit: {:?}", self.buffer);
        self.buffer[0]
    }

    pub fn destroy(&self) {
        tcsetattr(self.stdin, TCSANOW, & self.termios).unwrap();  // reset the stdin to original termios data
    }
}
