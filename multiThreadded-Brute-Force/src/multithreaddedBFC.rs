use std::io;
use std::cmp::Ordering;
use std::env::args;
use unicode_segmentation::UnicodeSegmentation;

// Multithreadded variation of Brute Force Class
// Splits potential list of things to search through into how many threads are available

// Seen as class variables
pub struct MultiBFSearch {
    maxLength: i8,
    realPassword: String, //Immutable string slice
    passGuess: String,
    lastGuess: String,
    unicodeList: Vec<char>,
    numGuesses: u128,
}

// Seen as class functions
impl MultiBFSearch {

    // Constructor that implements default variables
    pub fn new(maxLength: i8, realPassword: String, searchComplexity: char) -> Self {
        let mut tempCharList: Vec<char> = Vec::new();

        // Sets unicode list to iterate over
        match searchComplexity {

            // Basic
            'B'|'b' => {
                for ch in ' '..='~' {
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

            _ => {
                panic!("Invalid searchComplexity character at ")
            }

        }

        // Initalizes and returns BFsearch Struct (no semicolon)
        Self {
            maxLength,
            realPassword,
            passGuess: String::new(),
            lastGuess: String::new(),
            unicodeList: tempCharList,
            numGuesses: 0
        }
    }

    pub fn startSearch(&self) {

        // Figure out lastGuess
        for i in 0..self.maxLength {
            self.lastGuess += self.unicodeList[-1];
        }

        println!("{}", self.lastGuess);
    }

    // Constantly makes this check to see if password matches
    fn __isPasswordMatch(&self) -> bool {
        self.passGuess == self.realPassword
    }

    

    // Debugging
    pub fn printUnicodeListSize (&self) {
        println!("Size of Unicode List to iterate through: {}", self.unicodeList.len());
    }
}