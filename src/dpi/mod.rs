
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

extern crate eddie;
extern crate regex;

use crate::dpi::error::*;
use std::collections::BTreeMap;
use regex::Regex;
use rayon::prelude::*;


const KEY_PATTERN_PNTS: f64 = 80 as f64;
const KEY_WORD_PNTS: f64 = 100 as f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScoreKey {
  KeyPattern = 10,
  KeyWord = 20,
}

type KeyWordList = Vec<String>;
type KeyPatternList = Vec<String>;
type SoundexWord = Vec<char>;
type PatternMap = BTreeMap<String, char>;
type ScoreCard = BTreeMap<String, Score>;

/// The collection of methods that enable a structure to convert text to ngrams
pub trait Phonetic {
  /// Pads the vector of chars with zeros if length is less than 4
  /// 
  /// # Arguments
  /// 
  /// * chars: Vec<char> - The vector of chars to pad.</br>
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert_eq!(Prcsr::add_more_zeros(vec!['h','4','0']), vec!['h','4','0','0']);
  /// }
  /// ```
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

  /// Ensures that the vector of chars is only 4 chars in length
  /// 
  /// # Arguments
  /// 
  /// * chars: Vec<char> - The vector of chars to size.</br>
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert_eq!(Prcsr::fix_length(vec!['h','4','0']).len(), 4);
  /// }
  /// ```
  fn fix_length(mut chars: Vec<char>) -> Vec<char> {
    match chars.len() {
      4 => chars,
      0..=3 => Self::add_more_zeros(chars),
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
  /// ```rust
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

  /// Converts a vector of chars to a SoundexWord type
  /// 
  /// # Arguments
  /// 
  /// * chars: Vec<char> - The vector of chars to convert.</br>
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert_eq!(Prcsr::soundex_encoding(vec!['h','e','l','l','o']),vec!['h', '4', '0', '0']);
  /// }
  /// ```
  fn soundex_encoding(chars: Vec<char>) -> SoundexWord {
    Self::fix_length(Self::strip_similar_chars(chars))
  }

  /// Converts a word to a SoundexWord type
  /// 
  /// # Arguments
  /// 
  /// * word: &str - The word to convert.</br>    
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert_eq!(Prcsr::soundex_word("hello"), vec!['h', '4', '0', '0']);
  /// }
  /// ```
  fn soundex_word(word: &str) -> SoundexWord {
    let mut chars: Vec<char> = Vec::new();
  
    for c in word.chars() {
      chars.push(c);
    }

    chars = Self::soundex_encoding(chars);

    chars
  }

  /// Compares 2 words and determines if they sound similar (true=yes, false=no)
  /// 
  /// # Arguments
  /// 
  /// * word1: &str - The first textual word to compare to the second.</br>    
  /// * word2: &str - The second textual word to compare to the first.</br>
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert!(Prcsr::sounds_like("rupert","robert"));
  /// }
  /// ```
  fn sounds_like(word1: &str, word2: &str) -> bool {
    Self::soundex_word(word1) == Self::soundex_word(word2)
  }

  /// Removes duplicate chars that share the same char digits
  /// 
  /// # Arguments
  /// 
  /// * chars: Vec<char> - The vector of char digits.</br>    
  /// 
  /// #Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::Phonetic;
  ///
  /// fn main() {
  ///   struct Prcsr;
  ///   impl Phonetic for Prcsr {}
  ///   
  ///   assert_eq!(Prcsr::strip_similar_chars(vec!['h', 'e', 'l', 'l', 'o']), vec!['h', '4']);
  /// }
  /// ```
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
  /// ```rust
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
  /// ```rust
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
  /// ```rust
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

/// Represents a symbolic pattern of an entity (String)
pub struct Pattern {
	/// The regex rule used to find upper case consonants
	regex_consonant_upper: Regex,
	/// The regex rule used to find lower case consonants
	regex_consonant_lower: Regex,
	/// The regex rule used to find upper case vowels
	regex_vowel_upper: Regex,
	/// The regex rule used to find lower case vowels
	regex_vowel_lower: Regex,
	/// The regex rule used to find numeric digits
	regex_numeric: Regex,
	/// The regex rule used to find punctuation
	regex_punctuation: Regex,
	/// The regex rule used to find white spaces
	regex_space: Regex,
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            regex_consonant_upper: Regex::new(r"[B-DF-HJ-NP-TV-Z]").unwrap(),
            regex_consonant_lower: Regex::new(r"[b-df-hj-np-tv-z]").unwrap(),
            regex_vowel_upper: Regex::new(r"[A|E|I|O|U]").unwrap(),
            regex_vowel_lower: Regex::new(r"[a|e|i|o|u]").unwrap(),
            regex_numeric: Regex::new(r"[0-9]").unwrap(),
            regex_punctuation: Regex::new(r"[.,\\/#!$%\\^&\\*;:{}=\\-_`~()\\?]").unwrap(),
            regex_space: Regex::new(r"[\s]").unwrap(),
        }
    }
}

/// Represents the object managing all the symbols used in pattern definitions
pub struct PatternDefinition {
  pattern_map: PatternMap,
  pattern: Pattern,
}

impl PatternDefinition {
  /// Constructs a new PatternDefinition
  /// 
  /// # Example
  /// 
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::PatternDefinition;
  ///	
  /// fn main() {
  /// 	let pttrn_def = PatternDefinition::new();
  /// }
  /// ```
  pub fn new() -> PatternDefinition {	
    let symbols: [char; 9] = ['@','C','c','V','v','#','~','S','p'];
    let mut pttrn_def = PatternMap::new();

    pttrn_def.insert("Unknown".to_string(),        symbols[0]);
    pttrn_def.insert("ConsonantUpper".to_string(), symbols[1]);
    pttrn_def.insert("ConsonantLower".to_string(), symbols[2]);
    pttrn_def.insert("VowelUpper".to_string(),     symbols[3]);
    pttrn_def.insert("VowelLower".to_string(),     symbols[4]);
    pttrn_def.insert("Numeric".to_string(),        symbols[5]);
    pttrn_def.insert("RegExSpcChar".to_string(),   symbols[6]);
    pttrn_def.insert("WhiteSpace".to_string(),     symbols[7]);
    pttrn_def.insert("Punctuation".to_string(),    symbols[8]);

    PatternDefinition{
      pattern_map: pttrn_def,
      pattern: Pattern::default(),
    }
  }	

  /// This function converts an entity (&str) into a tuplet (String, Vec<Fact>)</br>
  ///
  /// # Arguments
  ///
  /// * `entity: String` - The textual str of the value to anaylze.</br>
  ///
  /// # Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::PatternDefinition;
  ///
  /// fn main() {
  ///		let mut pttrn_def = PatternDefinition::new();
  ///     let rslt = pttrn_def.analyze("Hello World");
  ///     assert_eq!(rslt, "CvccvSCvccc");
  /// }
  /// ```
  pub fn analyze(&mut self, entity: &str) -> String {
    let mut pttrn = String::new();

    for c in entity.chars() {
      pttrn.push(self.symbolize_char(c));
    }

    pttrn
  }
  
  /// This function returns a pattern symbol that represents the type of character 
  /// 
  /// # Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::PatternDefinition;
  ///	
  /// fn main() {
  /// 	let pttrn_def = PatternDefinition::new();
  ///     println!("Upper case vowel symbol: {:?}", pttrn_def.get(&"VowelUpper".to_string()));
  /// }
  /// ```
  pub fn get(&self, key: &str) -> char {
    *self.pattern_map.get(key).unwrap()
  }
  
  /// This function converts a char into a pattern symbol
  ///
  /// # Example
  ///
  /// ```rust
  /// extern crate pbd;
  ///
  /// use pbd::dpi::PatternDefinition;
  ///
  /// fn main() {
  /// 	let pttrn_def = PatternDefinition::new();
  /// 	println!("The pattern symbol for 'A' is {:?}", pttrn_def.symbolize_char('A'));
  ///     // The pattern symbol for 'A' is V
  /// }
  /// ```
  pub fn symbolize_char(&self, c: char) -> char {
      // if you have to escape regex special characters: &*regex::escape(&*$c.to_string())
    let mut symbol = self.pattern_map.get("Unknown");
      let mut found = false;
      
      if !found && self.pattern.regex_consonant_upper.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("ConsonantUpper");
          found = true;
      }

      if !found && self.pattern.regex_consonant_lower.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("ConsonantLower");
          found = true;
      }

      if !found && self.pattern.regex_vowel_upper.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("VowelUpper");
          found = true;
      }

      if !found && self.pattern.regex_vowel_lower.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("VowelLower");
          found = true;
      }

      if !found && self.pattern.regex_numeric.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("Numeric");
          found = true;
      }

      if !found && self.pattern.regex_space.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("WhiteSpace");
          found = true;
      }

      if !found && self.pattern.regex_punctuation.is_match(&c.to_string()) {
          symbol = self.pattern_map.get("Punctuation");
          found = true;
      }

      // if not matched, then use "Unknown" placeholder symbol
      if !found {
          symbol = self.pattern_map.get("Unknown");
      }

      *symbol.unwrap()
  }
}

/// Represents a Score
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    pub key_type: ScoreKey,
    pub key_value: String,
    pub points: f64, 
}

impl Score {
  pub fn new(ktype: ScoreKey, kvalue: String, pnts: f64) -> Score {
    Score {
      key_type: ktype,
      key_value: kvalue,
      points: pnts, 
    }
  }
}

/// Represents a Data Privacy Inspector (DPI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DPI {
    pub key_words: Option<KeyWordList>,
    pub key_patterns: Option<KeyPatternList>,
    pub scores: ScoreCard,
}

impl Tokenizer for DPI {}

impl DPI {
    /// Constructs a DPI object without using any predefined set of key words or patterns to learn from
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let dpi = DPI::new();
    /// }
    /// ```
    pub fn new() -> DPI {
      DPI {
          key_words: None,
          key_patterns: None,
          scores: ScoreCard::new(),
      }
    }

    /// Constructs a DPI object using a predefined set of key words and patterns to learn from
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
    ///     let dpi = DPI::with(words, patterns);
    ///     
    ///     println!("Using {} words and {} patterns for learning.", dpi.key_words.unwrap().len(), dpi.key_patterns.unwrap().len());
    /// }
    /// ```
    pub fn with(words: Option<KeyWordList>, patterns: Option<KeyWordList>) -> DPI {
        DPI {
            key_words: words,
            key_patterns: patterns,
            scores: ScoreCard::new(),
        }
    }

    /// Constructs a DPI object using a predefined set of key patterns to learn from
    /// 
    /// # Arguments
    /// 
    /// * patterns: KeyPatternList - A vector of Regex patterns that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let patterns = vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()];
    ///     let dpi = DPI::with_key_patterns(patterns);
    ///     
    ///     println!("Using {} patterns for learning.", dpi.key_patterns.unwrap().len());
    /// }
    /// ```
    pub fn with_key_patterns(patterns: KeyWordList) -> DPI {
      DPI {
          key_words: None,
          key_patterns: Some(patterns),
          scores: ScoreCard::new(),
      }
  }

    /// Constructs a DPI object using a predefined set of key words to learn from
    /// 
    /// # Arguments
    /// 
    /// * words: KeyWordList - A vector of words that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    ///
    /// fn main() {
    ///     let words = vec!["ssn".to_string()];
    ///     let dpi = DPI::with_key_words(words);
    ///     
    ///     println!("Using {} words for learning.", dpi.key_words.unwrap().len());
    /// }
    /// ```
    pub fn with_key_words(words: KeyWordList) -> DPI {
      DPI {
          key_words: Some(words),
          key_patterns: None,
          scores: ScoreCard::new(),
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
    ///     let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"],"scores":{}}"#;
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
    /// use pbd::dpi::{DPI};
    ///
    /// fn main() {
    ///     let mut dpi = DPI::with(
    ///         Some(vec!["ssn".to_string()]),
    ///         Some(vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()])
    ///       );
    ///     
    ///     println!("{:?}", dpi.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
		  serde_json::to_string(&self).unwrap()
    }

    pub fn get_score(&mut self, key: String) -> Score {
      match self.scores.get_mut(&key) {
        Some(s) => s.clone(),
        None => {
          Score::new(ScoreKey::KeyWord, key, 0 as f64).clone()
        },
      }
    }

    pub fn add_to_score_points(&mut self, key: String, pnts: f64) {
      let mut score = self.get_score(key);
      score.points += pnts;
      &self.upsert_score(score);
    }

    pub fn train_for_key_words(&mut self, tokens: Vec<&str>) {
      let kwords = self.key_words.clone();
      
      let list: Vec<&&str> = tokens.par_iter()
        .filter(|t| {
          kwords.as_ref().unwrap().iter().any(|w| w.to_lowercase() == t.to_lowercase())
        }).collect();

      list.iter()
        .for_each(|t| {
          self.add_to_score_points(t.to_string(), KEY_WORD_PNTS);
        });
    }

    pub fn train_from_keys(&mut self, text: String) {
      let tokens = Self::tokenize(&text);
      self.train_for_key_words(tokens);
    }

    pub fn upsert_score(&mut self, score: Score) {
      self.scores.insert(score.key_value.clone(), score);
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
                    scores: ScoreCard::new(),
                });
        v
    }

    fn get_text() -> String {
      String::from(r#"Here is my ssn that you requested: 003-75-9876."#)
    }

    #[test]
    fn test_dpi_new() {
      let dpi = DPI::new();

      assert!(dpi.key_words.is_none());
      assert!(dpi.key_patterns.is_none());
    }    

    #[test]
    fn test_dpi_with() {
      let words = Some(vec!["ssn".to_string()]);
      let patterns = Some(vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()]);
      let dpi = DPI::with(words, patterns);

      assert_eq!(dpi.key_words.unwrap().len(),1);
    }

    #[test]
    fn test_dpi_with_keypatterns() {
      let patterns = vec!["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$".to_string()];
      let dpi = DPI::with_key_patterns(patterns);
      
      assert_eq!(dpi.key_patterns.unwrap().len(),1);
    }

    #[test]
    fn test_dpi_with_keywords() {
      let words = vec!["ssn".to_string()];
      let dpi = DPI::with_key_words(words);
      
      assert_eq!(dpi.key_words.unwrap().len(),1);
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
    fn test_pattern_analyze() {
      let mut pttrn_def = PatternDefinition::new();
      let rslt = pttrn_def.analyze("Hello World");

      assert_eq!(rslt, "CvccvSCvccc");
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
    fn test_phonetic_fixed_length() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(Prcsr::fix_length(vec!['h','4','0']).len(), 4);
    }    
    
    #[test]
    fn test_phonetic_pad_zeros() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(Prcsr::add_more_zeros(vec!['h','4','0']), vec!['h','4','0','0']);
    }  

    #[test]
    fn test_phonetics_remove_similar_char_digits() {
        struct Prcsr;
        impl Phonetic for Prcsr {}
       
        assert_eq!(Prcsr::strip_similar_chars(vec!['h', 'e', 'l', 'l', 'o']), vec!['h', '4']);
    }    

    #[test]
    fn test_phonetics_soundex_encode() {
        struct Prcsr;
        impl Phonetic for Prcsr {}
       
        assert_eq!(Prcsr::soundex_encoding(vec!['h','e','l','l','o']),vec!['h', '4', '0', '0']);
    }

    #[test]
    fn test_phonetic_sounds_like() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert!(Prcsr::sounds_like("rupert","robert"));
        assert!(Prcsr::sounds_like("social","sozial"));
    }   

    #[test]
    fn test_phonetics_soundex_word() {
        struct Prcsr;
        impl Phonetic for Prcsr {}
       
        assert_eq!(Prcsr::soundex_word("hello"), vec!['h', '4', '0', '0']);
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
        let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"],"scores":{}}"#;
        let dpi = DPI::from_serialized(serialized);

        assert_eq!(dpi.key_words.unwrap().len(), 1);
        assert_eq!(dpi.key_patterns.unwrap().len(), 1);
    }

    #[test]
    fn test_dpi_serialize_ok() {
        let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"],"scores":{}}"#;
        let dpi = &mut get_dpi()[0];

        assert_eq!(dpi.serialize(), serialized);
    }

    #[test]
    fn test_dpi_train_for_key_words() {
      let serialized = r#"{"key_words":["ssn"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"],"scores":{"ssn":{"key_type":"KeyWord","key_value":"ssn","points":100.0}}}"#;
      let dpi = &mut get_dpi()[0];
      
      dpi.train_from_keys(get_text());

      assert_eq!(dpi.serialize(), serialized);
    }
}