use crate::rerepl::Rerepl;
use std::io;

impl Rerepl {
    pub(crate) fn run_as_child(&self) {
        let mut input = String::new();
        loop {
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim_start();
            if input.is_empty() {
                continue;
            }
            let cmd = input.splitn(2, ' ').take(1).next();
            match cmd {
                None => {
                    continue;
                }
                Some("exit") => {
                    break;
                }
                Some("help") => {
                    self.print_help();
                    continue;
                }
                Some(cmd) => {
                    if let Some(handler) = self.handlers.get(cmd) {
                        handler(input);
                    } else {
                        println!("Unknown command: {}", cmd);
                    }
                }
            }
        }
    }

    fn print_help(&self) {
        println!("Available commands:");
        for cmd in self.handlers.keys() {
            println!("\t{}", cmd);
        }
    }
}
