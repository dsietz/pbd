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
extern crate levenshtein;
extern crate regex;

use super::*;
use levenshtein::levenshtein;
use multimap::MultiMap;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use tfidf::{TfIdf, TfIdfDefault};

const KEY_PATTERN_PNTS: f64 = 80_f64;
const KEY_REGEX_PNTS: f64 = 90_f64;
const KEY_WORD_PNTS: f64 = 100_f64;

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

/// The collection of methods that enable a structure to find words that sound alike
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
        (0..4)
            .map(|idx| if idx < chars.len() { chars[idx] } else { '0' })
            .collect()
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
            _ => {
                chars.truncate(4);
                chars
            } //truncate doesn't return self?
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
            _ => '0',         //Vowels
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

    /// Compares 2 words and determines if they similar in spelling (true=yes, false=no)
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
    /// assert!(!Prcsr::similar_word("rupert","robert"));
    /// assert!(Prcsr::similar_word("Johnathan","Jonathan"));
    /// ```
    fn similar_word(word1: &str, word2: &str) -> bool {
        let length = (word1.len() as f64 + word2.len() as f64) / 2.0;
        let diff = levenshtein(word1, word2) as f64;

        if (diff / length) <= 0.30 {
            true
        } else {
            println!("Length:{} diff:{}", length, diff);
            false
        }
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
        for c in chars.iter().skip(1) {
            enc_chars.push(Self::get_char_digit(*c));
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
    /// The default tf-idf limit before the term is considered relevant
    const TFIDF_LIMIT: f64 = 0.50;
    /// The default tf limit before the term is considered relevant
    const TF_LIMIT: f64 = 0.15;

    /// Determines how important a term is in a document compared to other documents.
    ///
    /// # Arguments
    ///
    /// * tokens: Vec<&str> - A vector of words that are to be counted.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::Tfidf;
    ///
    /// struct FreqCnt {}
    /// impl Tfidf for FreqCnt {}
    ///
    /// let mut docs = Vec::new();
    /// let tokens_list = vec![
    ///   vec!["Hello","my","name","is","John","What","is","your","name"],
    ///   vec!["A","name","is","a","personal","identifier","Never","share","your","name"],
    ///   vec!["My","ssn","is","003-67-0998"]
    /// ];
    ///
    /// for tokens in tokens_list {
    ///   docs.push(FreqCnt::frequency_counts_as_vec(tokens));
    /// }
    ///
    /// assert_eq!(FreqCnt::tfidf("ssn", 2, docs.clone()), 1.0986122886681098);
    /// assert_eq!(FreqCnt::tfidf("name", 1, docs.clone()), 0.4054651081081644);
    /// assert_eq!(FreqCnt::tfidf("your", 1, docs), 0.3040988310811233);
    /// ```  
    fn tfidf(term: &str, doc_idx: usize, docs: Vec<Vec<(&str, usize)>>) -> f64 {
        TfIdfDefault::tfidf(term, &docs[doc_idx], docs.iter())
    }

    /// Takes a list of words and returns a distinct list of words with the number of times they appear in the list.
    ///
    /// # Arguments
    ///
    /// * tokens: Vec<&str> - A vector of words that are to be counted.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::Tfidf;
    ///
    /// struct FreqCnt {}
    /// impl Tfidf for FreqCnt {}
    /// let tokens = vec!["Hello","my","name","is","John","What","is","your","name","A","name","is","a","personal","identifier","Never","share","your","name","My","ssn","is","003-67-0998"];
    /// let _iter = tokens.iter().map(|t| t.to_string());
    ///
    /// println!("{:?}", FreqCnt::frequency_counts_as_vec(tokens));
    /// ```
    fn frequency_counts_as_vec(tokens: Vec<&str>) -> Vec<(&str, usize)> {
        let mut counts: Vec<(&str, usize)> = Vec::new();

        // MapReduce
        // Map input collection.
        let mapped: Vec<_> = tokens
            .into_par_iter()
            //.map(|s| s.chars()
            //    //.filter(|c| c.is_alphabetic()).collect::<String>())
            //    .collect::<String>())
            .map(|s| (s, ()))
            .collect();

        // Group by key.
        let shuffled = mapped
            .into_iter()
            .collect::<MultiMap<_, _>>()
            .into_iter()
            .collect::<Vec<_>>();
        // Reduce by key.
        let mut reduced: Vec<_> = shuffled
            .into_par_iter()
            .map(|kv| (kv.0, kv.1.len())) // Only using count of values
            .collect();

        // Post processing descending sort
        reduced.sort_by(|a, b| match a.1.cmp(&b.1).reverse() {
            Ordering::Equal => a.0.cmp(&b.0),
            other_ordering => other_ordering,
        });

        // Collect results
        for (word, count) in reduced.into_iter() {
            counts.push((word, count));
        }

        counts
    }

    /// Takes a list of words and returns a BTreeMap of key words with the number of times they appear in the list.
    ///
    /// # Arguments
    ///
    /// * tokens: Vec<&str> - A vector of words that are to be counted.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::Tfidf;
    ///
    /// struct FreqCnt {}
    /// impl Tfidf for FreqCnt {}
    /// let tokens = vec!["Hello","my","name","is","John","What","is","your","name","A","name","is","a","personal","identifier","Never","share","your","name","My","ssn","is","003-67-0998"];
    /// let _iter = tokens.iter().map(|t| t.to_string());
    /// let counts = FreqCnt::frequency_counts(tokens);
    ///
    /// assert_eq!(*counts.get("name").unwrap(), 4 as usize);
    /// ```
    fn frequency_counts(tokens: Vec<&str>) -> BTreeMap<&str, usize> {
        let counts: Vec<(&str, usize)> = Self::frequency_counts_as_vec(tokens);

        // Convert to BTreeMap
        let mut list = BTreeMap::new();
        for count in counts.iter() {
            list.insert(count.0, count.1);
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
                let sl = &tokenized_sequence[0..(n - num_blanks)];
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
                tc.extend_from_slice(&tokenized_sequence[(last_entry - num_tokens)..last_entry]);
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
        matches!(
            c,
            ' ' | ','
                | '.'
                | '!'
                | '?'
                | ';'
                | '\''
                | '"'
                | ':'
                | '\t'
                | '\n'
                | '\r'
                | '('
                | ')'
                | '{'
                | '}'
        )
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
    /// let pttrn_def = PatternDefinition::new();
    /// ```
    pub fn new() -> PatternDefinition {
        let symbols: [char; 9] = ['@', 'C', 'c', 'V', 'v', '#', '~', 'S', 'p'];
        let mut pttrn_def = PatternMap::new();

        pttrn_def.insert("Unknown".to_string(), symbols[0]);
        pttrn_def.insert("ConsonantUpper".to_string(), symbols[1]);
        pttrn_def.insert("ConsonantLower".to_string(), symbols[2]);
        pttrn_def.insert("VowelUpper".to_string(), symbols[3]);
        pttrn_def.insert("VowelLower".to_string(), symbols[4]);
        pttrn_def.insert("Numeric".to_string(), symbols[5]);
        pttrn_def.insert("RegExSpcChar".to_string(), symbols[6]);
        pttrn_def.insert("WhiteSpace".to_string(), symbols[7]);
        pttrn_def.insert("Punctuation".to_string(), symbols[8]);

        PatternDefinition {
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
        let pttrns: Vec<_> = entities
            .into_par_iter()
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

impl Default for PatternDefinition {
    fn default() -> Self {
        Self::new()
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

/// Represents a Suggestion
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suggestion {
    /// The word being suggested
    pub word: String,
    /// The regex that represents the suggested word
    pub regex: Option<String>,
    /// The pattern that represents the suggested word
    pub pattern: Option<String>,
    /// The points the suggestion has received as a score of relavence
    pub points: f64,
}

impl Suggestion {
    /// Constructs a Suggestion object
    ///
    /// # Arguments
    ///
    /// * token: String- The word being suggested.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::Suggestion;
    /// let suggestion = Suggestion::new("dob".to_string());
    /// ```
    pub fn new(token: String) -> Suggestion {
        Suggestion {
            word: token,
            regex: None,
            pattern: None,
            points: 0.0,
        }
    }

    /// Constructs a Suggestion object with all the attributes set
    ///
    /// # Arguments
    ///
    /// * token: String- The word being suggested.</br>
    /// * regex: String - The regex that represents the suggested word, (e.g.: "[aA-zZ]{3}").</br>
    /// * pttrn: String - The pattern that represents the suggested word, (e.g.: "cvc").</br>
    /// * pnts: f64 - The scored points that the key has received
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::Suggestion;
    /// let suggestion = Suggestion::with("dob".to_string(),"[aA-zZ]{3}".to_string(), "cvc".to_string(), 0.59874856);
    /// ```
    pub fn with(token: String, regex: String, pttrn: String, pnts: f64) -> Suggestion {
        Suggestion {
            word: token,
            regex: Some(regex),
            pattern: Some(pttrn),
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

impl Phonetic for DPI {}
impl Tokenizer for DPI {}
impl Tfidf for DPI {}

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
    pub fn with(
        words: Option<KeyWordList>,
        regexs: Option<KeyWordList>,
        patterns: Option<KeyWordList>,
    ) -> DPI {
        if let Some(reg) = regexs.clone() {
            if let Err(err) = Self::validate_regexs(reg) {
                panic!("Bad Regex: {:?}", err);
            }
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
            Ok(_) => {}
            Err(err) => {
                panic!("Bad Regex: {:?}", err);
            }
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
                    self.add_to_score_points(key.to_string(), KEY_WORD_PNTS);
                }
            }
            None => {}
        }
    }

    // Private function that initiates the DPI key_patterns attribute
    // Call this function from within the init function
    fn init_patterns(&mut self) {
        match &self.key_patterns.clone() {
            Some(pttrns) => {
                for pttrn in pttrns.iter() {
                    self.add_to_score_points(pttrn.to_string(), KEY_PATTERN_PNTS);
                }
            }
            None => {}
        }
    }

    // Private function that initiates the DPI key_regexs attribute
    // Call this function from within the init function
    fn init_regexs(&mut self) {
        match &self.key_regexs.clone() {
            Some(regexs) => {
                for regex in regexs.iter() {
                    self.add_to_score_points(regex.to_string(), KEY_REGEX_PNTS);
                }
            }
            None => {}
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
            None => Score::new(ScoreKey::KeyWord, key, 0 as f64),
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
        self.upsert_score(score);
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
        tokens
            .par_iter()
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

        tokens
            .par_iter()
            .filter(|t| re.is_match(t))
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
        tokens
            .par_iter()
            .filter(|t| t.to_lowercase() == word.to_lowercase())
            .collect::<Vec<&&str>>()
            .len()
    }

    fn get_suggested_words_from_patterns(
        key_patterns: Vec<String>,
        docs: Vec<String>,
    ) -> Vec<(String, f64)> {
        struct TfIdfzr;
        impl Tfidf for TfIdfzr {}

        let mut rslts: Vec<(String, f64)> = Vec::new();
        let mut cnts: Vec<Vec<(&str, usize)>> = Vec::new();

        docs.iter().for_each(|text| {
            let tokens = Self::tokenize(&text);
            let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
            cnts.push(feq_cnts);
        });

        docs.iter().for_each(|text| {
            for pattern in key_patterns.clone().iter() {
                let tokens = Self::tokenize(&text).clone();
                let suggestions = DPI::suggest_from_key_pattern(pattern, tokens);

                for (key, _val) in suggestions.iter() {
                    let mut n: f64 = 0.00;
                    for doc_idx in 0..docs.len() {
                        n += TfIdfzr::tfidf(key, doc_idx, cnts.clone());
                    }
                    if (n / docs.len() as f64) >= Self::TFIDF_LIMIT as f64 {
                        rslts.push((key.to_string(), n / docs.len() as f64 * KEY_WORD_PNTS));
                    }
                }
            }
        });

        rslts
    }

    fn get_suggested_words_from_regexs(
        key_regexs: Vec<String>,
        docs: Vec<String>,
    ) -> Vec<(String, f64)> {
        struct TfIdfzr;
        impl Tfidf for TfIdfzr {}

        let mut rslts: Vec<(String, f64)> = Vec::new();
        let mut cnts: Vec<Vec<(&str, usize)>> = Vec::new();

        docs.iter().for_each(|text| {
            let tokens = Self::tokenize(&text);
            let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
            cnts.push(feq_cnts);
        });

        docs.iter().for_each(|text| {
            for regex in key_regexs.clone().iter() {
                let tokens = Self::tokenize(&text).clone();
                let suggestions = DPI::suggest_from_key_regex(regex, tokens);

                for (key, _val) in suggestions.iter() {
                    let mut n: f64 = 0.00;
                    for doc_idx in 0..docs.len() {
                        n += TfIdfzr::tfidf(key, doc_idx, cnts.clone());
                    }
                    if (n / docs.len() as f64) >= Self::TFIDF_LIMIT as f64 {
                        rslts.push((key.to_string(), n / docs.len() as f64 * KEY_WORD_PNTS));
                    }
                }
            }
        });

        rslts
    }

    fn get_suggested_words_from_words(
        key_words: Vec<String>,
        docs: Vec<String>,
    ) -> Vec<(String, f64)> {
        struct TfIdfzr;
        impl Tfidf for TfIdfzr {}

        let mut rslts: Vec<(String, f64)> = Vec::new();
        let mut cnts: Vec<Vec<(&str, usize)>> = Vec::new();

        docs.iter().for_each(|text| {
            let tokens = Self::tokenize(&text);
            let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
            cnts.push(feq_cnts);
        });

        docs.iter().for_each(|text| {
            for word in key_words.clone().iter() {
                let tokens = Self::tokenize(&text).clone();
                let suggestions = DPI::suggest_from_key_word(word, tokens);

                for (key, _val) in suggestions.iter() {
                    let mut n: f64 = 0.00;
                    for doc_idx in 0..docs.len() {
                        n += TfIdfzr::tfidf(key, doc_idx, cnts.clone());
                    }
                    if (n / docs.len() as f64) >= Self::TFIDF_LIMIT as f64 {
                        rslts.push((key.to_string(), n / docs.len() as f64 * KEY_WORD_PNTS));
                    }
                }
            }
        });

        rslts
    }

    fn suggest_from_key_pattern<'a>(pattern: &str, tokens: Vec<&'a str>) -> Vec<(&'a str, i8)> {
        let mut suggestions: Vec<(&str, i8)> = Vec::new();
        struct Tknzr {}
        impl Tfidf for Tknzr {}
        let total_count = tokens.len();
        let freq_counts = Tknzr::frequency_counts(tokens.clone());

        for (idx, tkn) in tokens.iter().enumerate() {
            let pttrn_def = PatternDefinition::new();
            if pttrn_def.analyze(tkn) == pattern {
                let idx_scope: Vec<i8> = vec![-2, -1, 1, 2];
                for i in &idx_scope {
                    let cnt = freq_counts.get(&tokens[add(idx, *i)]).unwrap();
                    if (cnt / total_count) <= Self::TF_LIMIT as usize {
                        suggestions.push((tokens[add(idx, *i)], *i));
                    }
                }
            }
        }

        suggestions
    }

    fn suggest_from_key_regex<'a>(regex: &str, tokens: Vec<&'a str>) -> Vec<(&'a str, i8)> {
        let mut suggestions: Vec<(&str, i8)> = Vec::new();
        struct Tknzr {}
        impl Tfidf for Tknzr {}
        let total_count = tokens.len();
        let freq_counts = Tknzr::frequency_counts(tokens.clone());

        for (idx, tkn) in tokens.iter().enumerate() {
            if Regex::new(regex).unwrap().is_match(tkn) {
                let idx_scope: Vec<i8> = vec![-2, -1, 1, 2];
                for i in &idx_scope {
                    let cnt = freq_counts.get(&tokens[add(idx, *i)]).unwrap();
                    if (cnt / total_count) <= Self::TF_LIMIT as usize {
                        suggestions.push((tokens[add(idx, *i)], *i));
                    }
                }
            }
        }

        suggestions
    }

    fn suggest_from_key_word<'a>(word: &str, tokens: Vec<&'a str>) -> Vec<(&'a str, i8)> {
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

                        if (cnt / total_count) <= Self::TF_LIMIT as usize {
                            suggestions.push((tokens[add(idx, *i)], *i));
                        }
                    }
                }
                false => {}
            }
        }

        suggestions
    }
    #[allow(dead_code)]
    fn suggest_from_sounds_like<'a>(word: &str, tokens: Vec<&'a str>) -> Vec<(&'a str, usize)> {
        let mut suggestions: Vec<(&str, usize)> = Vec::new();

        for (idx, tkn) in tokens.iter().enumerate() {
            match Self::sounds_like(word, tkn) {
                true => {
                    suggestions.push((tkn, idx));
                }
                false => {}
            }
        }

        suggestions
    }

    /// Trains the DPI object using its keys against a list of Strings as the sample content.
    /// Returns a `BTreeMap<String, f64>` of suggested key words with average Tfidf greater than Tfidf::TFIDF_LIMIT
    /// which are recommended as additional keys for consideration.
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
    /// let mut docs = Vec::new();
    /// docs.push("Dear Acme Client, Thank you for your payment On 12/01/2020, a payment of $354.42 was received on your membership account in 3869. For transaction details, or to view statements, account information and more, please sign in to our Customer Portal at acme.com or our Mobile Portal application. We\'re here for you. If you need assistance, please call our Client Services Department at (800) 226-7321, Monday through Friday 8 a.m. to 8 p.m., Saturday 9 a.m. to 3 p.m., ET. Thank you for being a valued Acme client. ".to_string());
    /// docs.push("Thank you for being a loyal customer, John! Your membership renewal documents and billing information are now available in your online account. Get your new membership ID card. We\'ve gone green! From now on, your ID card will be available through your online account. You can save them to your mobile device or print them as proof of membership. Questions for customer service? Text your billing and policy questions to 1-800-111-2222. Quickly access your policy documents and more with our app. Access your ID card and policy details on the go Update your account information Pay your bill or update your payment preferences".to_string());
    /// docs.push("Dear JOHN DOE, Your current bank statement for ACCOUNT ENDING WITH *0011 was created on 01/31/2019 and is now available to view online. To access your statement, please sign on to online banking and select the Statements link. Please do not respond directly to this e-mail message. If you have any questions, please contact us at 1-800-325-6149. Sincerely, Helpful Bank ".to_string());
    ///
    /// let words = Some(vec![Lib::TEXT_ACCOUNT.to_string(),"membership".to_string()]);
    /// let patterns = Some(vec![Lib::PTTRN_ACCOUNT_CAMEL.to_string(),Lib::PTTRN_ACCOUNT_UPPER.to_string(),Lib::PTTRN_ACCOUNT_LOWER.to_string()]);
    /// let regexs = Some(vec![Lib::REGEX_ACCOUNT.to_string()]);
    /// let mut dpi = DPI::with(words, regexs, patterns);   
    /// let suggestions = dpi.train(docs);
    ///
    /// println!("SCORES: {:?}", dpi.scores);      
    /// println!("SUGGESTIONS: {:?}", suggestions);
    /// ```
    pub fn train(&mut self, docs: Vec<String>) -> BTreeMap<String, Suggestion> {
        let mut keys: Vec<(f64, Vec<String>)> = Vec::new();

        if let Some(k) = self.key_patterns.clone() {
            keys.push((KEY_PATTERN_PNTS, k))
        }
        if let Some(k) = self.key_regexs.clone() {
            keys.push((KEY_REGEX_PNTS, k))
        }
        if let Some(k) = self.key_words.clone() {
            keys.push((KEY_WORD_PNTS, k))
        }

        docs.iter().for_each(|text| {
            let tokens = Self::tokenize(&text);
            let rslts = Self::train_from_keys(keys.clone(), tokens);
            rslts.iter().for_each(|t| {
                self.add_to_score_points(t.0.to_string(), t.1);
            });
        });

        // get suggested words
        let mut rtn: BTreeMap<String, Suggestion> = BTreeMap::new();

        if self.key_words.is_some() {
            let words =
                Self::get_suggested_words_from_words(self.key_words.clone().unwrap(), docs.clone());
            words.iter().for_each(|s| {
                let suggest = Suggestion::with(
                    s.0.clone(),
                    Self::word_to_regex(s.0.to_string()),
                    Self::word_to_pattern(s.0.to_string()),
                    s.1,
                );
                rtn.insert(s.0.clone(), suggest);
            });
        }

        if self.key_regexs.is_some() {
            let words = Self::get_suggested_words_from_regexs(
                self.key_regexs.clone().unwrap(),
                docs.clone(),
            );
            words.iter().for_each(|s| {
                let suggest = Suggestion::with(
                    s.0.clone(),
                    Self::word_to_regex(s.0.to_string()),
                    Self::word_to_pattern(s.0.to_string()),
                    s.1,
                );
                rtn.insert(s.0.clone(), suggest);
            });
        }

        if self.key_patterns.is_some() {
            let words =
                Self::get_suggested_words_from_patterns(self.key_patterns.clone().unwrap(), docs);
            words.iter().for_each(|s| {
                let suggest = Suggestion::with(
                    s.0.clone(),
                    Self::word_to_regex(s.0.to_string()),
                    Self::word_to_pattern(s.0.to_string()),
                    s.1,
                );
                rtn.insert(s.0.clone(), suggest);
            });
        }

        rtn
    }

    fn word_to_pattern(word: String) -> String {
        let pttrn = PatternDefinition::new();
        pttrn.analyze(&word)
    }

    fn word_to_regex(word: String) -> String {
        let mut rtn = String::new();

        word.chars().for_each(|c| {
            if c.is_alphabetic() {
                rtn.push_str("[aA-zZ]");
            }
            if c.is_ascii_digit() {
                rtn.push_str("[0-9]");
            }
            if !c.is_ascii_alphanumeric() {
                rtn.push_str("[^a-zA-Z\\d\\s:]");
            }
        });

        rtn
    }

    /// Trains the DPI object using key patterns against a the list of words provided as the sample content and
    /// returns a list of found patterns and points slices
    ///
    /// # Arguments
    ///
    /// * pttrns: Vec<String> - The list of patterns to use for training.</br>
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
    /// println!("{:?}", DPI::train_for_key_patterns(dpi.key_patterns.clone().unwrap(), tokens));
    /// ```
    pub fn train_for_key_patterns(pttrns: Vec<String>, tokens: Vec<&str>) -> Vec<(String, f64)> {
        pttrns
            .par_iter()
            .filter(|p| DPI::contains_key_pattern(p, tokens.clone()) > 0)
            .map(|p| (p.to_string(), KEY_PATTERN_PNTS))
            .collect()
    }

    /// Trains the DPI object using key regular expressions against a the list of words provided as the sample content and
    /// returns a list of found regular expressions and points slices
    ///
    /// # Arguments
    ///
    /// * regexs: Vec<String> - The list of regular expressions to use for training.</br>
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
    /// println!("{:?}", DPI::train_for_key_regexs(dpi.key_regexs.clone().unwrap(), tokens));
    /// ```
    pub fn train_for_key_regexs(regexs: Vec<String>, tokens: Vec<&str>) -> Vec<(String, f64)> {
        regexs
            .par_iter()
            .filter(|x| DPI::contains_key_regex(x, tokens.clone()) > 0)
            .map(|x| (x.to_string(), KEY_REGEX_PNTS))
            .collect()
    }

    /// Trains the DPI object using key words against a the list of words provided as the sample content and
    /// returns a list of found word and points slices
    ///
    /// # Arguments
    ///
    /// * words: Vec<String> - The list of words to use for training.</br>
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
    /// let dpi = DPI::with_key_words(words);
    ///
    /// println!("{:?}", DPI::train_for_key_words(dpi.key_words.clone().unwrap(), tokens));
    /// ```
    pub fn train_for_key_words(words: Vec<String>, tokens: Vec<&str>) -> Vec<(String, f64)> {
        //let kwords = words.clone();
        words
            .par_iter()
            .filter(|w| DPI::contains_key_word(w, tokens.clone()) > 0)
            .map(|w| (w.to_string(), KEY_WORD_PNTS))
            .collect()
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
    /// use pbd::dpi::{DPI, Tokenizer};
    /// use pbd::dpi::reference::Lib;
    ///
    /// struct Tknzr {}
    ///  impl Tokenizer for Tknzr{}
    ///
    ///  let text = "My ssn is 003-76-0098".to_string();
    ///  let tokens = Tknzr::tokenize(&text);
    ///  let words = (100 as f64, vec![Lib::TEXT_SSN_ABBR.to_string()]);
    ///  let regexs = (90 as f64, vec![Lib::REGEX_SSN_DASHES.to_string()]);
    ///  let patterns = (80 as f64, vec![Lib::PTTRN_SSN_DASHES.to_string()]);
    ///  
    ///  let mut pnts: f64 = 0.0;
    ///  let rslts = DPI::train_from_keys(vec![patterns, regexs, words,], tokens);
    ///
    ///  println!("{:?}",rslts);
    /// ```
    pub fn train_from_keys(keys: Vec<(f64, Vec<String>)>, tokens: Vec<&str>) -> Vec<(String, f64)> {
        let mut rtn: Vec<(String, f64)> = Vec::new();
        let pttrns: Vec<(f64, Vec<String>)> = keys
            .iter()
            .filter(|(k, _)| k == &KEY_PATTERN_PNTS)
            .map(|x| (x.0, x.1.clone()))
            .collect();
        let regexs: Vec<(f64, Vec<String>)> = keys
            .iter()
            .filter(|(k, _)| k == &KEY_REGEX_PNTS)
            .map(|x| (x.0, x.1.clone()))
            .collect();
        let words: Vec<(f64, Vec<String>)> = keys
            .iter()
            .filter(|(k, _)| k == &KEY_WORD_PNTS)
            .map(|x| (x.0, x.1.clone()))
            .collect();

        pttrns.iter().for_each(|(_, v)| {
            rtn.append(&mut Self::train_for_key_patterns(
                v.to_vec(),
                tokens.clone(),
            ))
        });
        regexs.iter().for_each(|(_, v)| {
            rtn.append(&mut Self::train_for_key_regexs(v.to_vec(), tokens.clone()))
        });
        words.iter().for_each(|(_, v)| {
            rtn.append(&mut Self::train_for_key_words(v.to_vec(), tokens.clone()))
        });

        rtn
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
    pub fn validate_regexs(regexs: KeyRegexList) -> Result<u8, KeyRegexList> {
        let bad = regexs
            .into_par_iter()
            .filter(|x| Regex::new(x).is_err())
            .map(|x| x)
            .collect::<KeyRegexList>();

        if bad.is_empty() {
            Ok(1)
        } else {
            error!("Bad Regex: {:?}", bad);
            Err(bad)
        }
    }
}

impl Default for DPI {
    fn default() -> Self {
        Self::new()
    }
}

pub mod error;
pub mod reference;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dpi::reference::Lib;
    use std::fs;

    fn get_dpi() -> Vec<DPI> {
        let mut v = Vec::new();
        v.push( DPI {
                    key_patterns: Some(vec!["###p##p####".to_string()]),
                    key_regexs: Some(vec![r"^(?!b(d)1+-(d)1+-(d)1+b)(?!123-45-6789|219-09-9999|078-05-1120)(?!666|000|9d{2})d{3}-(?!00)d{2}-(?!0{4})d{4}$".to_string()]),
                    key_words: Some(vec!["ssn".to_string()]),
                    scores: ScoreCard::new(),
                });
        v
    }

    fn get_files() -> Vec<String> {
        let files = vec![
            "acme_payment_notification.txt",
            "renewal_notification.txt",
            "statement_ready_notification.txt",
        ];
        let mut docs: Vec<String> = Vec::new();

        for file in files.iter() {
            docs.push(
                fs::read_to_string(format!("./tests/dpi/{}", file))
                    .expect("File could not be read."),
            );
        }

        docs
    }

    fn get_text() -> String {
        String::from(r#"Here is my ssn that you requested: 003-75-9876."#)
    }

    fn get_tokens() -> Vec<&'static str> {
        let v = vec![
            "Hello",
            "my",
            "name",
            "is",
            "John",
            "What",
            "is",
            "your",
            "name",
            "A",
            "name",
            "is",
            "a",
            "personal",
            "identifier",
            "Never",
            "share",
            "your",
            "name",
            "My",
            "ssn",
            "is",
            "003-67-0998",
        ];
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
        assert_eq!(
            DPI::contains_key_pattern(Lib::PTTRN_SSN_DASHES.as_str().unwrap(), tokens),
            1
        );
    }

    #[test]
    fn test_dpi_contains_key_regex() {
        let mut tokens = get_tokens();
        tokens.push("008-43-2213");
        assert_eq!(
            DPI::contains_key_regex(Lib::REGEX_SSN_DASHES.as_str().unwrap(), tokens),
            2
        );
    }

    #[test]
    fn test_dpi_contains_key_word() {
        let tokens = get_tokens();
        assert_eq!(
            DPI::contains_key_word(Lib::TEXT_SSN_ABBR.as_str().unwrap(), tokens),
            1
        );
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
    fn test_suggested_key_regexs() {
        struct Tknzr;
        impl Tokenizer for Tknzr {}

        struct TfIdfzr;
        impl Tfidf for TfIdfzr {}

        let regex = "([Aa]..[aeiouAEIOU]{2}..)";
        let files = get_files();
        let mut rslts: BTreeMap<String, f64> = BTreeMap::new();
        let mut docs: Vec<Vec<(&str, usize)>> = Vec::new();

        for content in files.iter() {
            let tokens = Tknzr::tokenize(&content);
            let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
            docs.push(feq_cnts);
            let suggestions = DPI::suggest_from_key_regex(regex, tokens);

            for (key, _val) in suggestions.iter() {
                let mut n: f64 = 0.00;
                for doc_idx in 0..docs.len() {
                    n = n + TfIdfzr::tfidf(key, doc_idx, docs.clone());
                }

                if (n / docs.len() as f64) >= DPI::TFIDF_LIMIT as f64 {
                    rslts.insert(key.to_string(), n / docs.len() as f64 * KEY_WORD_PNTS);
                }
            }
        }

        assert_eq!(*rslts.get("statement").unwrap(), 67.13741764082893 as f64);
    }

    #[test]
    fn test_dpi_suggest_from_sounds_like() {
        let tokens = vec![
            "Hello",
            "my",
            "name",
            "is",
            "Robert",
            "Smith",
            "I",
            "was",
            "wondering",
            "if",
            "you",
            "would",
            "send",
            "me",
            "the",
            "application",
            "My",
            "address",
            "is",
            "42",
            "Sunny",
            "Way",
            "AnyTown",
            "MN",
            "09887",
        ];
        let suggestions = DPI::suggest_from_sounds_like("Sunday", tokens);
        let expected = vec![("Smith", 5)];

        assert_eq!(suggestions, expected);
    }

    #[test]
    fn test_suggested_key_words() {
        struct Tknzr;
        impl Tokenizer for Tknzr {}

        struct TfIdfzr;
        impl Tfidf for TfIdfzr {}

        let word = "account";
        let files = get_files();
        let mut rslts: BTreeMap<String, f64> = BTreeMap::new();
        let mut docs: Vec<Vec<(&str, usize)>> = Vec::new();

        for content in files.iter() {
            let tokens = Tknzr::tokenize(&content);
            let feq_cnts = TfIdfzr::frequency_counts_as_vec(tokens.clone());
            docs.push(feq_cnts);
            let suggestions = DPI::suggest_from_key_word(word, tokens);

            for (key, _val) in suggestions.iter() {
                let mut n: f64 = 0.00;
                for doc_idx in 0..docs.len() {
                    n = n + TfIdfzr::tfidf(key, doc_idx, docs.clone());
                }

                if (n / docs.len() as f64) >= DPI::TFIDF_LIMIT as f64 {
                    rslts.insert(key.to_string(), n / docs.len() as f64 * KEY_WORD_PNTS);
                }
            }
        }

        assert_eq!(*rslts.get("statement").unwrap(), 67.13741764082893 as f64);
    }

    #[test]
    fn test_dpi_train() {
        let files = get_files();
        let mut docs: Vec<String> = Vec::new();
        let words = Some(vec![
            Lib::TEXT_ACCOUNT.to_string(),
            "membership".to_string(),
        ]);
        let patterns = Some(vec![
            Lib::PTTRN_ACCOUNT_CAMEL.to_string(),
            Lib::PTTRN_ACCOUNT_UPPER.to_string(),
            Lib::PTTRN_ACCOUNT_LOWER.to_string(),
        ]);
        let regexs = Some(vec![Lib::REGEX_ACCOUNT.to_string()]);
        let mut dpi = DPI::with(words, regexs, patterns);

        for content in files.iter() {
            docs.push(content.to_string());
        }

        let suggestions = dpi.train(docs);

        assert_eq!(dpi.get_score(Lib::TEXT_ACCOUNT.to_string()).points, 400.0);
        assert_eq!(dpi.get_score(Lib::REGEX_ACCOUNT.to_string()).points, 360.0);
        assert_eq!(
            dpi.get_score(Lib::PTTRN_ACCOUNT_CAMEL.to_string()).points,
            80.0
        );
        assert_eq!(
            dpi.get_score(Lib::PTTRN_ACCOUNT_LOWER.to_string()).points,
            240.0
        );
        assert_eq!(
            dpi.get_score(Lib::PTTRN_ACCOUNT_UPPER.to_string()).points,
            160.0
        );
        assert_eq!(
            suggestions.get("statement").unwrap().points,
            67.13741764082893
        );

        println!("SUGGESTIONS: {:?}", suggestions);
        let _3869 = suggestions.get("3869").unwrap();
        assert_eq!(_3869.points, 59.50816563618928);
        assert_eq!(_3869.regex.as_ref().unwrap(), "[0-9][0-9][0-9][0-9]");
        assert_eq!(_3869.pattern.as_ref().unwrap(), "####");
    }

    #[test]
    fn test_dpi_train_for_key_regexs() {
        let tokens = vec!["My", "ssn", "is", "003-76-0098"];
        let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
        let dpi = DPI::with_key_regexs(regexs);

        let rslts = DPI::train_for_key_regexs(dpi.key_regexs.clone().unwrap(), tokens);

        assert_eq!(rslts[0].1, 90.0);
    }

    #[test]
    fn test_dpi_train_for_key_words() {
        let tokens = vec!["My", "ssn", "is", "003-76-0098"];
        let words = vec!["ssn".to_string()];
        let dpi = DPI::with_key_words(words);

        let rslts = DPI::train_for_key_words(dpi.key_words.clone().unwrap(), tokens);

        assert_eq!(rslts[0].1, 100.0);
    }

    #[test]
    fn test_dpi_train_using_keys() {
        struct Tknzr {}
        impl Tokenizer for Tknzr {}

        let text = get_text();
        let tokens = Tknzr::tokenize(&text);
        let words = (KEY_WORD_PNTS, vec![Lib::TEXT_SSN_ABBR.to_string()]);
        let regexs = (KEY_REGEX_PNTS, vec![Lib::REGEX_SSN_DASHES.to_string()]);
        let patterns = (KEY_PATTERN_PNTS, vec![Lib::PTTRN_SSN_DASHES.to_string()]);

        let mut pnts: f64 = 0.0;
        let rslts = DPI::train_from_keys(vec![patterns, regexs, words], tokens);
        rslts.iter().for_each(|x| pnts = pnts + x.1);

        assert_eq!(pnts, 270.0);
    }

    #[test]
    fn test_dpi_with() {
        let words = Some(vec![Lib::TEXT_SSN_ABBR.to_string()]);
        let patterns = Some(vec![Lib::PTTRN_SSN_DASHES.to_string()]);
        let regexs = Some(vec![Lib::REGEX_SSN_DASHES.to_string()]);
        let dpi = DPI::with(words, regexs, patterns);

        assert_eq!(dpi.key_words.unwrap().len(), 1);
    }

    #[test]
    fn test_dpi_with_keypatterns() {
        let patterns = vec![Lib::PTTRN_SSN_DASHES.to_string()];
        let dpi = DPI::with_key_patterns(patterns);

        assert_eq!(dpi.key_patterns.unwrap().len(), 1);
    }

    #[test]
    fn test_dpi_with_keyregexs() {
        let regexs = vec![Lib::REGEX_SSN_DASHES.to_string()];
        let dpi = DPI::with_key_regexs(regexs);

        assert_eq!(dpi.key_regexs.unwrap().len(), 1);
    }

    #[test]
    fn test_dpi_with_keywords() {
        let words = vec![Lib::TEXT_SSN_ABBR.to_string()];
        let dpi = DPI::with_key_words(words);

        assert_eq!(dpi.key_words.unwrap().len(), 1);
    }

    #[test]
    fn test_word_to_regex() {
        let sample = vec![
            "1234".to_string(),
            "1aA4".to_string(),
            "$100".to_string(),
            "Smith".to_string(),
            "14%".to_string(),
        ];

        for s in sample {
            assert!(Regex::new(&DPI::word_to_regex(s.to_string()))
                .unwrap()
                .is_match(&s));
        }
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
            }
        }
    }

    #[test]
    fn test_dpi_validate_regexs_bad() {
        let regexs = vec![r"^(?!b(d)1+b)(?!123456789|219099999|078051120)(?!666|000|9d{2})d{3}(?!00)d{2}(?!0{4})d{4}$".to_string()];

        match DPI::validate_regexs(regexs) {
            Ok(_x) => assert!(false),
            Err(_e) => {
                assert!(true)
            }
        }
    }

    #[test]
    fn test_ngram_calculate() {
        struct Prcsr;
        impl Tokenizer for Prcsr {}

        assert_eq!(
            Prcsr::ngram("This is my private data", 2, "----"),
            vec![
                ["----", "This"],
                ["This", "is"],
                ["is", "my"],
                ["my", "private"],
                ["private", "data"],
                ["data", "----"]
            ]
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
        let pttrns = vec![
            "Cvccv",
            "cc",
            "cvcv",
            "vc",
            "Cvcc",
            "Ccvc",
            "vc",
            "cvvc",
            "cvcv",
            "V",
            "cvcv",
            "vc",
            "v",
            "cvccvcvc",
            "vcvccvcvvc",
            "Cvcvc",
            "ccvcv",
            "cvvc",
            "cvcv",
            "Cc",
            "ccc",
            "vc",
            "###@##@####",
        ];

        assert_eq!(rslt, pttrns);
    }

    #[test]
    fn test_phonetic_char_digit() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(Prcsr::get_char_digit('p'), '1');
        assert_eq!(Prcsr::get_char_digit('g'), '2');
        assert_eq!(Prcsr::get_char_digit('d'), '3');
        assert_eq!(Prcsr::get_char_digit('n'), '5');
        assert_eq!(Prcsr::get_char_digit('r'), '6');
        assert_eq!(Prcsr::get_char_digit('w'), '9');
        assert_eq!(Prcsr::get_char_digit('e'), '0');
    }

    #[test]
    fn test_phonetic_fixed_length() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(Prcsr::fix_length(vec!['h', '4', '0']).len(), 4);
    }

    #[test]
    fn test_phonetic_pad_zeros() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(
            Prcsr::add_more_zeros(vec!['h', '4', '0']),
            vec!['h', '4', '0', '0']
        );
    }

    #[test]
    fn test_phonetics_remove_similar_char_digits() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(
            Prcsr::strip_similar_chars(vec!['h', 'e', 'l', 'l', 'o']),
            vec!['h', '4']
        );
    }

    #[test]
    fn test_phonetics_similar_word() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert!(!Prcsr::similar_word("rupert", "robert"));
        assert!(Prcsr::similar_word("Johnathan", "Jonathan"));
    }

    #[test]
    fn test_phonetics_soundex_encode() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert_eq!(
            Prcsr::soundex_encoding(vec!['h', 'e', 'l', 'l', 'o']),
            vec!['h', '4', '0', '0']
        );
    }

    #[test]
    fn test_phonetic_sounds_like() {
        struct Prcsr;
        impl Phonetic for Prcsr {}

        assert!(Prcsr::sounds_like("rupert", "robert"));
        assert!(Prcsr::sounds_like("social", "sozial"));
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

        assert_eq!(
            Prcsr::tokenize("My personal data"),
            vec!["My", "personal", "data"]
        );
        assert_eq!(
            Prcsr::tokenize(r#"{"ssn":"003-08-5546"}"#),
            vec!["ssn", "003-08-5546"]
        );
    }

    #[test]
    fn test_tfidf_frequency_counts() {
        struct FreqCnt {}
        impl Tfidf for FreqCnt {}
        let tokens = get_tokens();
        let counts = r#"{"003-67-0998": 1, "A": 1, "Hello": 1, "John": 1, "My": 1, "Never": 1, "What": 1, "a": 1, "identifier": 1, "is": 4, "my": 1, "name": 4, "personal": 1, "share": 1, "ssn": 1, "your": 2}"#;

        assert_eq!(format!("{:?}", FreqCnt::frequency_counts(tokens)), counts);
    }

    #[test]
    fn test_tfidf_frequency_counts_as_vec() {
        struct FreqCnt {}
        impl Tfidf for FreqCnt {}
        let tokens = get_tokens();
        let counts = r#"[("is", 4), ("name", 4), ("your", 2), ("003-67-0998", 1), ("A", 1), ("Hello", 1), ("John", 1), ("My", 1), ("Never", 1), ("What", 1), ("a", 1), ("identifier", 1), ("my", 1), ("personal", 1), ("share", 1), ("ssn", 1)]"#;

        assert_eq!(
            format!("{:?}", FreqCnt::frequency_counts_as_vec(tokens)),
            counts
        );
    }

    #[test]
    fn test_tfidf_tfidf() {
        struct FreqCnt {}
        impl Tfidf for FreqCnt {}
        let mut docs = Vec::new();
        let tokens_list = vec![
            vec![
                "Hello", "my", "name", "is", "John", "What", "is", "your", "name",
            ],
            vec![
                "A",
                "name",
                "is",
                "a",
                "personal",
                "identifier",
                "Never",
                "share",
                "your",
                "name",
            ],
            vec!["My", "ssn", "is", "003-67-0998"],
        ];

        for tokens in tokens_list {
            docs.push(FreqCnt::frequency_counts_as_vec(tokens));
        }

        assert_eq!(FreqCnt::tfidf("ssn", 2, docs.clone()), 1.0986122886681098);
        assert_eq!(FreqCnt::tfidf("name", 1, docs.clone()), 0.4054651081081644);
        assert_eq!(FreqCnt::tfidf("your", 1, docs), 0.3040988310811233);
    }
}
