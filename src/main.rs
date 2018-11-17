#![feature(slice_patterns)]

use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!(
            "{}> ",
            env::current_dir()
                .expect("Unable to find current directory")
                .into_os_string()
                .to_str()
                .expect("Unable to convert OS String into string")
        );
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Unable to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let mut child = match parts.as_slice() {
            ["cd", args..] => {
                let new_dir = args.into_iter().peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
                continue;
            }
            ["exit"] => return,
            [command, args..] => Command::new(command)
                .args(args)
                .spawn()
                .expect("Unable to run given command"),
            _ => continue,
        };
        child.wait();
    }
}
