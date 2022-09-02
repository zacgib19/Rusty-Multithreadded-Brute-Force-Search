use std::char;
use std::collections::HashMap;
extern crate num_cpus;

// Seen as class variables
pub struct MTBFSearch {
    max_length: u128,  
    
    real_password: String,
    real_password_char_arr: Vec::<char>,

    pub pass_guess: String,
    pass_guess_char_arr: Vec::<char>,

    max_num_guesses: u128,
    pub num_guesses: u128,
    guessing_size: u128,

    curr_index: usize,       //Index for string array in binary search algorithm
    first_char: char,           
    last_char: char,
    last_guess: Vec::<char>,

    pub is_found: bool,
}

// Seen as class functions
impl MTBFSearch {

    // Constructor that implements default variables
    pub fn new(max_length: i8, input_password: &str, search_complexity: char) -> Self {
        let mut temp_char_list: Vec<char> = Vec::new(); //Temporary char array used only for guessToCharArray function

        // Gets first and last character
        let temp_f_char: char = ' '; //Temporary character used for first_char
        let mut temp_l_char: char = ' '; //Temporary character used for last_char
        
        // Sets unicode list to iterate over
        match search_complexity {
            // Basic ASCII
            'B'|'b' => {
                //DEBUGGING should be from ' ' to '~'
                for ch in ' '..='~' {   
                    temp_char_list.push(ch);
                }
                temp_l_char = '~';          
            },

            // Full Unicode Library
            'F'|'f' => {
                // From space character throughout the entire unicode library
                for ch in ' '..='\u{D7FF}' {
                    temp_char_list.push(ch);
                }
                // Skips invalid characters \u{D800} through \u{DFFF}
                for ch2 in '\u{E000}'..='\u{10FFFF}' {
                    temp_char_list.push(ch2);
                }
                temp_l_char = '\u{10FFFF}';
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid search_complexity character passed in")
            }
        }

        // Converts max_length to u128
        let max_length = max_length as u128;

        // Calculate max amount of guesses
        let mut max_guess: u128 = 0;
        let num_chars: u128 = temp_char_list.len() as u128;
        for len_index in 1..=max_length {
            max_guess += u128::pow(num_chars, len_index as u32);
        }

        // Handles 0th position (null) character
        // null character can't be included in max_guess calculation
        temp_char_list.insert(0, '\0');

        // Function that converts specific guess to starting password for thread
        // WITHOUT Looping through guesses again
        fn guess_to_char_array(guess_num: u128, base: u128, chr_list: &Vec<char>) -> Vec<char> {
            let mut list_of_remainders: Vec::<u128> = vec!();
            let mut dividend = guess_num;

            // Converts to base n from base 10, where n is amount of possible characters
            // Another way to conceptually think about passwords
            while dividend > 0 {
                list_of_remainders.push((dividend % base));
                print!("{}", dividend);
                dividend = dividend / base; //Integer division
                println!(", to {} w/ remainder {:?}", dividend, list_of_remainders[list_of_remainders.len()-1]);
            }
            
            println!("{:?}", list_of_remainders);
            // Convert remainders to characters
            let mut vec_char: Vec<char> = Vec::new();
            for i in list_of_remainders {
                let ch = chr_list[i as usize];
                vec_char.push(ch);
            }
            println!("Remainders turn into: {:?} \n", vec_char);
            return vec_char;
        }

        // Set up vec of vecs for passguess with starting guesses
        let num_threads: u128 = (num_cpus::get_physical()) as u128; // Get number of cores in the system
        let mut vec_of_pass_guesses: Vec<Vec<char>> = Vec::new(); // Vec<char> is faster than strings
        let guessing_size = max_guess / num_threads; //Integer division
        
        // Evenly divides the starting points
        for i in 0..num_threads {
            let starting_point = i*guessing_size; //Starting guess # of each thread
            vec_of_pass_guesses.push(guess_to_char_array(starting_point, num_chars, &temp_char_list));
        }

        println!("Char_Array of starting_guesses: {:?}", vec_of_pass_guesses);

        // Initalizes and returns MTBFsearch Struct (no semicolon to return struct)
        Self {
            max_length,

            real_password: String::from(input_password),
            real_password_char_arr: input_password.chars().collect::<Vec<char>>(),

            pass_guess: String::new(),
            pass_guess_char_arr: vec!(temp_f_char),
                
            max_num_guesses: max_guess,
            num_guesses: 0,
            guessing_size,

            curr_index: 0,
            first_char: temp_f_char,
            last_char: temp_l_char,
            last_guess: Vec::new(),

            is_found: false,
        }
    }
    
    // Master Controller over threads, initiallizes search
    pub fn start_search (&mut self) {
        
    }

    // Starts brute force search
    pub fn single_thread_search (&mut self) {   
        self.get_last_guess();
        
        loop {
            self.num_guesses += 1;
            if self.is_pw_match() {
                self.is_found = true;   
                break;
            } else if self.is_last_guess() {
                break;
            } else {
                self.curr_index = 0;
                self.pass_guess_char_arr = self.str_next();  
            }            
        }
        self.cleanup_to_string();
    }

    // Sets last_guess to max_length copies of the last character
    fn get_last_guess (&mut self) {
        // Figure out last_guess
        for _i in 0..self.max_length {
            self.last_guess.push(self.last_char);
        }
    }

    // Constantly makes this check to see if password matches
    fn is_pw_match(&self) -> bool {
        self.pass_guess_char_arr == self.real_password_char_arr
    }

    // Check to see if search needs to end
    fn is_last_guess(&self) -> bool {
        self.num_guesses == self.max_num_guesses
    }
    
    // Converts char array to string
    fn cleanup_to_string(&mut self) {
        for ch in &self.pass_guess_char_arr {
            self.pass_guess.push(*ch);
        }
    }

    // Updates pass_guess_char_arr in binary search fashion
    fn str_next(&mut self) -> Vec<char> {
        //VERY fast unicode iterator
        struct UnicodeWrapper {
            current_loc: u32
        }

        impl UnicodeWrapper {
            fn new(id: u32) -> Self {
                Self {
                    current_loc: id
                }
            }
        }

        // Turn the struct into something that can be looped through
        impl Iterator for UnicodeWrapper {
            // Output of the iterator is a char
            type Item = char;
            // Returns next Unicode character, updating currentLoc
            // to the next possible location
            fn next(&mut self) -> Option<Self::Item> {
                // Exit the for loop if we've already gotten the last
                // Unicode character.
                if self.current_loc == 0x110000 { return None; }

                self.current_loc = match self.current_loc {
                    
                    //Skips invalid characters
                    0xd7ff => 0xe000,
                    // Bump up the count if everything is normal
                    _      => self.current_loc + 1
                };
                let result = char::from_u32(self.current_loc).unwrap();
                
                // Give result to for loop
                Some(result)
            }
        }

        // If char at index is the 'null' character
        if self.pass_guess_char_arr[self.curr_index] == '\0' {
            self.pass_guess_char_arr.remove(self.curr_index);
            self.pass_guess_char_arr.insert(self.curr_index, self.first_char);
            return self.pass_guess_char_arr.clone();
        }
        // If char at index is not the last character
        else if self.pass_guess_char_arr[self.curr_index] != self.last_char {
            let mut unicode_looper = UnicodeWrapper::new((self.pass_guess_char_arr[self.curr_index]) as u32);

            // Change pass_guess_char_arr' character at curr_index position
            self.pass_guess_char_arr.remove(self.curr_index);
            
            self.pass_guess_char_arr.insert(self.curr_index, unicode_looper.next().unwrap());//next_uni_codepoint)
                                                            //.unwrap_or(self.int_to_char_map[&next_char_mapping]));
            //println!("{:?}", self.pass_guess_char_arr);
            return self.pass_guess_char_arr.clone();
        }

        // If char at index is last 
        else {
            
            if self.pass_guess_char_arr.len() == 1 {
                // Reset first character
                self.pass_guess_char_arr.remove(0);
                self.pass_guess_char_arr.insert(0, self.first_char);
                // Add second character
                self.pass_guess_char_arr.push(self.first_char);
                return self.pass_guess_char_arr.clone();
            }

            // Else if time to add another letter
            else if self.pass_guess_char_arr.len() == (self.curr_index + 1) {
                // Replace character at index with first character
                self.pass_guess_char_arr.remove(self.curr_index);
                self.pass_guess_char_arr.insert(self.curr_index, self.first_char);
                // Append first character to pass_guess_char_arr
                self.pass_guess_char_arr.push(self.first_char);

                return self.pass_guess_char_arr.clone();
            }

            // If last possible string to check
            else if self.is_last_guess() {
                // Do nothing and return
                return self.pass_guess_char_arr.clone();
            }

            else {
                //Increment currIndex
                self.curr_index += 1;
                let mut return_string = self.str_next();
                //Decrement currIndex
                self.curr_index -= 1;

                // Replace character at curr_index with first char
                return_string.remove(self.curr_index);
                return_string.insert(self.curr_index, self.first_char);

                return return_string;
            }
        };
    }   
}