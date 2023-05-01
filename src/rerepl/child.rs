use crate::rerepl::Rerepl;
use std::io;

impl Rerepl {
    pub(crate) fn run_as_child(&self)  -> Result<(), anyhow::Error>{
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim_start();
            if input.is_empty() {
                continue;
            }
            let cmd = input.split_whitespace().next();
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

        Ok(())
    }

    fn print_help(&self) {
        print!("Available commands:");
        for cmd in self.handlers.keys() {
            print!("\t{}", cmd);
        }
        println!();
    }
}
