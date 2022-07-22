use unicode_segmentation::UnicodeSegmentation;
use std::char;
use std::collections::HashMap;


// Seen as class variables
pub struct BFSearch {
    maxLength: i8,
    realPassword: String,      //Immutable string slice
    pub passGuess: String,
    lastGuess: String,
    charFromIntMap: HashMap<i32, String>,
    intFromCharMap: HashMap<String, i32>,
    pub numGuesses: u128,
    currentIndex: usize,       //Index for string array in binary search algorithm
    firstChar: char,           
    lastChar: String,
    pub isFound: bool,
}

// Seen as class functions
impl BFSearch {

    // Constructor that implements default variables
    pub fn new(maxLength: i8, realPassword: String, searchComplexity: char) -> Self {
        let mut tempCharMap = HashMap::new();
        let mut tempIntMap = HashMap::new();
        let mut tempCharList: Vec<char> = Vec::new(); //Temporary char array used for charFromIntMap
        let mut tempFChar: char = ' '; //Temporary character used for firstChar
        let mut tempLChar: char = ' '; //Temporary character used for lastChar
        
        // Sets unicode list to iterate over
        match searchComplexity {

            // Basic ASCII
            'B'|'b' => {
                //DEBUGGING should be from ' ' to '~'
                for ch in ' '..='~' {
                    tempCharList.push(ch);
                }           
            },

            // Extended
            'E'|'e' => {

                //Takes two bytes to store these
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

        // Convert vector to hashmap
        for i in 0..tempCharList.len() {
            tempCharMap.insert(i as i32, String::from(tempCharList[i]));  //used for charToInt
            tempIntMap.insert(String::from(tempCharList[i]), i as i32);   //used for intToChar
        } 
       

        // Gets first and last character
        tempFChar = tempCharList[0];
        tempLChar = tempCharList[tempCharList.len()-1];
        
        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            maxLength,
            realPassword,
            passGuess: String::new(),
            lastGuess: String::new(),
            charFromIntMap: tempCharMap,
            intFromCharMap: tempIntMap,
            numGuesses: 0,
            currentIndex: 0,
            firstChar: tempFChar,
            lastChar: String::from(tempLChar),  
            isFound: false,
        }
    }

   
    // Starts brute force search
    pub fn startSearch (&mut self) {   
        self.getLastGuess();
        
        while !(self.isLastGuess() && self.isPasswordMatch()) {
            
            if self.isPasswordMatch() {
                self.isFound = true;
                break;
            }

            else if self.isLastGuess() {
                break;
            }

            else {
                self.currentIndex = 0;
                self.passGuess = self.str_next();
            }

            self.numGuesses += 1;
            //Debugging
            println!("{},", self.passGuess.chars().count());
        }    
    }

    // Sets lastGuess to maxLength copies of the last character in charFromIntMap
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
        if self.numGuesses == 0 {
            // Add first character
            self.passGuess.push(self.firstChar);
            return self.passGuess.clone();
        }

        // For every other guess
        else {
            // New instance of this supposed to run each recursion of str_next()
            //BUG: Extended and Full search crash here at last character

            // USE UNICODE CODEPOINT TO GET NEXT CHARACTER
                        
            // If char at index is not the last character in self.charFromIntMap
            if self.passGuess.graphemes(true).nth(self.currentIndex).unwrap() != self.lastChar {
                let mut tempChar = self.passGuess.chars().nth(self.currentIndex).unwrap() as char;

                let mut tempInt = tempChar as u32;

                // Skip compiler checks for valid unicode characters
                unsafe {
                    tempInt += 1;
                    
                    // Change passGuess' character at currentIndex position
                    self.passGuess.remove(self.currentIndex);
                    self.passGuess.insert(self.currentIndex, char::from_u32(tempInt).unwrap());
                    
                }
                //println!("{}", self.passGuess);
                return self.passGuess.clone();
            }

            // If char at index is last 
            else {   
                
                //DEBUGGING
                //println!("Only character in passGuess? {}", self.passGuess.graphemes(true).count() == 1);
                //println!("Is it time to add letter? {}", self.passGuess.graphemes(true).count() == (self.currentIndex + 1));

                // If only character in self.passGuess
                if self.passGuess.graphemes(true).count() == 1 {
                    
                    // Reset first character
                    self.passGuess.remove(0);
                    self.passGuess.insert(0, self.firstChar);
                    // Add second character
                    self.passGuess.push(self.firstChar);

                    // Reset currCharMapIndex
                    let currCharMapIndex = 0;
                    return self.passGuess.clone();
                }


                // Else if time to add another letter
                else if self.passGuess.graphemes(true).count() == (self.currentIndex + 1) {
                    // Replace character at index with first character of charFromIntMap
                    self.passGuess.remove(self.currentIndex);
                    self.passGuess.insert(self.currentIndex, self.firstChar);
                    // Append first character of charFromIntMap to passGuess
                    self.passGuess.push(self.firstChar);
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
                    // Replace character at currentIndex with first char in charFromIntMap
                    returnString.remove(self.currentIndex);
                    returnString.insert(self.currentIndex, self.firstChar);

                    return returnString;
                }
            };
        }
    }   
}