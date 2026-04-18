use std::io::{self, Write};

pub fn info(msg: &str) {
    let _ = writeln!(io::stderr(), "info: {msg}");
}

pub fn warn(msg: &str) {
    let _ = writeln!(io::stderr(), "warn: {msg}");
}

pub fn err(msg: &str) {
    let _ = writeln!(io::stderr(), "error: {msg}");
}
