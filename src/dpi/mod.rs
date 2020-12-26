
//! ### Background
//! The practice of implementing a Data Privacy Inspector addresses the following Privacy Design Strategies:
//! - Control
//! - Enforce
//!
//! Explanation goes here ...
//! 
//! Special thanks to [`rs-natural`](https://crates.io/crates/natural) for their work on Phonetics, NGrams, Tokenization, and Tf-ldf.
//!
//! ### Usage
//! 
//! 

extern crate regex;

// use regex::Regex;

type KeyWordList = Vec<String>;
type KeyPatternList = Vec<String>;
type SoundexWord = Vec<char>;

/// The collection of methods that enable a structure to convert text to ngrams
pub trait Phonetic {
    fn add_more_zeros(chars: Vec<char>) -> Vec<char> {
      (0..4).map( |idx| { 
        if idx < chars.len() {
          chars[idx]
        }
        else {
          '0'
        }
      }).collect()
    }

    fn fix_length(mut chars: Vec<char>) -> Vec<char> {
      match chars.len() {
        4 => chars,
        0...3 => Self::add_more_zeros(chars),
        _ => { chars.truncate(4); chars} //truncate doesn't return self?
      }
    }

    /// Converts a char to a digital char
    /// 
    /// # Arguments
    /// 
    /// * c: char - The character to covnert to a digital char.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::Phonetic;
    ///
    /// fn main() {
    ///   struct Prcsr;
    ///   impl Phonetic for Prcsr {}
    ///   
    ///   assert_eq!(Prcsr::get_char_digit('p'),'1');
    ///   assert_eq!(Prcsr::get_char_digit('g'),'2');
    ///   assert_eq!(Prcsr::get_char_digit('d'),'3');
    ///   assert_eq!(Prcsr::get_char_digit('n'),'5');
    ///   assert_eq!(Prcsr::get_char_digit('r'),'6');
    ///   assert_eq!(Prcsr::get_char_digit('w'),'9');
    ///   assert_eq!(Prcsr::get_char_digit('e'),'0');
    /// }
    /// ```
    fn get_char_digit(c: char) -> char {
      match c {
        'b' | 'f' | 'p' | 'v' => '1',
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => '2',
        'd' | 't' => '3',
        'l' => '4',
        'm' | 'n' => '5',
        'r' => '6',
        'h' | 'w' => '9', //0 and 9 are removed later, this is just to separate vowels from h and w
        _ => '0' //Vowels
      }
    }

    fn soundex_encoding(chars: Vec<char>) -> SoundexWord {
      Self::fix_length(Self::strip_similar_chars(chars))
    }

    fn soundex_word(word: &str) -> SoundexWord {
      let mut chars: Vec<char> = Vec::new();
    
      for c in word.chars() {
        chars.push(c);
      }

      chars = Self::soundex_encoding(chars);

      chars
    }

    fn sounds_like(word1: &str, word2: &str) -> bool {
      Self::soundex_word(word1) == Self::soundex_word(word2)
    }

    fn strip_similar_chars(chars: Vec<char>) -> Vec<char> {
      let mut enc_chars = Vec::new();
      enc_chars.push(chars[0]);
      for i in 1..chars.len() {
        enc_chars.push(Self::get_char_digit(chars[i]));
      }
      let mut chars_no_hw = Vec::new();
      let mut chars_no_vowels = Vec::new();
      for c in enc_chars.into_iter() {
        if c != '9' {
          chars_no_hw.push(c);
        }
      }
      chars_no_hw.dedup();
      for c in chars_no_hw.into_iter() {
        if c != '0' {
          chars_no_vowels.push(c);
        }
      }
      chars_no_vowels
    }
}

/// The collection of methods that enable a structure to tokenize and convert text to ngrams
pub trait Tokenizer {
    /// Creates the NGram
    /// 
    /// # Arguments
    /// 
    /// * text: &'a str - The textual content to split into grams.</br>
    /// * n: usize - The number of gram in a split.</br>
    /// * pad: &'a str - The string to use as padding at the beginning and end of the ngrams.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::Tokenizer;
    ///
    /// fn main() {
    ///   struct Prcsr;
    ///   impl Tokenizer for Prcsr {}
    ///
    ///   assert_eq!(
    ///     Prcsr::ngram("This is my private data", 2, "----"), 
    ///     vec![["----", "This"], ["This", "is"], ["is", "my"], ["my", "private"], ["private", "data"], ["data", "----"]]
    ///   );
    /// }
    /// ```
    fn ngram<'a>(text: &'a str, n: usize, pad: &'a str) -> Vec<Vec<&'a str>> {
      let mut tokenized_sequence = Self::tokenize(text);
      tokenized_sequence.shrink_to_fit();
      
      let count = tokenized_sequence.len() - n + 1;
      
      let mut ngram_result = Vec::new();
      
      //left-padding
      if !pad.is_empty() {
        for i in 1..n {
          let num_blanks = n - i;
          let mut this_sequence = Vec::new();
          for _ in 0..num_blanks {
            this_sequence.push(pad);
          }
          let sl = &tokenized_sequence[0 .. (n - num_blanks)];
          this_sequence.extend_from_slice(sl);
          ngram_result.push(this_sequence);
        }
      }
      
      //Fill the rest of the ngram
      for i in 0..count {
        let a = &tokenized_sequence[i..i + n];
        let sl = a.to_vec();
        ngram_result.push(sl);
      }
      
      //right-padding
      if !pad.is_empty() {
        for num_blanks in 1..n {
          let num_tokens = n - num_blanks;
          let last_entry = tokenized_sequence.len();
          let mut tc = Vec::new();
          tc.extend_from_slice(&tokenized_sequence[(last_entry - num_tokens) .. last_entry]);
          for _ in 0..num_blanks {
            tc.push(pad);
          }
          ngram_result.push(tc);
        }
      }
      ngram_result
    }

    /// Splits text into a list of words
    /// 
    /// # Arguments
    /// 
    /// * text: &str - A textual string to be split apart into separate words.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::Tokenizer;
    ///
    /// fn main() {
    ///     struct Tknzr;
    ///     impl Tokenizer for Tknzr {}
    ///     
    ///     assert_eq!(Tknzr::tokenize("My personal data"), vec!["My","personal","data"]);
    /// }
    /// ```
    fn tokenize(text: &str) -> Vec<&str> {
        text.split(Self::is_match)
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Indicates if a char is one of the predefined delimiters that is used to spearate words
    /// 
    /// # Arguments
    /// 
    /// * c: char - A character to be checked.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::Tokenizer;
    ///
    /// fn main() {
    ///     struct Tknzr;
    ///     impl Tokenizer for Tknzr {}
    ///     
    ///     assert_eq!(Tknzr::is_match(' '), true);
    /// }
    /// ```
    fn is_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
            | ':' | '\t' | '\n' | '(' | ')' | '{' | '}' => true,
            _ => false
        }
    }
}

/// Represents a Data Provacy Inspector (DPI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DPI {
    pub key_words: Option<KeyWordList>,
    pub key_patterns: Option<KeyPatternList>,
}

impl Tokenizer for DPI {}

impl DPI {
    /// Constructs a DPI object
    /// 
    /// # Arguments
    /// 
    /// * words: Option<KeyWordList> - A vector of words that are known identifiers for private data.</br>
    /// * patterns: Option<KeyPatternList> - A vector of Regex patterns that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let words = Some(vec!["ssn".to_string()]);
    ///     let patterns = Some(vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()]);
    ///     let dpi = DPI::new(words, patterns);
    ///     
    ///     println!("Using {} words and {} patterns for learning.", dpi.key_words.unwrap().len(), dpi.key_patterns.unwrap().len());
    /// }
    /// ```
    pub fn new(words: Option<KeyWordList>, patterns: Option<KeyWordList>) -> DPI {
        DPI {
            key_words: words,
            key_patterns: patterns,
        }
    }

    /// Constructs a DPI object from a serialized string
    /// 
    /// # Arguments
    /// 
    /// * serialized: &str - The string that represents the serialized object.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"]}"#;
    ///     let dpi = DPI::from_serialized(&serialized);
    ///     
    ///     println!("{:?}", dpi);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DPI {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Serialize a DPI object
    /// 
    /// # Arguments
    /// 
    /// * serialized: &str - The string that represents the serialized object.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let mut dpi = DPI {
    ///         key_words: Some(vec!["ssn".to_string()]),
    ///         key_patterns: Some(vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()]),
    ///     };
    ///     
    ///     println!("{:?}", dpi.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
		serde_json::to_string(&self).unwrap()
    }
}

pub mod error;
//pub mod extractor;
//pub mod middleware;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn get_dpi() -> Vec<DPI>{
        let mut v = Vec::new();
        v.push( DPI {
                    key_words: Some(vec!["ssn".to_string()]),
                    key_patterns: Some(vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()]),
                });
        v
    }

    #[test]
    fn test_ngram_calculate() {
        struct Prcsr;
        impl Tokenizer for Prcsr {}

        assert_eq!(
          Prcsr::ngram("This is my private data", 2, "----"), 
          vec![["----", "This"], ["This", "is"], ["is", "my"], ["my", "private"], ["private", "data"], ["data", "----"]]
        );
    }

    #[test]
    fn test_phonetic_char_digit() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(Prcsr::get_char_digit('p'),'1');
        assert_eq!(Prcsr::get_char_digit('g'),'2');
        assert_eq!(Prcsr::get_char_digit('d'),'3');
        assert_eq!(Prcsr::get_char_digit('n'),'5');
        assert_eq!(Prcsr::get_char_digit('r'),'6');
        assert_eq!(Prcsr::get_char_digit('w'),'9');
        assert_eq!(Prcsr::get_char_digit('e'),'0');
    }    

    #[test]
    fn test_phonetic_sounds_like() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert!(Prcsr::sounds_like("rupert","robert"));
    }   

    #[test]
    fn test_tokenizer_tokenize() {
        struct Prcsr;
        impl Tokenizer for Prcsr {}

        assert_eq!(Prcsr::tokenize("My personal data"), vec!["My","personal","data"]);
        assert_eq!(Prcsr::tokenize(r#"{"ssn":"003-08-5546"}"#), vec!["ssn","003-08-5546"]);
    }

    #[test]
    fn test_dpi_from_serialized_ok() {
        let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"]}"#;
        let dpi = DPI::from_serialized(serialized);

        assert_eq!(dpi.key_words.unwrap().len(), 1);
        assert_eq!(dpi.key_patterns.unwrap().len(), 1);
    }

    #[test]
    fn test_dpi_serialize_ok() {
        let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"]}"#;
        let dpi = &mut get_dpi()[0];

        assert_eq!(dpi.serialize(), serialized);
    }
}