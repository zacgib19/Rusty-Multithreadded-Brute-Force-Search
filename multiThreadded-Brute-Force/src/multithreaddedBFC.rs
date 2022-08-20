use unicode_segmentation::UnicodeSegmentation;
use std::char;
use std::collections::HashMap;
use num_cpus;

// Seen as class variables
pub struct MTBFSearch {
    max_length: i8,
    real_password: String,
    real_password_char_arr: Vec::<char>,
    pub pass_guess: String,
    pass_guess_char_arr: Vec::<char>,
    last_guess: Vec::<char>,
    max_num_guesses: u128,
    pub num_guesses: u128,
    curr_index: usize,       //Index for string array in binary search algorithm (in graphemes)
    first_char: char,           
    last_char: char,
    pub is_found: bool,
}

// Seen as class functions
impl MTBFSearch {

    // Constructor that implements default variables
    pub fn new(max_length: i8, input_password: &str, search_complexity: char) -> Self {
        let mut temp_char_list: Vec<char> = Vec::new(); //Temporary char array used for char_from_int_map
        let mut temp_f_char: char = ' '; //Temporary character used for first_char
        let mut temp_L_char: char = ' '; //Temporary character used for last_char
        
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
                    temp_char_list.push(ch);
                }
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid search_complexity character passed in")
            }
        }      

        // Gets first and last character
        temp_f_char = temp_char_list[0];
        temp_L_char = temp_char_list[temp_char_list.len()-1];

        // Figure out max amount
        let mut max_guess: u128 = 0;
        let mut num_chars: u128 = temp_char_list.len() as u128;
 
        for len_index in (1..=max_length) {
            max_guess += u128::pow(num_chars, len_index as u32);
        }
        
        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            max_length,
            real_password: String::from(input_password),
            real_password_char_arr: input_password.chars().collect::<Vec<char>>(),
            pass_guess: String::new(),
            pass_guess_char_arr: Vec::new(),
            last_guess: Vec::new(),
            max_num_guesses: max_guess,
            num_guesses: 0,
            curr_index: 0,
            first_char: temp_f_char,
            last_char: temp_L_char,  
            is_found: false,
        }
    }

    
   
    // Starts brute force search
    pub fn start_search (&mut self) {   
        self.get_last_guess();
        
        loop {
            if self.is_pw_match() {
                self.is_found = true;   
                break;
            } else if self.is_last_guess() {
                break;
            } else {
                self.curr_index = 0;
                
                self.pass_guess_char_arr = self.str_next();
                self.num_guesses += 1;
            }            
        }
        
        self.cleanup_to_string();
    }

    // Sets last_guess to max_length copies of the last character in char_from_int_map
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
        self.pass_guess_char_arr == self.last_guess
    }
    
    fn cleanup_to_string(&mut self) {
        for ch in &self.pass_guess_char_arr {
            self.pass_guess.push(*ch);
        }
    }

    // Updates pass_guess_char_arr in binary search fashion
    fn str_next(&mut self) -> Vec<char> {

        // For very first guess
        if self.pass_guess_char_arr.len() == 0 {
            // Add first character
            self.pass_guess_char_arr.push(self.first_char);
            return self.pass_guess_char_arr.clone();
        }

        
        // For every other guess
        else {
            // New instance of this supposed to run each recursion of str_next()

            // USE UNICODE CODEPOINT TO GET NEXT CHARACTER

            // If char at index is not the last character in self.char_from_int_map
            if self.pass_guess_char_arr[self.curr_index] != self.last_char {

                let mut temp_char = self.pass_guess_char_arr[self.curr_index]; //Cant use grapheme library, returns &str, not char  
                let mut temp_int = temp_char as u32;

                temp_int += 1;
                
                // Change pass_guess_char_arr' character at curr_index position
                self.pass_guess_char_arr.remove(self.curr_index);
                
                self.pass_guess_char_arr.insert(self.curr_index, char::from_u32(temp_int).unwrap_or('�'));
                
                return self.pass_guess_char_arr.clone();
            }

            // If char at index is last 
            else {   
                //println!(", Selected char: {:?}", self.pass_guess_char_arr.graphemes(true).nth(self.curr_index).unwrap());

                // If only character in self.pass_guess_char_arr
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
                    
                    // Replace character at index with first character of char_from_int_map
                    self.pass_guess_char_arr.remove(self.curr_index);
                    self.pass_guess_char_arr.insert(self.curr_index, self.first_char);
                    // Append first character of char_from_int_map to pass_guess_char_arr
                    self.pass_guess_char_arr.push(self.first_char);

                    return self.pass_guess_char_arr.clone();
                }

                // If last possible string to check
                else if self.is_last_guess() {
                    // Do nothing and return
                    return self.pass_guess_char_arr.clone();
                }

                else {
                    
                    //Increment currIndexes
                    self.curr_index += 1;
                    
                    let mut return_string = self.str_next();
                    
                    //Decrement currIndexes
                    self.curr_index -= 1;
  
                    // Replace character at curr_index with first char
                    return_string.remove(self.curr_index);
                    return_string.insert(self.curr_index, self.first_char);

                    return return_string;
                }
            };
        }
    }   
}