// Assignment 1 : Say Hello to everyone in the program argument
// list whose name begins with the letter W.

use std::env::args;

fn main() {
    for name in args() {
        for (index, letter) in name.char_indices() {
            match index {
                0 => match letter {
                    'W'|'w' => println!("Hello {}", name),
                    _ => break
                }
                _ => break
            }            
        }
    }    
}
