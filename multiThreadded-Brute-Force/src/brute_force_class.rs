use unicode_segmentation::UnicodeSegmentation;
use std::char;
use std::collections::HashMap;


// Seen as class variables
pub struct BFSearch {
    max_length: i8,
    real_password: Vec::<char>,
    pub pass_guess: Vec::<char>,
    last_guess: Vec::<char>,
    char_from_int_map: HashMap<i32, String>,
    int_from_char_map: HashMap<String, i32>,
    pub num_guesses: u128,
    curr_index: usize,       //Index for string array in binary search algorithm (in graphemes)
    first_char: char,           
    last_char: char,
    pub is_found: bool,
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(max_length: i8, password: String, search_complexity: char) -> Self {
        let mut temp_char_map = HashMap::new();
        let mut temp_int_map = HashMap::new();
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
                // From null character throughout the entire unicode library
                for ch in ''..='𫠝' {
                    temp_char_list.push(ch);
                }
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid search_complexity character passed in")
            }

        }

        // Convert vector to hashmap
        for i in 0..temp_char_list.len() {
            temp_char_map.insert(i as i32, String::from(temp_char_list[i]));  //used for charToInt
            temp_int_map.insert(String::from(temp_char_list[i]), i as i32);   //used for intToChar
        } 
       

        // Gets first and last character
        temp_f_char = temp_char_list[0];
        temp_L_char = temp_char_list[temp_char_list.len()-1];


        
        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            max_length,
            real_password: password.chars().collect::<Vec<char>>(),
            pass_guess: Vec::new(),
            last_guess: Vec::new(),
            char_from_int_map: temp_char_map,
            int_from_char_map: temp_int_map,
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
        
        while true {
            //println!("{:?}", self.pass_guess);
            if self.is_pw_match() {
                self.is_found = true; 
                
                break;
            }

            else if self.is_last_guess() {
                break;
            }

            else {
                self.curr_index = 0;
                
                self.pass_guess = self.str_next();
                self.num_guesses += 1;
            }            
        }    
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
        self.pass_guess == self.real_password
    }

    // Check to see if search needs to end
    fn is_last_guess(&self) -> bool {
        self.pass_guess == self.last_guess
    }
    

    // Updates pass_guess in binary search fashion
    fn str_next(&mut self) -> Vec<char> {

        // For very first guess
        if self.pass_guess.len() == 0 {
            // Add first character
            self.pass_guess.push(self.first_char);
            return self.pass_guess.clone();
        }

        
        // For every other guess
        else {
            // New instance of this supposed to run each recursion of str_next()

            // USE UNICODE CODEPOINT TO GET NEXT CHARACTER

            // If char at index is not the last character in self.char_from_int_map
            if self.pass_guess[self.curr_index] != self.last_char {

                let mut temp_char = self.pass_guess[self.curr_index]; //Cant use grapheme library, returns &str, not char  
                let mut temp_int = temp_char as u32;

                temp_int += 1;
                
                // Change pass_guess' character at curr_index position
                self.pass_guess.remove(self.curr_index);
                
                self.pass_guess.insert(self.curr_index, char::from_u32(temp_int).unwrap());
                
                return self.pass_guess.clone();
            }

            // If char at index is last 
            else {   
                //println!(", Selected char: {:?}", self.pass_guess.graphemes(true).nth(self.curr_index).unwrap());

                // If only character in self.pass_guess
                if self.pass_guess.len() == 1 {
                    
                    // Reset first character
                    self.pass_guess.remove(0);
                    self.pass_guess.insert(0, self.first_char);
                    // Add second character
                    self.pass_guess.push(self.first_char);
                    return self.pass_guess.clone();
                }


                // Else if time to add another letter
                else if self.pass_guess.len() == (self.curr_index + 1) {
                    
                    // Replace character at index with first character of char_from_int_map
                    self.pass_guess.remove(self.curr_index);
                    self.pass_guess.insert(self.curr_index, self.first_char);
                    // Append first character of char_from_int_map to pass_guess
                    self.pass_guess.push(self.first_char);

                    return self.pass_guess.clone();
                }

                // If last possible string to check
                else if self.is_last_guess() {
                    // Do nothing and return
                    return self.pass_guess.clone();
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