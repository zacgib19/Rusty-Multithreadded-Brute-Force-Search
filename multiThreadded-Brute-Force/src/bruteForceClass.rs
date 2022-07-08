use std::io;
use std::cmp::Ordering;
use std::env::args;
use unicode_segmentation::UnicodeSegmentation;
use std::array::IntoIter;


// Seen as class variables
pub struct BFSearch {
    maxLength: i8,
    realPassword: String, //Immutable string slice
    passGuess: String,
    lastGuess: String,
    unicodeList: Vec<char>,
    numGuesses: u128,
    currentIndex: usize,  //Which string array slot in binary search algorithm
    currCharIndex: usize,      //Index for unicodeList
    lastChar: String,
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(maxLength: i8, realPassword: String, searchComplexity: char) -> Self {
        let mut tempCharList: Vec<char> = Vec::new(); //Temporary char array used for unicodeList
        let mut tempChar: char = ' '; //Temporary character used for lastChar
        
        // Sets unicode list to iterate over
        match searchComplexity {

            // Basic
            'B'|'b' => {
                for ch in ' '..='~' {
                    tempCharList.push(ch);
                }

                tempChar = tempCharList[tempCharList.len()-1];
            },

            // Extended
            'E'|'e' => {
                let extendedCharList = vec![
                    '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂',
                    '♀', '♪', '♫', '☼', '►', '◄', '↕', '‼', '¶', '§', '▬',
                    '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼',
                    '⌂', 'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë',
                    'è', 'ï', 'î', 'ì', 'Ä', 'Å', 'É', 'æ', 'Æ', 'ô', 'ö',
                    'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
                    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬',
                    '½', '¼', '¡', '«', '»', '░', '▒', '▓', '│', '┤', '╡',
                    '╢', '╖', '╕', '╣', '║', '╗', '╝', '╛', '┐', '└', '┴',
                    '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠',
                    '═', '╬', '╧', '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫',
                    '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 'α', 'ß', 'Γ',
                    'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ',
                    'ε', '∩', '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°',
                    '∙', '·', '√', 'ⁿ', '²', '■', '€', '‚', 'ƒ', '„', '…',
                    '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', 'Ž', '‘', '’', '“',
                    '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', 'ž', 'Ÿ',
                    '¡', '¤', '¥', '¦', '¨', '©', 'ª', '®', '¯', '°',
                    '³', '´', '·', '¸', '¹', 'º', '¾', '¿', 'À', 'Á', 'Â',
                    'Å', 'È', 'Ê', 'Ë', 'Ì', 'Í', 'Ï', 'Ð', 'Ò', 'Ó',
                    'Ô', 'Õ', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß',
                    'ã', 'å', 'æ', 'ç', 'í', 'î', 'ï',
                    'ð', 'õ', '÷', 'ø', 'ü', 'ý', 'þ', 'ÿ'
                ];

                for ch in ' '..='~' {
                    tempCharList.push(ch);
                }

                for ch in extendedCharList {
                    tempCharList.push(ch);
                }

                tempChar = tempCharList[tempCharList.len()-1];
            },

            // Full
            'F'|'f' => {
                // From null character throughout the entire unicode library
                for ch in ''..='𫠝' {
                    tempCharList.push(ch);
                }
                tempChar = tempCharList[tempCharList.len()-1];
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid searchComplexity character passed in")
            }

        }
        

        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            maxLength,
            realPassword,
            passGuess: String::new(),
            lastGuess: String::new(),
            unicodeList: tempCharList,
            numGuesses: 0,
            currentIndex: 0,
            currCharIndex: 0,
            lastChar: String::from(tempChar),
        }
    }

   
    // Starts brute force search
    pub fn startSearch (&mut self) {   
        self.getLastGuess();

        
        while !(self.isLastGuess() && self.isPasswordMatch()) {
            self.numGuesses += 1;
            self.updateGuess();

            //Debugging
            print!("{}, ", self.passGuess)
        }
        
    }

    // Sets lastGuess to maxLength copies of the last character in unicodeList
    fn getLastGuess (&mut self) {
        // Figure out lastGuess
        for i in 0..self.maxLength {
            self.lastGuess.push_str(&self.lastChar);
        }
    }

    // Updates passGuess in binary search fashion
    fn updateGuess(&mut self) {

        // For very first guess
        if self.passGuess.len() == 0 {
            // Add first character
            self.passGuess.push(self.unicodeList[0]);
        }

        // For every other guess
        else {
            // If char at index is not the last character in self.unicodeList
            if (self.currCharIndex != self.unicodeList.len()) {
                self.currCharIndex += 1;
                
                // Change passGuess
                self.passGuess.pop();
                self.passGuess.push(self.unicodeList[self.currCharIndex]);
            }

            // If last character
            else {

            }
        }
    }

    // Constantly makes this check to see if password matches
    fn isPasswordMatch(&self) -> bool {
        self.passGuess == self.realPassword
    }

    // Check to see if search needs to end
    fn isLastGuess(&self) -> bool {
        self.passGuess == self.lastGuess
    }

    // Debugging
    pub fn debugging(&self) {
        //let debug: bool = (self.passGuess.graphemes(true).nth(self.currentIndex).unwrap() != &self.lastChar);
        //println!("{}", debug);
    }
}