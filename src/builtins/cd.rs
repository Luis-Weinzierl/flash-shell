use std::path::Path;
use std::env::{set_current_dir};

pub fn main(command: crate::parser::Command) {
    if command.args.len() != 1 {
        println!("\x1b[31mExpected 1 argument but received {}\x1b[0m",
            command.args.len()
        );
        return;
    }

    let path = Path::new(&command.args[0]);

    if !path.exists() {
        println!("\x1b[31mDirectory \"{}\" does not exist.\x1b[0m",
            command.args[0].clone()
        );
        return;
    }

    set_current_dir(&path).expect(&format!("Unable to cd into {}", path.display()));
}