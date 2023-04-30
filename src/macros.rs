#[macro_export]
macro_rules! rerepl_init {
    ($body:block) => {
        if !Rerepl::is_parent() {
            $body
        }
    };
}

fn split_whitespace_preserve_empty(s: &str) -> impl Iterator<Item = &str> {
    s.split(|c: char| c.is_whitespace())
}

#[macro_export]
macro_rules! add_cmd {
    ($rerepl:ident, $cmd:expr, move || $body:block) => {
        $rerepl.add_handler($cmd, Box::new(move |s: &str| {
            $body
        }));
    };
    ($rerepl:ident, $cmd:expr, move |$($arg:ident: $arg_ty:ty),*, $string_arg:ident: String| $body:expr) => {
        $rerepl.add_handler($cmd, Box::new(move |s: &str| {
            let mut s = split_whitespace_preserve_empty(s);
            s.next(); // Discard the command name
            $(
                let $arg = s.next()
                    .and_then(|arg_str| arg_str.parse::<$arg_ty>().ok())
                    .unwrap();
            )*
            let $string_arg = s.collect::<Vec<_>>().join(" ");

            $body
        }));
    };

    ($rerepl:ident, $cmd:expr, move |$($arg:ident: $arg_ty:ty),*| $body:expr) => {
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
    };
}

pub use add_cmd;
pub use rerepl_init;
