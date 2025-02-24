#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    let r = stdin.read_line(&mut input);
    match r {
        Ok(val) => print!("{val}: command not found"),
        Err(err) => {
            print!("An error ocurred while reading input: {err}");
        }
    }
}
