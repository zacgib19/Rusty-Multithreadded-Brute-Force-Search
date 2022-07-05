use std::io;

// Seen as class variables
pub struct BruteForceSearch {
    maxLen: i8,
    realPassword: String::new(),
    passToTry: String::new(),
    unicodeList: Vec<char>,
    numGuesses: u128,
    searchComplexity: char,
    lastGuess: String::new(),
};

// Seen as class functions
impl BruteForceSearch {
    // Adds characters to unicodeList
    pub fn setSearchLen(&mut self) {
        match searchComplexity {
            'B'|'b' => {
                self.unicodeList = (' '..='~').collect();
                // Debugging
                println!("{}", unicodeList);
            },

            'E'|'e' => {

            },

            'F'|'f' => {

            },

            _ => {
                panic!("Invalid character. IDK how you got an \
                        invalid within the struct/implementation, but here we are.");
            }

        }
    }

    pub fn
};