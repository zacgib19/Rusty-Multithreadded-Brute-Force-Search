use std::char;
use std::collections::HashMap;
extern crate num_cpus;

// Seen as class variables
pub struct MTBFSearch {
    max_length: u128,

    list_of_char_options: Vec::<char>,          //Used in initialization and master search
    char_to_int_map: HashMap::<char, u32>,    //Used in each thread
    int_to_char_map: HashMap::<u32, char>,    
    
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
        let mut temp_char_list: Vec<char> = Vec::new(); //Temporary char array used
        let mut char_to_int_map: HashMap<char, u32> = HashMap::new();
        let mut int_to_char_map: HashMap<u32, char> = HashMap::new();
        
        // Sets unicode list to iterate over
        match search_complexity {
            // Basic ASCII
            'B'|'b' => {
                //DEBUGGING should be from ' ' to '~'
                for ch in ' '..='~' {   
                    temp_char_list.push(ch);
                }           
            },

            // Full Unicode Library
            'F'|'f' => {
                // From space character throughout the entire unicode library
                for ch in ' '..='𫠝' {
                    // If valid unicode character, then push to list (For loop based on codepoints, not valid characters)
                    let codepoint = ch as u32;
                    match char::from_u32(codepoint) {
                        Some(_) => {
                            temp_char_list.push(ch);
                        },
                        None => {
                        },
                    };
                }
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid search_complexity character passed in")
            }
        }
        
        // Gets first and last character
        let temp_f_char: char = temp_char_list[0]; //Temporary character used for first_char
        let temp_L_char: char = temp_char_list[temp_char_list.len()-1]; //Temporary character used for last_char

        //Create HashMaps from temp_char_list (Starts counting at 1)
        for i in 0..temp_char_list.len() {
            int_to_char_map.insert((i+1) as u32, temp_char_list[i]);
            char_to_int_map.insert(temp_char_list[i], (i+1) as u32);
        }

        // Handles 0th position (null) character of base 95
        // (character is actually outside given unicode range)
        int_to_char_map.insert(0, '􀀀');
        char_to_int_map.insert('􀀀', 0);

        let max_length = max_length as u128;

        // Calculate max amount of guesses
        let mut max_guess: u128 = 0;
        let num_chars: u128 = temp_char_list.len() as u128;
        for len_index in 1..=max_length {
            max_guess += u128::pow(num_chars, len_index as u32);
        }

        // Function that converts specific guess to starting password for thread
        // WITHOUT Looping through guesses again
        fn guess_to_char_array(guess_num: u128, base: u128, chr_list: &HashMap<u32,char>) -> Vec<char> {
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
                let ch = chr_list[&(i as u32)];
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
            vec_of_pass_guesses.push(guess_to_char_array(starting_point, num_chars, &int_to_char_map));
        }

        // "!!" for Full Search
        //println!("{:?}", guess_to_char_array(352254, num_chars, &int_to_char_map))
        println!("Char_Array of starting_guesses: {:?}", vec_of_pass_guesses);

        // Initalizes and returns MTBFsearch Struct (no semicolon to return struct)
        Self {
            max_length,

            list_of_char_options: temp_char_list,
            char_to_int_map,
            int_to_char_map,

            real_password: String::from(input_password),
            real_password_char_arr: input_password.chars().collect::<Vec<char>>(),

            pass_guess: String::new(),
            pass_guess_char_arr: vec!(temp_f_char),
                
            max_num_guesses: max_guess,
            num_guesses: 0,
            guessing_size,

            curr_index: 0,
            first_char: temp_f_char,
            last_char: temp_L_char,
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
        // If char at index is not the last character
        if self.pass_guess_char_arr[self.curr_index] != self.last_char {

            let temp_char = self.pass_guess_char_arr[self.curr_index];
            let next_uni_codepoint: u32 = (temp_char as u32 + 1) as u32;
            let next_char_mapping: u32 = next_uni_codepoint-31;
            
            //println!("{:?}", self.pass_guess_char_arr);
            // Change pass_guess_char_arr' character at curr_index position
            self.pass_guess_char_arr.remove(self.curr_index);
            
            self.pass_guess_char_arr.insert(self.curr_index, char::from_u32(next_uni_codepoint)
                                                            .unwrap_or(self.int_to_char_map[&next_char_mapping]));
            
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