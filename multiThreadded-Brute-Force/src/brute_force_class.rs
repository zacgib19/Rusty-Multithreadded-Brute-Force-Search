use std::char;

// Seen as class variables
pub struct BFSearch {
    real_password_char_arr: Vec::<char>,

    pub pass_guess: String,  // Only used for converting to string at the very end
    pub pass_guess_char_arr: Vec::<char>,

    pub num_guesses: u128,
    max_num_guesses: u128,
    curr_index: usize,       //Index for string array in binary search algorithm

    first_char: char,           
    last_char: char,
    num_possible_chars: u128,

    pub is_found: bool
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(max_length: i8, input_password: &str, search_complexity: char) -> Self {
        let mut temp_char_list: Vec<char> = Vec::new();
        let temp_f_char: char = ' '; //Temporary character used for first_char
        let mut temp_l_char: char = ' '; //Temporary character used for last_char
        
        // Sets unicode list to iterate over
        match search_complexity {
            // Basic ASCII
            'B'|'b' => {
                temp_l_char = '~';          
            },

            // Full Unicode Library
            'F'|'f' => {
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

        //Calculate number of possible characters (used for base n conversion)
        let mut num_possible_chars: u128 = ((temp_l_char as u32) - (temp_f_char as u32)).try_into().unwrap(); 
        println!("{:?}", num_possible_chars);

        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            real_password_char_arr: input_password.chars().collect::<Vec<char>>(),

            pass_guess: String::new(),
            pass_guess_char_arr: vec![temp_f_char],
            
            num_guesses: 0,
            max_num_guesses: max_guess,
            curr_index: 0,

            first_char: temp_f_char,
            last_char: temp_l_char,
            num_possible_chars,

            is_found: false
        }
    }

   
    // Starts brute force search
    pub fn start_search (&mut self) {   
        
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
                //println!("{:?}", self.pass_guess_char_arr)
            }            
        }
        
        self.cleanup_to_string();
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
        
        let mut list_of_remainders: Vec::<u128> = vec!();
        let mut dividend = self.num_guesses;

        // Converts from base 10 to base 'n', where n is amount of possible characters
        // Another way to conceptually think about passwords
        while dividend > 0 {
            list_of_remainders.push(dividend % self.num_possible_chars);
            //print!("{}", dividend);
            dividend = dividend / self.num_possible_chars; //Integer division
            println!(", to {} w/ remainder {:?}", dividend, list_of_remainders[list_of_remainders.len()-1]);
        }
        
        //println!("{:?}", list_of_remainders);
        // Convert remainders to characters
        let mut vec_char: Vec<char> = Vec::new();
        for i in list_of_remainders {
            let ch = chr_list[i as usize];
            vec_char.push(ch);
        }
        //println!("Remainders turn into: {:?} \n", vec_char);
        return vec_char;
    }   
}