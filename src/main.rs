#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.pop();
        let input_list: Vec<&str> = input.split(' ').collect();
        if input_list[0] == "exit" {
            std::process::exit(0);
        }

        println!("{}: command not found", input);
    }
}
