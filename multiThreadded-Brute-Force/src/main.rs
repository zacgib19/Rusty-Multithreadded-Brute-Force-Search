 // Find library for keeping time for measuring how long it takes to crack
use std::io;

fn main() {
    let mut maxLength: i16 = 0;
    let mut input_text = String::new();
    let choice: char = ' ';
    let hasGuessed: bool = false;
    let numGuesses: u32 = 0;
    println!("\n\n\nHello! Welcome to this brute force program!\
             \nHere, you will be prompted for a password, and the computer will\
             attempt to brute force guess the password.\
             \nPlease enter the maximum length of password you want to guess: ");

    io::stdin() // Incoporate user validation somehow
        .read_line(&mut input_text)
        .expect("Failed to read line");



}
