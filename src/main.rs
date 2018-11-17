#![feature(slice_patterns)]
extern crate simple_error;
use simple_error::SimpleError;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::{error::Error, result::Result};

fn main() {
    loop {
        if let Err(e) = run() {
            // keep running in the case of an error
            eprintln!("{}", e)
        } else {
            return;
        }
    }
}

fn run() -> Result<(), Box<Error>> {
    loop {
        print!(
            "{}> ",
            env::current_dir()?
                .into_os_string()
                .to_str()
                .ok_or(SimpleError::new("Unable to decode OS String"))?
        );
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["cd", args..] => {
                let new_dir = args.into_iter().peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
                continue;
            }
            ["exit"] => return Ok(()),
            [command, args..] => Command::new(command).args(args).spawn()?.wait(),
            _ => continue,
        }?;
    }
}
