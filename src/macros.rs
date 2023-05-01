#[macro_export]
macro_rules! add_cmd {
    ($rerepl:ident, $cmd:expr, move |$($arg:ident: $arg_ty:ty),*| $body:expr) => {
        if $crate::prelude::Rerepl::is_parent() {
            // add empty handler
            $rerepl.add_handler($cmd, Box::new(|_: &str| {}));
        } else {
            #[allow(unused_variables)]
            $rerepl.add_handler($cmd, Box::new(move |s: &str| {
                let mut s = s.split_ascii_whitespace();
                s.next(); // Discard the command name
                $(
                    let $arg = s.next()
                        .and_then(|arg_str| arg_str.parse::<$arg_ty>().ok())
                        .unwrap();
                )*

                $body
            }));
        }
    };
    ($rerepl:ident, $cmd:expr, move || $body:expr) => {
        if $crate::prelude::Rerepl::is_parent() {
            // add empty handler
            $rerepl.add_handler($cmd, Box::new(|_: &str| {}));
        } else {
            $rerepl.add_handler($cmd, Box::new(move |_: &str| {
                $body
            }));
        }
    };
}

pub use add_cmd;
