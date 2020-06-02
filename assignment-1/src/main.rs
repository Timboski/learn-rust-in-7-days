// Assignment 1 : Say Hello to everyone in the program argument
// list whose name begins with the letter W.

use std::env::args;

fn main() {
    for name in args() {
        if let Some(first_letter) = name.chars().next() {
            match first_letter {
                'W'|'w' => println!("Hello {}", name),
                _ => {},
            }
        }
    }
}
