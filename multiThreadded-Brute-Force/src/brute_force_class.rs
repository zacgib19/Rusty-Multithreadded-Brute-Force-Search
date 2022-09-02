use std::char;
use std::collections::HashMap;

// Seen as class variables
pub struct BFSearch {
    max_length: i8,

    char_to_int_map: HashMap::<char, usize>,
    int_to_char_map: HashMap::<usize, char>,

    real_password: String,
    real_password_char_arr: Vec::<char>,

    pub pass_guess: String,
    pass_guess_char_arr: Vec::<char>,

    pub num_guesses: u128,
    curr_index: usize,       //Index for string array in binary search algorithm

    first_char: char,           
    last_char: char,
    last_guess: Vec::<char>,

    pub is_found: bool,
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(max_length: i8, input_password: &str, search_complexity: char) -> Self {
        let mut temp_char_list: Vec<char> = Vec::new(); //Temporary char array used for char_from_int_map
        let mut char_to_int_map: HashMap<char, usize> = HashMap::new();
        let mut int_to_char_map: HashMap<usize, char> = HashMap::new();
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
                for ch in ' '..='\u{D7FF}' {
                    temp_char_list.push(ch);
                }
                // Skips invalid characters \u{D800} through \u{DFFF}
                for ch2 in '\u{E000}'..='\u{10FFFF}' {
                    temp_char_list.push(ch2);
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

        //Create HashMaps from list
        for i in 0..temp_char_list.len() {
            int_to_char_map.insert(i, temp_char_list[i]);
            char_to_int_map.insert(temp_char_list[i], i);
        }
        
        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            max_length,

            //HashMaps
            char_to_int_map,
            int_to_char_map,

            real_password: String::from(input_password),
            real_password_char_arr: input_password.chars().collect::<Vec<char>>(),

            pass_guess: String::new(),
            pass_guess_char_arr: vec![temp_f_char],
            
            num_guesses: 0,
            curr_index: 0,

            first_char: temp_f_char,
            last_char: temp_L_char,
            last_guess: Vec::new(),

            is_found: false,
        }
    }

   
    // Starts brute force search
    pub fn start_search (&mut self) {   
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
    
    // Converts char array to string
    fn cleanup_to_string(&mut self) {
        for ch in &self.pass_guess_char_arr {
            self.pass_guess.push(*ch);
        }
    }

    // Updates pass_guess_char_arr in binary search fashion
    fn str_next(&mut self) -> Vec<char> {
        // If char at index is not the last character in self.char_from_int_map
        if self.pass_guess_char_arr[self.curr_index] != self.last_char {

            let mut temp_char = self.pass_guess_char_arr[self.curr_index];  
            let mut temp_int = self.char_to_int_map[&temp_char];

            temp_int += 1;
            
            // Change pass_guess_char_arr' character at curr_index position
            self.pass_guess_char_arr.remove(self.curr_index);
            
            self.pass_guess_char_arr.insert(self.curr_index, self.int_to_char_map[&(temp_int as usize)]);
            
            return self.pass_guess_char_arr.clone();
        }

        // If char at index is last 
        else {   
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
                // Increment currIndexes
                self.curr_index += 1;
                
                // Recursive check for last char
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