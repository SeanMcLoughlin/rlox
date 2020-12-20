extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate thiserror;

mod cmdline_args;
mod error;
mod lexer;
mod scanner;
mod utils;

use crate::scanner::Scanner;
use cmdline_args::get_script_name;
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let lox = Lox::new();

    let script_name = match get_script_name() {
        Ok(m) => m,
        Err(e) => {
            print!("{}", e);
            return Ok(());
        }
    };

    match script_name {
        Some(file) => lox.run_file(file)?,
        None => lox.run_prompt()?,
    }

    Ok(())
}

struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(&self, file: String) -> Result<(), Box<dyn Error>> {
        self.run(read_to_string(file)?)
    }

    pub fn run_prompt(&self) -> Result<(), Box<dyn Error>> {
        loop {
            print!("> ");

            io::Write::flush(&mut io::stdout())?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            if input == "exit()\n" {
                break;
            }

            match self.run(input) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<(), Box<dyn Error>> {
        for token in Scanner::new(source).scan_tokens()? {
            println!("{:?}", token);
        }
        Ok(())
    }
}
