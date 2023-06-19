use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("hello, 单元包");

    for (key, value) in env::vars() {
        println!("{} => {}", key, value);
    }

    let f = File::open("Cargo.toml");
    match f {
        Ok(mut file) => {
            let mut x = String::new();
            file.read_to_string(&mut x).expect("TODO: panic message");
            println!("{}", x);
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    println!("END!++++++++++++++++++++++++++++++++++++++++++++++++++++")

}