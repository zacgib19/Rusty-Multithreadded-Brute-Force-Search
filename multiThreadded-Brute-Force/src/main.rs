 // Find library for keeping time for measuring how long it takes to crack
use std::io;
use std::cmp::Ordering;
use std::env::args;
use unicode_segmentation::UnicodeSegmentation;
mod bruteForceClass;

fn main() {
    let mut maxLength: i8 = 0;
    let mut password = String::new();
    let mut complexityChoice: char;
    let mut hasGuessedCorrect: bool = false;
    let mut numGuesses: u32 = 0;
    let mut wantToCrack: bool = true;

    //Welcome message
    println!("\n\n\nHello! Welcome to this brute force program!\
    \nHere, you will be prompted for a password, and the computer will\
    attempt to brute force guess the password.");

    while wantToCrack {
        
        // Prompt for maxLength
        // Ask again if not an integer
        // Ask again if less than 1 but bigger than 7
        loop {           
            println!("Please enter the maximum length of password you want to guess (greater than 0, less than 10): ");
            let mut maxLenInput = String::new();

            io::stdin().read_line(&mut maxLenInput).expect("Failed to read line");

            // Checks if entered string is number
            let mut maxLenInput: i8 = match maxLenInput.trim().parse() {
                Ok(num) => num,

                Err(_) => {
                    println!("\nInvalid Entry!");
                    continue;
                },
            };
            
            if maxLenInput <= 0 {
                println!("\nNumber too small!");
                continue;
            }
            else if maxLenInput > 10 {
                println!("\nNumber too big, it will take too long to brute force!");
                continue;
            }
            maxLength = maxLenInput;

            break;
        }

        // Asks for password
        // Ask again if string is too long
        loop {
            let mut passInput = String::new();
            
            println!("\nPlease enter a password to guess: ");
            io::stdin().read_line(&mut passInput).expect("Failed to read line");
            
            let mut numOfChar: i8 = passInput.trim().graphemes(true).count() as i8;

            // Handles bug where last char is a space
            if passInput.chars().last().unwrap() == ' ' {
                numOfChar -= 1;
            }

            if numOfChar as i8 > maxLength {
                println!("\nPassword too long!");
                continue;
            }
            else {
                password = passInput;
                break;
            }
        }

        // Gets rid of whitespace after
        let password = password.trim();
        let password = String::from(password);
        
        // Details on search complexities
        println!("\nHow in-depth would you like this search to go?\
                 \nBasic: Only searches through the ASCII-code characters\
                 \n(Faster, but will miss passwords with ALT-Code characters. Examples: Ö, ÿ, ☺)\n\
                 \nExtended: Will search through every one of the ALT-code characters\
                 \n(Slower, but will check every typable password)\n\
                 \nFull: Will search through EVERY character in the entire Unicode Library.\
                 \n(VERY Slow, but covers ALT-Code characters, and characters from every typable language\n");
        
        // Asks for search complexity
        // Asks again if not a char
        // Asks again if not correct character
        loop {
            let mut complexityInput = String::new();
            println!("Please enter either B for Basic, E for Extended, or F for full.");
            io::stdin().read_line(&mut complexityInput).expect("Failed to read line");

            let mut numOfChar: i8 = complexityInput.trim().graphemes(true).count() as i8;
        
            if numOfChar == 0 {
                println!("Invalid Entry! (didn't enter anything)");
                continue;
            }

            else {
                // Converts to string slice, then trims the trailing newline
                let mut complexityInput: &str = &complexityInput[..];
                complexityInput = complexityInput.trim();

                match complexityInput {
                    "B"|"b"|"Basic"|"basic"|"BASIC" => complexityChoice = 'B',
                    "E"|"e"|"Extended"|"extended"|"EXTENDED" => complexityChoice = 'E',
                    "F"|"f"|"Full"|"full"|"FULL" => complexityChoice = 'F',
                    _=> {
                        println!("\nInvalid entry! (not right character)");
                        continue;
                    }
                };
                break;
            }
        }

        // CALL BFSEARCH STRUCT instance HERE      
        let mut BFS = bruteForceClass::BFSearch::new(maxLength, password, complexityChoice);

        BFS.startSearch();

        // Debugging
        BFS.debugging();
        

        // Asks user if they want to continue 
        // Asks again if responce is not Yes or no
        // Convert to boolean for wantToCrack
        loop {
            let mut wantCrackInput = String::new();
            println!("Do you wanna crack another password?");
            io::stdin().read_line(&mut wantCrackInput).expect("Failed to read line");

            // Converts to string slice, then trims the trailing newline
            let mut wantCrackInput: &str = &wantCrackInput[..];
            wantCrackInput = wantCrackInput.trim();

            match wantCrackInput {
                "Yes"|"yes"|"Y"|"y"|"YES" => wantToCrack = true,
                "No"|"no"|"N"|"n"|"NO" => wantToCrack = false,
                _ => {
                    println!("\nInvalid entry!");
                    continue;
                }            
            }

            break;
        }
    }
}
