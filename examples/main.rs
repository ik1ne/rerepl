use std::sync::{Arc, Mutex};

use rerepl::prelude::*;

#[tokio::main]
async fn main() {
    let mut rerepl = Rerepl::init("Command >".to_string());

    rerepl_init!({
        let acc = Arc::new(Mutex::from(0));

        let acc_clone = acc.clone();

        add_cmd!(rerepl, "add", move |a: i32| {
            let mut acc = acc_clone.lock().unwrap();
            *acc += a;
        });

        add_cmd!(rerepl, "hello", move |_a: i32, hello: String| {
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
    });

    rerepl.run();
}
