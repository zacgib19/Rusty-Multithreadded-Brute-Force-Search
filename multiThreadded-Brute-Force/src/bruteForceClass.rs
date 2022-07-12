use std::io;
use std::cmp::Ordering;
use std::env::args;
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;


// Seen as class variables
pub struct BFSearch {
    maxLength: i8,
    realPassword: String,      //Immutable string slice
    passGuess: String,
    lastGuess: String,
    charMap: HashMap<i32, char>,
    numGuesses: u128,
    currentIndex: usize,       //Index for string array in binary search algorithm
    lastChar: String,
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(maxLength: i8, realPassword: String, searchComplexity: char) -> Self {
        let mut tempCharMap = HashMap::new();
        let mut tempCharList: Vec<char> = Vec::new(); //Temporary char array used for charMap
        let mut tempChar: char = ' '; //Temporary character used for lastChar
        
        // Sets unicode list to iterate over
        match searchComplexity {

            // Basic ASCII
            'B'|'b' => {
                //DEBUGGING should be from ' ' to '~'
                for ch in 'A'..='F' {
                    tempCharList.push(ch);
                }           
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
            },

            // Full
            'F'|'f' => {
                // From null character throughout the entire unicode library
                for ch in ''..='𫠝' {
                    tempCharList.push(ch);
                }
            }

            // Crash program if anything else
            _ => {
                panic!("Invalid searchComplexity character passed in")
            }

        }

        for i in 0..tempCharList.len() {
            tempCharMap.insert(i, tempCharList[i]);
        } 

        tempChar = tempCharList[tempCharList.len()-1];
        
        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            maxLength,
            realPassword,
            passGuess: String::new(),
            lastGuess: String::new(),
            charMap: tempCharMap,
            numGuesses: 0,
            currentIndex: 0,
            lastChar: String::from(tempChar),
        }
    }

   
    // Starts brute force search
    pub fn startSearch (&mut self) {   
        self.getLastGuess();

        
        while !(self.isLastGuess() && self.isPasswordMatch()) {
            self.numGuesses += 1;
            if self.isPasswordMatch() {
                break;
            }

            else {
                self.currentIndex = 0;
                self.passGuess = self.str_next();
            }

            //Debugging
            // print!("{}, ", self.passGuess);
        }
        
    }

    // Sets lastGuess to maxLength copies of the last character in charMap
    fn getLastGuess (&mut self) {
        // Figure out lastGuess
        for i in 0..self.maxLength {
            self.lastGuess.push_str(&self.lastChar);
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

    // Updates passGuess in binary search fashion
    fn str_next(&mut self) -> String {
        
        // For very first guess
        if self.passGuess.graphemes(true).count() == 0 {
            // Add first character
            self.passGuess.push(self.charMap.get(&0).unwrap());
            return self.passGuess.clone();
        }

        // For every other guess
        else {
            // New instance of this supposed to run each recursion of str_next()
            // Vector iterator something
            let mut currCharIndex = self.charMap.entry(self.passGuess
                                                .graphemes(true)
                                                .nth(self.currentIndex)
                                                .unwrap()
                                                );                        // Dont initiallize to 0!

            
            match self.charMap.get(&currCharIndex) {
                // If char at index is not the last character in self.charMap
                Some(&_) {
                    println!("{}", currCharIndex);

                currCharIndex += 1;
                
                // Change passGuess' character at currentIndex position
                self.passGuess.pop();
                self.passGuess.push(self.charMap.get(currCharIndex));
                return self.passGuess.clone();
                }
            }

            // If last character in self.charMap
            else {
                // If only character in self.passGuess
                if self.passGuess.graphemes(true).count() == 1 {
                    // Reset first character
                    self.passGuess.remove(0);
                    self.passGuess.insert(0, self.charMap.get(&0));
                    // Add second character
                    self.passGuess.push(self.charMap.get(&0));


                    // Reset currCharIndex
                    currCharIndex = self.charMap.get(&0);
                    

                    return self.passGuess.clone();
                }

                // Else if time to add another letter
                else if (self.passGuess.graphemes(true).count() == (self.currentIndex + 1)) {
                    // Replace character at index with first character of charMap
                    self.passGuess.remove(self.currentIndex);
                    self.passGuess.insert(self.currentIndex, self.charMap[0]);
                    // Append first character of charMap to passGuess
                    self.passGuess.push(self.charMap[0]);
                    return self.passGuess.clone();
                }

                // If last possible string to check
                else if self.isLastGuess() {
                    // Do nothing and return
                    return self.passGuess.clone();
                }

                else {
                    self.currentIndex += 1;
                    let mut returnString = self.str_next();
                    self.currentIndex -= 1;
                    // Replace character at currentIndex with first char in charMap
                    returnString.remove(self.currentIndex);
                    returnString.insert(self.currentIndex, self.charMap[0]);

                    return returnString;
                }
            }
        };
    }

    // Debugging
    pub fn debugging(&self) {
        
    }
    
}