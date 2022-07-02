 // Find library for keeping time for measuring how long it takes to crack
use std::io;
use BruteForceSearch;

fn main() {
    let mut maxLength: i16 = 0;
    let mut input_text = String::new();
    let mut password = String::new();
    let mut passGuess
    let mut complexChoice: char = ' ';
    let mut hasGuessedCorrect: bool = false;
    let mut numGuesses: u32 = 0;
    let mut wantToCrack: bool = true;

    //Welcome message
    println!("\n\n\nHello! Welcome to this brute force program!\
    \nHere, you will be prompted for a password, and the computer will\
    attempt to brute force guess the password."

    while (wantToCrack) {
        
        // Prompt for maxLength
        // Ask again if not an integer
        println!("\nPlease enter the maximum length of password you want to guess: ");  
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        // Asks for password
        // Ask again if string is too long
        println!("\nPlease enter a password to guess: ");
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        // Asks for search complexity
        // Asks again if not a char
        // Asks again if not correct character
        println!("How in-depth would you like this search to go?\
                \nBasic: Only searches through the ASCII-code characters\
                \n(Faster, but will miss passwords with ALT-Code characters. Examples: Ö, ÿ, ☺)\n\
                \nExtended: Will search through every one of the ALT-code characters\
                \n(Slower, but will check every typable password)\n\
                \nFull: Will search through EVERY character in the entire Unicode Library.\
                \n(VERY Slow, but covers ALT-Code characters, and characters from every typable language\n\
                \n(Please type 'B' for Basic, 'E' for Extended, or 'F' for full):");
        io::stdin().read_line(&mut input_text).expect("Failed to read line");

        // CALL BFSEARCH STRUCT HERE

        // Asks user if they want to continue 
        // Asks again if responce is not Yes or no
        // Convert to boolean for wantToCrack
        println!("Do you wanna crack another password?");
        io::stdin().read_line(&mut input_text).expect("Failed to read line");
    };
    

}
