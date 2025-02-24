pub mod evaluate {

    trait Runable {
        fn run(&self);
    }

    struct Echo {
        message: String,
    }

    struct Exit {}

    struct Unknown {
        command: String,
    }

    impl Runable for Unknown {
        fn run(&self) {
            println!("{}: command not found", self.command);
        }
    }

    impl Runable for Exit {
        fn run(&self) {
            std::process::exit(0);
        }
    }

    impl Runable for Echo {
        fn run(&self) {
            println!("{}", self.message)
        }
    }

    fn parse_command(input: &Vec<&str>) -> Box<dyn Runable> {
        match input[0] {
            "echo" => {
                let message = String::from("");

                if input.len() == 2 {
                    let message = String::from(input[1]);
                    return Box::new(Echo { message });
                }
                if input.len() > 2 {
                    std::process::exit(1)
                }
                Box::new(Echo { message })
            }
            "exit" => Box::new(Exit {}),
            command @ _ => Box::new(Unknown {
                command: String::from(command),
            }),
        }
    }

    pub fn evaluate_input(input: &mut String) {
        input.pop();
        let input_list: Vec<&str> = input.split(' ').collect();
        let evaluated = parse_command(&input_list);
        evaluated.run();
    }
}
