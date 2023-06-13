use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("hello, rust");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello, world!");
}
