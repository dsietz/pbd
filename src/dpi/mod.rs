
//! ### Background
//! The practice of implementing a Data Privacy Inspector addresses the following Privacy Design Strategies:
//! - Control
//! - Enforce
//!
//! Explanation goes here ...
//! 
//!
//! ### Usage
//! 
//! 

extern crate regex;

// use regex::Regex;

type KeyWordList = Vec<String>;
type KeyPatternList = Vec<String>;

trait Tokenizer {
    fn tokenize(text: &str) -> Vec<&str> {
        text.split(Self::is_match)
            .filter(|s| !s.is_empty())
            .collect()
    }

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
    fn test_tokenizer_tokenize() {
        struct Tknzr;
        impl Tokenizer for Tknzr {}

        assert_eq!(Tknzr::tokenize("My personal data"), vec!["My","personal","data"]);
        assert_eq!(Tknzr::tokenize(r#"{"ssn":"003-08-5546"}"#), vec!["ssn","003-08-5546"]);
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