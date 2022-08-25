use std::io;
use unicode_segmentation::UnicodeSegmentation;
use std::time::Instant;
mod brute_force_class;
mod multithreaddedBFC;

fn main() {
    let mut max_length: i8;
    let mut password = String::new();
    let mut complexity_choice: char;
    let mut has_guessed_correct: bool = false;
    let mut want_to_crack: bool = true;

    //Welcome message
    println!("\n\n\nHello! Welcome to this brute force program!\
    \nHere, you will be prompted for a password, and the computer will\
    attempt to brute force guess the password.");

    while want_to_crack {
        
        // Prompt for max_length
        // Ask again if not an integer
        // Ask again if less than 1 but bigger than 7
        loop {           
            println!("Please enter the maximum length of password you want to guess (greater than 0, less than 10): ");
            let mut max_len_input = String::new();

            io::stdin().read_line(&mut max_len_input).expect("Failed to read line");

            // Checks if entered string is number
            let mut max_len_input: i8 = match max_len_input.trim().parse() {
                Ok(num) => num,

                Err(_) => {
                    println!("\nInvalid Entry!");
                    continue;
                },
            };
            
            if max_len_input <= 0 {
                println!("\nNumber too small!");
                continue;
            }
            else if max_len_input > 10 {
                println!("\nNumber too big, it will take too long to brute force!");
                continue;
            }
            max_length = max_len_input;

            break;
        }

        // Asks for password
        // Ask again if string is too long
        loop {
            let mut pass_input = String::new();
            
            println!("\nPlease enter a password to guess: ");
            io::stdin().read_line(&mut pass_input).expect("Failed to read line");
            
            let mut num_of_char: i8 = pass_input.trim().graphemes(true).count() as i8;

            // Handles bug where last char is a space
            if pass_input.chars().last().unwrap() == ' ' {
                num_of_char -= 1;
            }

            if num_of_char as i8 > max_length {
                println!("\nPassword too long!");
                continue;
            } else {
                password = pass_input;
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
                 \nFull: Will search through EVERY character in the entire Unicode Library.\
                 \n(VERY Slow, but covers ALT-Code characters, and characters from every typable language\n");
        
        // Asks for search complexity
        // Asks again if not a char
        // Asks again if not correct character
        loop {
            let mut complexity_input = String::new();
            println!("Please enter either B for Basic, or F for full.");
            io::stdin().read_line(&mut complexity_input).expect("Failed to read line");

            let num_of_char: i8 = complexity_input.trim().graphemes(true).count() as i8;
        
            if num_of_char == 0 {
                println!("Invalid Entry! (didn't enter anything)");
                continue;
            } else {
                // Converts to string slice, then trims the trailing newline
                let mut complexity_input: &str = &complexity_input[..];
                complexity_input = complexity_input.trim();

                match complexity_input {
                    "B"|"b"|"Basic"|"basic"|"BASIC" => complexity_choice = 'B',
                    "F"|"f"|"Full"|"full"|"FULL" => complexity_choice = 'F',
                    _=> {
                        println!("\nInvalid entry! (not right character)");
                        continue;
                    }
                };
                break;
            }
        }

        println!("\nStarting normal brute force cracking. NOTE, this may take a while!");

        // CALL BFSEARCH STRUCT instance HERE  
        /*  
        let mut BFS = brute_force_class::BFSearch::new(max_length, &password, complexity_choice);

        let start_BF_time = Instant::now();
        BFS.start_search();
        let stop_BF_time = Instant::now();

        let BF_time_elapsed = stop_BF_time - start_BF_time;
        let BF_time_elapsed = BF_time_elapsed.as_millis();
        let has_BF_guessed_correct = BFS.is_found;
        
        if has_BF_guessed_correct {
            println!("Password found! Your password was: {:?}", BFS.pass_guess);
            println!("It took {} tries to guess, and {:?} milliseconds to crack!", BFS.num_guesses, BF_time_elapsed);
        } else {
            println!("Despite {} guesses, your password couldn't be cracked. Great work!", BFS.num_guesses);
        }
        */
        //CALL MTBFS HERE
        
        let mut MTBFS = multithreaddedBFC::MTBFSearch::new(max_length, &password, complexity_choice);

        let start_MTBF_time = Instant::now();
        MTBFS.single_thread_search();
        let stop_MTBF_time = Instant::now();

        let MTBF_time_elapsed = stop_MTBF_time - start_MTBF_time;
        let MTBF_time_elapsed = MTBF_time_elapsed.as_millis();
        let has_MTBF_guessed_correct = MTBFS.is_found;

        //This line will be used when multithreadded is done
        //has_guessed_correct = (BFS.isFound || MTBFS.isFound);

        if has_MTBF_guessed_correct {
            println!("Password found! Your password was: {:?}", MTBFS.pass_guess);
            println!("It took {} tries to guess, and {:?} milliseconds to crack!", MTBFS.num_guesses, MTBF_time_elapsed);
        } else {
            println!("Despite {} guesses, your password couldn't be cracked. Great work!", MTBFS.num_guesses);
        }

        // Asks user if they want to continue 
        // Asks again if responce is not Yes or no
        // Convert to boolean for want_to_crack
        loop {
            let mut want_crack_input = String::new();
            println!("Do you wanna crack another password?");
            io::stdin().read_line(&mut want_crack_input).expect("Failed to read line");

            // Converts to string slice, then trims the trailing newline
            let mut want_crack_input: &str = &want_crack_input[..];
            want_crack_input = want_crack_input.trim();

            match want_crack_input {
                "Yes"|"yes"|"Y"|"y"|"YES" => want_to_crack = true,
                "No"|"no"|"N"|"n"|"NO" => want_to_crack = false,
                _ => {
                    println!("\nInvalid entry!");
                    continue;
                }            
            }

            break;
        }
    }
}
