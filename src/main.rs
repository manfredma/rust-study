use std::cmp::Ordering;
use std::io;
use std::fs;

use ferris_says::say;

use rand::Rng;

fn main() {
    hello_world();
    guess_game();
    reference();

    println!("{}", enum_test());
}

fn enum_test() -> i32 {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let coin: Coin = Coin::Penny(String::from("OOH"));

    match coin {
        Coin::Penny(str) => {
            println!("Luck penny! \n {:?}", Coin::Penny(String::from(str)));
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Coin {
    Penny(String),
    Nickel,
    Dime,
    Quarter,
}


fn reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' length is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn hello_world() {
    let stdout = io::stdout();
    let message = String::from("hello, rust");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello, world!");
}

fn guess_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
