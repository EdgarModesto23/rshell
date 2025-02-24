#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    // match r {
    //     Ok(val) => print!(""),
    //     Err(err) => {
    //         print!("An error ocurred while reading input");
    //     }
    // }
}
