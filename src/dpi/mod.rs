
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

/*
** LOGIC
** 1. Words that appear infrequently across multiple documents but frequently in a few documents are relevant (TF-IDF)
**    (https://crates.io/crates/rust-tfidf)
**    Use map reduce to multi-process the tokens for frequency counts.
** 2. Patterns of words that appear within NGram of key words are relevant
** 3. Words that are simalar (Sounds like or Levenstein distince) are slightly relevant
*/

extern crate eddie;
extern crate regex;

use super::*;
use std::collections::{BTreeMap};
use regex::Regex;
use rayon::prelude::*;
use multimap::MultiMap;
use std::cmp::Ordering;
use tfidf::{TfIdf, TfIdfDefault};

const KEY_PATTERN_PNTS: f64 = 80 as f64;
const KEY_REGEX_PNTS: f64 = 90 as f64;
const KEY_WORD_PNTS: f64 = 100 as f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScoreKey {
  KeyPattern = 10,
  KeyWord = 20,
}

type KeyPatternList = Vec<String>;
type KeyRegexList = Vec<String>;
type KeyWordList = Vec<String>;
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
  /// use pbd::dpi::Phonetic;
  ///   
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::add_more_zeros(vec!['h','4','0']), vec!['h','4','0','0']);
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::fix_length(vec!['h','4','0']).len(), 4);
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::get_char_digit('p'),'1');
  /// assert_eq!(Prcsr::get_char_digit('g'),'2');
  /// assert_eq!(Prcsr::get_char_digit('d'),'3');
  /// assert_eq!(Prcsr::get_char_digit('n'),'5');
  /// assert_eq!(Prcsr::get_char_digit('r'),'6');
  /// assert_eq!(Prcsr::get_char_digit('w'),'9');
  /// assert_eq!(Prcsr::get_char_digit('e'),'0');
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::soundex_encoding(vec!['h','e','l','l','o']),vec!['h', '4', '0', '0']);
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::soundex_word("hello"), vec!['h', '4', '0', '0']);
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert!(Prcsr::sounds_like("rupert","robert"));
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
  /// use pbd::dpi::Phonetic;
  ///
  /// struct Prcsr;
  /// impl Phonetic for Prcsr {}
  ///   
  /// assert_eq!(Prcsr::strip_similar_chars(vec!['h', 'e', 'l', 'l', 'o']), vec!['h', '4']);
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

pub trait Tfidf {

  // determine how important a term is in a document compared to other documents
  fn tfidf(term: &str, doc_idx: usize, docs: Vec<Vec<(&str, usize)>>) -> f64{
    TfIdfDefault::tfidf(term, &docs[doc_idx], docs.iter())
  }

  fn frequency_counts_as_vec(tokens: Vec<&str>) -> Vec<(&str, usize)>{
    let mut counts: Vec<(&str, usize)> = Vec::new();

    // MapReduce
    // Map input collection.
    let mapped: Vec<_> = 
        tokens.into_par_iter()
        //.map(|s| s.chars()
        //    //.filter(|c| c.is_alphabetic()).collect::<String>())
        //    .collect::<String>())
        .map(|s| (s, ()))
        .collect();

    // Group by key.
    let shuffled = mapped.into_iter().collect::<MultiMap<_,_>>()
            .into_iter().collect::<Vec<_>>();
    // Reduce by key.
    let mut reduced: Vec<_> = shuffled.into_par_iter()
        .map(|kv| (kv.0, kv.1.len())) // Only using count of values
        .collect();

    // Post processing descending sort
    reduced.sort_by(|a,b| match a.1.cmp(&b.1).reverse() {
        Ordering::Equal => a.0.cmp(&b.0),
        other_ordering => other_ordering
    });

    // Collect results
    for (word, count) in reduced.into_iter() {
        counts.push((word, count));
    }

    counts
  }

  fn frequency_counts(tokens: Vec<&str>) -> BTreeMap<&str, usize>{
    let counts: Vec<(&str, usize)> = Self::frequency_counts_as_vec(tokens);

    // Convert to BTreeMap
    let mut list = BTreeMap::new();
    for count in counts.iter() {
      list.insert(count.0.clone(), count.1);
    }

    list
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
  /// use pbd::dpi::Tokenizer;
  ///
  /// struct Prcsr;
  /// impl Tokenizer for Prcsr {}
  ///
  /// assert_eq!(
  ///   Prcsr::ngram("This is my private data", 2, "----"), 
  ///   vec![["----", "This"], ["This", "is"], ["is", "my"], ["my", "private"], ["private", "data"], ["data", "----"]]
  /// );
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
  /// use pbd::dpi::Tokenizer;
  ///
  /// struct Tknzr;
  /// impl Tokenizer for Tknzr {}
  ///     
  /// assert_eq!(Tknzr::tokenize("My personal data"), vec!["My","personal","data"]);
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
  /// use pbd::dpi::Tokenizer;
  ///
  /// struct Tknzr;
  /// impl Tokenizer for Tknzr {}
  ///     
  /// assert_eq!(Tknzr::is_match(' '), true);
  /// ```
  fn is_match(c: char) -> bool {
      match c {
          ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
          | ':' | '\t' | '\n' | '\r' | '(' | ')' | '{' | '}' => true,
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
  /// Constructs a Pattern object with all the default settings
  /// 
  /// #Example
  ///
  /// ```rust
  /// use pbd::dpi::Pattern;
  /// let pattern = Pattern::default();
  /// ```
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
  /// A list of patterns
  pattern_map: PatternMap,
  /// The Pattern object
  pattern: Pattern,
}

impl PatternDefinition {
  /// Constructs a new PatternDefinition
  /// 
  /// # Example
  /// 
  /// ```rust
  /// use pbd::dpi::PatternDefinition;
  ///	let pttrn_def = PatternDefinition::new();
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

  /// This function converts an entity into a pattern String</br>
  ///
  /// # Arguments
  ///
  /// * `entity: String` - The textual str of the value to anaylze.</br>
  ///
  /// # Example
  ///
  /// ```rust
  /// use pbd::dpi::PatternDefinition;
  ///
  /// let mut pttrn_def = PatternDefinition::new();
  /// let rslt = pttrn_def.analyze("Hello World");
  /// 
  /// assert_eq!(rslt, "CvccvSCvccc");
  /// ```
  pub fn analyze(self, entity: &str) -> String {
    let mut pttrn = String::new();

    for c in entity.chars() {
      pttrn.push(self.symbolize_char(c));
    }

    pttrn
  }

  /// This function converts a list of entities into a vector of pattern Strings</br>
  ///
  /// # Arguments
  ///
  /// * `entities: Vec<&str>` - The list of textual str of the value to anaylze.</br>
  ///
  /// # Example
  ///
  /// ```rust
  /// use pbd::dpi::PatternDefinition;
  ///
  /// let entities = vec!["Hello","my","name","is","John","What","is","your","name","A","name","is","a","personal","identifier","Never","share","your","name","My","ssn","is","003-67-0998"];
  /// let mut pttrn_def = PatternDefinition::new();
  /// let rslt = pttrn_def.analyze_entities(entities);
  /// let pttrns = vec!["Cvccv", "cc", "cvcv", "vc", "Cvcc", "Ccvc", "vc", "cvvc", "cvcv", "V", "cvcv", "vc", "v", "cvccvcvc", "vcvccvcvvc", "Cvcvc", "ccvcv", "cvvc", "cvcv", "Cc", "ccc", "vc", "###@##@####"];
  ///   
  /// assert_eq!(rslt, pttrns);
  /// ```
  pub fn analyze_entities(self, entities: Vec<&str>) -> Vec<String> {
    let pttrns: Vec<_> = entities.into_par_iter()
        .map(|e| {
          //self.analyze(t).as_str()
          let mut pttrn = String::new();

          for c in e.chars() {
            pttrn.push(self.symbolize_char(c));
          }
      
          pttrn
        })
        .collect();

    pttrns
  }
  
  /// This function returns a pattern symbol that represents the type of character 
  /// 
  /// # Example
  ///
  /// ```rust
  /// use pbd::dpi::PatternDefinition;
  ///	
  /// let pttrn_def = PatternDefinition::new();
  /// println!("Upper case vowel symbol: {:?}", pttrn_def.get(&"VowelUpper".to_string()));
  /// ```
  pub fn get(&self, key: &str) -> char {
    *self.pattern_map.get(key).unwrap()
  }
  
  /// This function converts a char into a pattern symbol
  ///
  /// # Example
  ///
  /// ```rust
  /// use pbd::dpi::PatternDefinition;
  ///
  /// let pttrn_def = PatternDefinition::new();
  /// println!("The pattern symbol for 'A' is {:?}", pttrn_def.symbolize_char('A'));
  /// // The pattern symbol for 'A' is V
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
  /// An enum value that identifies the type of key used to identify the Score object
  pub key_type: ScoreKey,
  /// The key used to identify the Score object
  pub key_value: String,
  /// The points the key has received as a score of relavence
  pub points: f64, 
}

impl Score {
  /// Constructs a Score object
  /// 
  /// # Arguments
  /// 
  /// * ktype: ScoreKey- The `ScoreKey` enum value that identifies the type of score key, (e.g.: ScoreKey::KeyWord).</br>
  /// * kvalue: String - A key that identifies the score, (e.g.: "dob").</br>
  /// * pnts: f64 - The scored points that the key has received 
  /// 
  /// #Example
  ///
  /// ```rust
  /// use pbd::dpi::{Score, ScoreKey};
  /// let score = Score::new(ScoreKey::KeyWord,"dob".to_string(),25.0);
  /// ```
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
  /// A list of predefined patterns that identify private data
  pub key_patterns: Option<KeyPatternList>,
  /// A list of predefined regular expressions that identify private data
  pub key_regexs: Option<KeyRegexList>,
  /// A list of predefined words that identify private data
  pub key_words: Option<KeyWordList>,
  /// A list of Scores identified by keys
  pub scores: ScoreCard,
}

impl Tokenizer for DPI {}

impl DPI {
    /// Constructs a DPI object without using any predefined set of key words or patterns to learn from
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// let dpi = DPI::new();
    /// ```
    pub fn new() -> DPI {
      let mut dpi = DPI {
          key_patterns: None,
          key_regexs: None,
          key_words: None,
          scores: ScoreCard::new(),
      };
      dpi.init();
      dpi
    }

    /// Constructs a DPI object using a predefined set of key words and patterns to learn from
    /// 
    /// # Arguments
    /// 
    /// * words: Option<KeyWordList> - A vector of words that are known identifiers for private data.</br>
    /// * regexs: Option<KeyRegexList> - A vector of regular expressions that are known identifiers for private data.</br>
    /// * patterns: Option<KeyPatternList> - A vector of patterns that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let words = Some(vec![Lib::TEXT_SSN_ABBR.to_string()]);
    /// let regexs = Some(vec![Lib::REGEX_SSN_DASHES.to_string()]);
    /// let patterns = Some(vec![Lib::PTTRN_SSN_DASHES.to_string()]);
    /// let dpi = DPI::with(words, regexs, patterns);
    ///     
    /// println!("Using {} words and {} patterns for learning.", dpi.key_words.unwrap().len(), dpi.key_patterns.unwrap().len());
    /// ```
    pub fn with(words: Option<KeyWordList>, regexs: Option<KeyWordList>, patterns: Option<KeyWordList>) -> DPI {
      match regexs.clone() {
        Some(reg) =>{
          match Self::validate_regexs(reg) {
            Err(err) => {
              panic!("Bad Regex: {:?}", err);
            },
            _ => {},
          }
        },
        _ => {},
      }     
      
      let mut dpi = DPI {
          key_patterns: patterns,
          key_regexs: regexs,
          key_words: words,
          scores: ScoreCard::new(),
      };
      dpi.init();
      dpi
    }

    /// Constructs a DPI object using a predefined set of key patterns to learn from
    /// 
    /// # Arguments
    /// 
    /// * patterns: KeyPatternList - A vector of patterns that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let patterns = vec![Lib::PTTRN_SSN_DASHES.to_string()];
    /// let dpi = DPI::with_key_patterns(patterns);
    ///     
    /// println!("Using {} patterns for learning.", dpi.key_patterns.unwrap().len());
    /// ```
    pub fn with_key_patterns(patterns: KeyPatternList) -> DPI {
      let mut dpi = DPI {
          key_patterns: Some(patterns),
          key_regexs: None,
          key_words: None,
          scores: ScoreCard::new(),
      };
      dpi.init();
      dpi
    }

    /// Constructs a DPI object using a predefined set of key regular expressions to learn from
    /// 
    /// # Arguments
    /// 
    /// * regexs: KeyRegexList - A vector of Regex patterns that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
    /// let dpi = DPI::with_key_regexs(regexs);
    ///     
    /// println!("Using {} regexs for learning.", dpi.key_regexs.unwrap().len());
    /// ```
    pub fn with_key_regexs(regexs: KeyRegexList) -> DPI {
      match Self::validate_regexs(regexs.clone()) {
        Ok(_) => {},
        Err(err) => {
          panic!("Bad Regex: {:?}", err);
        },
      }

      let mut dpi = DPI {
          key_patterns: None,
          key_regexs: Some(regexs),
          key_words: None,
          scores: ScoreCard::new(),
      };
      dpi.init();
      dpi
    }

    /// Constructs a DPI object using a predefined set of key words to learn from
    /// 
    /// # Arguments
    /// 
    /// * words: KeyWordList - A vector of words that are known identifiers for private data.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let words = vec![Lib::TEXT_SSN_ABBR.to_string()];
    /// let dpi = DPI::with_key_words(words);
    ///     
    /// println!("Using {} words for learning.", dpi.key_words.unwrap().len());
    /// ```
    pub fn with_key_words(words: KeyWordList) -> DPI {
      let mut dpi = DPI {
          key_patterns: None,
          key_regexs: None,
          key_words: Some(words),
          scores: ScoreCard::new(),
      };
      dpi.init();
      dpi
    }

    // Private funciton that initiates the DPI attributes 
    // Call this function from within the constructor functions
    fn init(&mut self) {
      self.init_words();
      self.init_patterns();
      self.init_regexs();
    }

    // Private functon that initiates the DPI key_words attribute 
    // Call this function from within the init function
    fn init_words(&mut self) {
      match &self.key_words.clone() {
        Some(keys) => {
          for key in keys.iter() {
            &self.add_to_score_points(key.to_string(), KEY_WORD_PNTS);
          }
        },
        None => {},
      }
    }

    // Private function that initiates the DPI key_patterns attribute 
    // Call this function from within the init function
    fn init_patterns(&mut self) {
      match &self.key_patterns.clone() {
        Some(pttrns) => {
          for pttrn in pttrns.iter() {
            &self.add_to_score_points(pttrn.to_string(), KEY_PATTERN_PNTS);
          }
        },
        None => {},
      }
    }

    // Private function that initiates the DPI key_regexs attribute 
    // Call this function from within the init function
    fn init_regexs(&mut self) {
      match &self.key_regexs.clone() {
        Some(regexs) => {
          for regex in regexs.iter() {
            &self.add_to_score_points(regex.to_string(), KEY_REGEX_PNTS);
          }
        },
        None => {},
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
    /// ```rust
    /// use pbd::dpi::DPI;
    ///
    /// let serialized = r#"{"key_words":["SSN"],"key_patterns":["^(?!666|000|9\\d{2})\\d{3}-(?!00)\\d{2}-(?!0{4})\\d{4}$"],"scores":{}}"#;
    /// let dpi = DPI::from_serialized(&serialized);
    ///     
    /// println!("{:?}", dpi);
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
    /// ```rust
    /// use pbd::dpi::DPI;
    ///
    /// let mut dpi = DPI::with(
    ///     Some(vec!["SSN".to_string()]),
    ///     Some(vec![r"^\d{3}-\d{2}-\d{4}$".to_string()]),
    ///     Some(vec!["###p##p####".to_string()])
    ///   );
    /// 
    /// println!("{:?}", dpi.serialize());
    /// ```
    pub fn serialize(&mut self) -> String {
		  serde_json::to_string(&self).unwrap()
    }

    /// Retreives the Score object based on the specified key
    /// 
    /// # Arguments
    /// 
    /// * key: String - The key that identifies the Score object.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::{DPI, Score, ScoreKey};
    ///
    /// let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
    /// let mut dpi = DPI::new();
    ///
    /// dpi.upsert_score(score);
    ///
    /// let returned_score = dpi.get_score("ssn".to_string());
    ///   
    /// assert_eq!(returned_score.points, 25.0);
    /// ```
    pub fn get_score(&mut self, key: String) -> Score {
      match self.scores.get_mut(&key) {
        Some(s) => s.clone(),
        None => {
          Score::new(ScoreKey::KeyWord, key, 0 as f64).clone()
        },
      }
    }

    /// Adds points to an existing Score object
    /// 
    /// # Arguments
    /// 
    /// * key: String - The key that identifies the Score object.</br>
    /// * pnts: f64 - The points to add to the Score object.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::{DPI, Score, ScoreKey};
    ///
    /// let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
    /// let mut dpi = DPI::new();
    ///
    /// dpi.upsert_score(score);
    /// dpi.add_to_score_points("ssn".to_string(), 5.5);
    ///
    /// let returned_score = dpi.get_score("ssn".to_string());
    ///   
    /// assert_eq!(returned_score.points, 30.5);
    /// ```
    pub fn add_to_score_points(&mut self, key: String, pnts: f64) {
      let mut score = self.get_score(key);
      score.points += pnts;
      &self.upsert_score(score);
    }

    /// Determines how many times a pattern appears in a list of tokens
    /// 
    /// # Arguments
    /// 
    /// * pattern: &str - The pattern to search for in the list of tokens.</br>
    /// * tokens: Vec<&str> - The list of tokens to search through for the pattern.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098","Let","me","know","if","you","need","my","son's","ssn"];
    ///     
    /// assert_eq!(DPI::contains_key_pattern(Lib::PTTRN_SSN_DASHES.as_str().unwrap(), tokens), 1);
    /// ```
    pub fn contains_key_pattern(pattern: &str, tokens: Vec<&str>) -> usize {
      tokens.par_iter()
      .filter(|t| {
        let pttrn_def = PatternDefinition::new();
        pttrn_def.analyze(t) == pattern
      })
      .collect::<Vec<&&str>>()
      .len()
    }

    /// Determines how many times a regular expression appears in a list of tokens
    /// 
    /// # Arguments
    /// 
    /// * regex: &str - The regular expression to search for in the list of tokens.</br>
    /// * tokens: Vec<&str> - The list of tokens to search through for the regular expression.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098","Let","me","know","if","you","need","my","son's","ssn"];
    ///     
    /// assert_eq!(DPI::contains_key_regex(Lib::REGEX_SSN_DASHES.as_str().unwrap(), tokens), 1);
    /// ```
    pub fn contains_key_regex(regex: &str, tokens: Vec<&str>) -> usize {
      let re = Regex::new(regex).unwrap();

      tokens.par_iter()
      .filter(|t| {
        re.is_match(t)
      })
      .collect::<Vec<&&str>>()
      .len()
    }

    /// Determines how many times a word appears in a list of tokens
    /// 
    /// # Arguments
    /// 
    /// * word: &str - The word to search for in the list of tokens.</br>
    /// * tokens: Vec<&str> - The list of tokens to search through for the word.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098","Let","me","know","if","you","need","my","son's","ssn"];
    ///     
    /// assert_eq!(DPI::contains_key_word(Lib::TEXT_SSN_ABBR.as_str().unwrap(), tokens), 2);
    /// ```
    pub fn contains_key_word(word: &str, tokens: Vec<&str>) -> usize {
      tokens.par_iter()
      .filter(|t| {
        t.to_lowercase() == word.to_lowercase()
      })
      .collect::<Vec<&&str>>()
      .len()
    }

    /// Trains the DPI object using its key patterns against a the list of words provided as the sample content
    /// 
    /// # Arguments
    /// 
    /// * tokens: Vec<&str> - The list of words that is sample content.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098"];
    /// let patterns = vec![Lib::PTTRN_SSN_DASHES.to_string()];
    /// let mut dpi = DPI::with_key_patterns(patterns);
    /// 
    /// dpi.train_for_key_patterns(tokens);
    ///   
    /// assert_eq!(dpi.get_score(Lib::PTTRN_SSN_DASHES.to_string()).points, 160.0);
    /// ```
    pub fn train_for_key_patterns(&mut self, tokens: Vec<&str>) {
      let kpttrns = self.key_patterns.clone();
      
      let list: Vec<&String> = kpttrns.as_ref().unwrap().par_iter()
        .filter(|x| {
          DPI::contains_key_pattern(x, tokens.clone()) > 0
        })
        .collect();
      
      list.iter()
        .for_each(|p| {
          self.add_to_score_points(p.to_string(), KEY_PATTERN_PNTS);
        });
    }    

    /// Trains the DPI object using its key regular expressions against a the list of words provided as the sample content
    /// 
    /// # Arguments
    /// 
    /// * tokens: Vec<&str> - The list of words that is sample content.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098"];
    /// let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
    /// let mut dpi = DPI::with_key_regexs(regexs);
    /// 
    /// dpi.train_for_key_regexs(tokens);
    ///   
    /// assert_eq!(dpi.get_score(Lib::REGEX_SSN_DASHES.to_string()).points, 180.0);
    /// ```
    pub fn train_for_key_regexs(&mut self, tokens: Vec<&str>) {
      let kregexs = self.key_regexs.clone();

      let list: Vec<&String> = kregexs.as_ref().unwrap().par_iter()
        .filter(|x| {
          DPI::contains_key_regex(x, tokens.clone()) > 0
        })
        .collect();
      
      list.iter()
        .for_each(|t| {
          self.add_to_score_points(t.to_string(), KEY_REGEX_PNTS);
        });
    } 

    /// Trains the DPI object using its key words against a the list of words provided as the sample content
    /// 
    /// # Arguments
    /// 
    /// * tokens: Vec<&str> - The list of words that is sample content.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let tokens = vec!["My","ssn","is","003-76-0098"];
    /// let words = vec![Lib::TEXT_SSN_ABBR.to_string()];
    /// let mut dpi = DPI::with_key_words(words);
    /// 
    /// dpi.train_for_key_words(tokens);
    ///     
    /// assert_eq!(dpi.get_score(Lib::TEXT_SSN_ABBR.to_string()).points, 200.0);
    /// ```
    pub fn train_for_key_words(&mut self, tokens: Vec<&str>) {
      let kwords = self.key_words.clone();
      
      let list: Vec<&String> = kwords.as_ref().unwrap().par_iter()
       .filter(|w| {
          DPI::contains_key_word(w, tokens.clone()) > 0
       })
       .collect();
    
      list.iter()
       .for_each(|w| {
         self.add_to_score_points(w.to_string(), KEY_WORD_PNTS);
       });  
    }

    fn suggest_rom_key_word<'a>(word: &str, tokens: Vec<&'a str>) -> Vec<(&'a str, i8)> {
      let mut suggestions: Vec<(&str, i8)> = Vec::new();
      struct Tknzr {}
      impl Tfidf for Tknzr {}
      let total_count = tokens.len();
      let freq_counts = Tknzr::frequency_counts(tokens.clone());

      for (idx, tkn) in tokens.iter().enumerate() {
        match tkn.to_lowercase() == word.to_lowercase() {
          true => {
            let idx_scope: Vec<i8> = vec![-2, -1, 1, 2];

            for i in &idx_scope {              
              let cnt = freq_counts.get(&tokens[add(idx, *i)]).unwrap();

              if (cnt / total_count ) <= 0.15 as usize {
                suggestions.push(( tokens[add(idx, *i)], *i) );
              }
            } 
          },
          false => {},
        }
      }

      suggestions
    }

    /// Trains the DPI object using its key words against a String as the sample content
    /// 
    /// # Arguments
    /// 
    /// * text: String - The text that is sample content.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::Lib;
    ///
    /// let text = "My ssn is 003-76-0098".to_string();
    /// let words = Some(vec![Lib::TEXT_SSN_ABBR.to_string()]);
    /// let regexs = Some(vec![Lib::REGEX_SSN_DASHES.to_string()]);
    /// let patterns = Some(vec![Lib::PTTRN_SSN_DASHES.to_string()]);
    /// let mut dpi = DPI::with(words, regexs, patterns);
    /// 
    /// dpi.train_using_keys(text);
    ///     
    /// assert_eq!(dpi.get_score(Lib::TEXT_SSN_ABBR.to_string()).points, 200.0);
    /// assert_eq!(dpi.get_score(Lib::REGEX_SSN_DASHES.to_string()).points, 180.0);
    /// assert_eq!(dpi.get_score(Lib::PTTRN_SSN_DASHES.to_string()).points, 160.0);
    /// ```
    pub fn train_using_keys(&mut self, text: String) {
      let tokens = Self::tokenize(&text);
      self.train_for_key_patterns(tokens.clone());
      self.train_for_key_regexs(tokens.clone());
      self.train_for_key_words(tokens);
    }

    /// Update (if not exits then inserts) a Score object
    /// 
    /// # Arguments
    /// 
    /// * score: Score - The Score object to upsert.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::{DPI, Score, ScoreKey};
    ///
    /// let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
    /// let mut dpi = DPI::new();
    /// dpi.upsert_score(score);
    ///     
    /// assert_eq!(dpi.get_score("ssn".to_string()).points, 25.0);
    /// ```
    pub fn upsert_score(&mut self, score: Score) {
      self.scores.insert(score.key_value.clone(), score);
    }

    /// Checks the list of regular expressions to make sure they are valid.
    /// This funciton returns a Result:
    ///   Ok => 1
    ///   Err => List of the invalid regular expressions
    /// 
    /// # Arguments
    /// 
    /// * regexs: KeyRegexList - The list that contains the regular expressions ot validate.</br>
    /// 
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::{DPI};
    ///
    /// let regexs = vec![r"^\d{3}-\d{2}-\d{4}$".to_string()];
    /// 
    /// match DPI::validate_regexs(regexs) {
    ///   Ok(_x) => assert!(true),
    ///   Err(e) => {
    ///     println!("Bad Regexs: {:?}", e);
    ///     assert!(false)
    ///   },
    /// }
    /// ```
    pub fn validate_regexs(regexs: KeyRegexList) -> Result<u8 , KeyRegexList> {
      let bad = regexs.into_par_iter()
        .filter(|x|{
          Regex::new(x).is_err()
        })
        .map(|x| x)
        .collect::<KeyRegexList>();

      if bad.len() == 0 {
        Ok(1)
      } else {
        error!("Bad Regex: {:?}", bad);
        Err(bad)
      }
    }
}

pub mod error;
pub mod reference;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::dpi::reference::Lib;

    fn get_dpi() -> Vec<DPI>{
        let mut v = Vec::new();
        v.push( DPI {
                    key_patterns: Some(vec!["###p##p####".to_string()]),
                    key_regexs: Some(vec![r"^(?!b(d)1+-(d)1+-(d)1+b)(?!123-45-6789|219-09-9999|078-05-1120)(?!666|000|9d{2})d{3}-(?!00)d{2}-(?!0{4})d{4}$".to_string()]),
                    key_words: Some(vec!["ssn".to_string()]),
                    scores: ScoreCard::new(),
                });
        v
    }

    fn get_files() -> Vec<String>{
      let files = vec!["acme_payment_notification.txt","renewal_notification.txt","statement_ready_notification.txt"];      
      let mut docs: Vec<String> = Vec::new();

      for file in files.iter() {
        docs.push(fs::read_to_string(format!("./tests/dpi/{}",file)).expect("File could not be read."));
      }

      docs
    }

    fn get_text() -> String {
      String::from(r#"Here is my ssn that you requested: 003-75-9876."#)
    }

    fn get_tokens() -> Vec<&'static str>{
      let v = vec!["Hello","my","name","is","John","What","is","your","name","A","name","is","a","personal","identifier","Never","share","your","name","My","ssn","is","003-67-0998"];
      let _iter = v.par_iter().map(|t| t.to_string());
      v
    }

    #[test]
    fn test_dpi_new() {
      let dpi = DPI::new();

      assert!(dpi.key_words.is_none());
      assert!(dpi.key_patterns.is_none());
    }  

    #[test]
    fn test_dpi_add_to_score_points() {
      let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
      let mut dpi = DPI::new();
    
      dpi.upsert_score(score);
      dpi.add_to_score_points("ssn".to_string(), 5.5);
    
      let returned_score = dpi.get_score("ssn".to_string());
        
      assert_eq!(returned_score.points, 30.5);
    }    

    #[test]
    fn test_dpi_contains_key_pattern() {
      let tokens = get_tokens();
      assert_eq!(DPI::contains_key_pattern(Lib::PTTRN_SSN_DASHES.as_str().unwrap(), tokens), 1);
    }

    #[test]
    fn test_dpi_contains_key_regex() {
      let mut tokens = get_tokens();
      tokens.push("008-43-2213");
      assert_eq!(DPI::contains_key_regex(Lib::REGEX_SSN_DASHES.as_str().unwrap(), tokens), 2);
    }
    
    #[test]
    fn test_dpi_contains_key_word() {
      let tokens = get_tokens();
      assert_eq!(DPI::contains_key_word(Lib::TEXT_SSN_ABBR.as_str().unwrap(), tokens), 1);
    }

    #[test]
    fn test_dpi_get_score() {
      let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
      let mut dpi = DPI::new();
    
      dpi.upsert_score(score);
    
      let returned_score = dpi.get_score("ssn".to_string());
        
      assert_eq!(returned_score.points, 25.0);
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
        let serialized = "{\"key_patterns\":[\"###p##p####\"],\"key_regexs\":[\"^(?!b(d)1+-(d)1+-(d)1+b)(?!123-45-6789|219-09-9999|078-05-1120)(?!666|000|9d{2})d{3}-(?!00)d{2}-(?!0{4})d{4}$\"],\"key_words\":[\"ssn\"],\"scores\":{}}";
        let dpi = &mut get_dpi()[0];

        assert_eq!(dpi.serialize(), serialized);
    }

    #[test]
    fn test_suggested_key_words() {
      struct Tknzr;
      impl Tokenizer for Tknzr {}
      
      struct TfIdfzr;
      impl Tfidf for TfIdfzr{}

      let word = "account";
      let files = get_files();      
      let mut rslts: BTreeMap<String, f64> = BTreeMap::new();
      let mut docs: Vec<Vec<(&str, usize)>> = Vec::new();

      for content in files.iter() {
        let tokens = Tknzr::tokenize(&content);
        let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
        docs.push(feq_cnts);
        let suggestions = DPI::suggest_rom_key_word(word, tokens);
        
        for (key, _val) in suggestions.iter() {
          let mut n: f64 = 0.00;
          for doc_idx in 0..docs.len() {
            n = n + TfIdfzr::tfidf(key, doc_idx, docs.clone());
            
          }
          //println!("tfidf for {} is {}",key,(n/docs.len() as f64));
          if (n/docs.len() as f64) >= 0.30 as f64 {
            rslts.insert(key.to_string(), n/docs.len() as f64 * KEY_WORD_PNTS);
          }
        }
      }
      
      //for (k,v) in rslts.iter() {
      //  println!("Key: {} val: {}",k,v);
      //}

      assert_eq!(*rslts.get("statement").unwrap(), 67.13741764082893 as f64);
    }

    #[test]
    fn test_dpi_train_for_key_regexs() {
      let tokens = vec!["My","ssn","is","003-76-0098"];
      let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
      let mut dpi = DPI::with_key_regexs(regexs);
    
      dpi.train_for_key_regexs(tokens);
        
      assert_eq!(dpi.get_score(Lib::REGEX_SSN_DASHES.to_string()).points, 180.0);
    }

    #[test]
    fn test_dpi_train_for_key_words() {
      let tokens = vec!["My","ssn","is","003-76-0098"];
      let words = vec!["ssn".to_string()];
      let mut dpi = DPI::with_key_words(words);
        
      dpi.train_for_key_words(tokens);

      assert_eq!(dpi.get_score("ssn".to_string()).points, 200.0);
    }

    #[test]
    fn test_dpi_train_using_keys() {
      let text = get_text();
      let words = Some(vec![Lib::TEXT_SSN_ABBR.to_string()]);
      let regexs = Some(vec![Lib::REGEX_SSN_DASHES.to_string()]);
      let patterns = Some(vec![Lib::PTTRN_SSN_DASHES.to_string()]);
      let mut dpi = DPI::with(words, regexs, patterns);
      
      dpi.train_using_keys(text);

      assert_eq!(dpi.get_score(Lib::TEXT_SSN_ABBR.to_string()).points, 200.0);
      assert_eq!(dpi.get_score(Lib::REGEX_SSN_DASHES.to_string()).points, 180.0);
      assert_eq!(dpi.get_score(Lib::PTTRN_SSN_DASHES.to_string()).points, 160.0);
    } 

    #[test]
    fn test_dpi_with() {
      let words = Some(vec![Lib::TEXT_SSN_ABBR.to_string()]);
      let patterns = Some(vec![Lib::PTTRN_SSN_DASHES.to_string()]);
      let regexs = Some(vec![Lib::REGEX_SSN_DASHES.to_string()]);
      let dpi = DPI::with(words, regexs, patterns);

      assert_eq!(dpi.key_words.unwrap().len(),1);
    }

    #[test]
    fn test_dpi_with_keypatterns() {
      let patterns = vec![Lib::PTTRN_SSN_DASHES.to_string()];
      let dpi = DPI::with_key_patterns(patterns);
      
      assert_eq!(dpi.key_patterns.unwrap().len(),1);
    }

    #[test]
    fn test_dpi_with_keyregexs() {
      let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
      let dpi = DPI::with_key_regexs(regexs);
      
      assert_eq!(dpi.key_regexs.unwrap().len(),1);
    }


    #[test]
    fn test_dpi_with_keywords() {
      let words = vec![Lib::TEXT_SSN_ABBR.to_string()];
      let dpi = DPI::with_key_words(words);
      
      assert_eq!(dpi.key_words.unwrap().len(),1);
    }

    #[test]
    fn test_dpi_upsert_score() {
      let score = Score::new(ScoreKey::KeyWord, "ssn".to_string(), 25.0);
      let mut dpi = DPI::new();
      dpi.upsert_score(score);
        
      assert_eq!(dpi.get_score("ssn".to_string()).points, 25.0);
    } 

    #[test]
    fn test_dpi_validate_regexs_good() {
      let regexs = vec![r"^\d{3}-\d{2}-\d{4}$".to_string()];

      match DPI::validate_regexs(regexs) {
        Ok(_x) => assert!(true),
        Err(e) => {
          println!("Bad Regexs: {:?}", e);
          assert!(false)
        },
      }
    }

    #[test]
    fn test_dpi_validate_regexs_bad() {
      let regexs = vec![r"^(?!b(d)1+b)(?!123456789|219099999|078051120)(?!666|000|9d{2})d{3}(?!00)d{2}(?!0{4})d{4}$".to_string()];

      match DPI::validate_regexs(regexs) {
        Ok(_x) => assert!(false),
        Err(_e) => {
          assert!(true)
        },
      }
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
      let pttrn_def = PatternDefinition::new();
      let rslt = pttrn_def.analyze("Hello World");

      assert_eq!(rslt, "CvccvSCvccc");
    }

    #[test]
    fn test_pattern_analyze_entities() {
      let entities = get_tokens();
      let pttrn_def = PatternDefinition::new();
      let rslt = pttrn_def.analyze_entities(entities);
      let pttrns = vec!["Cvccv", "cc", "cvcv", "vc", "Cvcc", "Ccvc", "vc", "cvvc", "cvcv", "V", "cvcv", "vc", "v", "cvccvcvc", "vcvccvcvvc", "Cvcvc", "ccvcv", "cvvc", "cvcv", "Cc", "ccc", "vc", "###@##@####"];

      assert_eq!(rslt, pttrns);
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
    fn test_tfidf_frequency_counts() {
      struct FreqCnt {}
      impl Tfidf for FreqCnt {}
      let tokens = get_tokens();
      let counts = r#"{"003-67-0998": 1, "A": 1, "Hello": 1, "John": 1, "My": 1, "Never": 1, "What": 1, "a": 1, "identifier": 1, "is": 4, "my": 1, "name": 4, "personal": 1, "share": 1, "ssn": 1, "your": 2}"#;

      assert_eq!(format!("{:?}", FreqCnt::frequency_counts(tokens)), counts);
    }
}