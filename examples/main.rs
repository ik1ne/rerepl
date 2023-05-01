use std::sync::{Arc, Mutex};

use rerepl::prelude::*;

#[tokio::main]
async fn main() {
    let mut rerepl = Rerepl::init("Command >".to_string());

    if Rerepl::is_parent() {
        // initialization for parent process does not exist

        // add commands for parent's help function
        // note: the handlers will not be called in parent process, and only the keys will be used for help function
        add_cmd!(rerepl, "add", move |a: i32| {});
        add_cmd!(rerepl, "hello", move |hello: String| {});
        add_cmd!(rerepl, "print", move || {});
    } else {
        // initialization for child process
        let acc = Arc::new(Mutex::from(0));

        let acc_clone = acc.clone();

        // add handlers for child process
        add_cmd!(rerepl, "add", move |a: i32| {
            let mut acc = acc_clone.lock().unwrap();
            *acc += a;
        });

        // FIXME: if the input was "hello world everybody", it would print "hello: world" instead of "hello: world everybody".
        add_cmd!(rerepl, "hello", move |hello: String| {
            println!("hello: {}", hello);
        });

        add_cmd!(rerepl, "print", move || {
            let acc_clone = acc.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    println!("sum: {}", acc_clone.lock().unwrap());
                }
            });
        });
    }

    rerepl.run();
}
