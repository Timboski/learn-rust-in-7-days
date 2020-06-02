// Assignment 1 : Say Hello to everyone in the program argument
// list whose name begins with the letter W.

use std::env::args;

fn main() {
    for name in args() {
        println!("Hello {}", name);
    }    
}
