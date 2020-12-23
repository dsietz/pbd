
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

type KeyWordList = Vec<String>;
type KeyPatternList = Vec<String>;

/// Represents a Data Provacy Inspector (DPI)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DPI {
    key_words: KeyWordList,
    key_patterns: KeyPatternList,
}

impl DPI {
    /// Constructs a DPI object
    /// 
    /// # Arguments
    /// 
    /// * parameter1: String - The ....</br>
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
    ///     
    ///     match dpi.agreement_name.as_ref() {
    ///         "For Billing Purpose" => println!("We can use the data for sending a bill."),
    ///         _ => println!("Oops: We can't use the data this way!")
    ///     }
    /// }
    /// ```
    pub fn new(agreement: String, uri: String, agreed_on: u64) -> DPI {
        DPI {
            key_words: Vec::new(),
            key_patterns: Vec::new(),
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
    ///     let serialized = r#"{ "agreement_name": "billing", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#;
    ///     let usage_agreement = DUA::from_serialized(&serialized);
    ///     
    ///     println!("{:?}", usage_agreement);
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
    ///     let serialized = r#"{ "agreement_name": "billing", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#;
    ///     let mut dpi = DPI {
    ///         agreement_name: "billing".to_string(),
    ///         location: "www.dua.org/billing.pdf".to_string(),
    ///         agreed_dtm: 1553988607,
    ///     };
    ///
    ///     let data_privacy_inspector = dpi.serialize();
    ///     
    ///     println!("{:?}", data_privacy_inspector);
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
                    agreement_name: "billing".to_string(),
                    location: "www.dua.org/billing.pdf".to_string(),
                    agreed_dtm: 1553988607,
                });
        v
    }

    #[test]
    fn test_dua_from_serialized_ok() {
        let serialized = r#"{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607}"#;
        let dpi = DPI::from_serialized(serialized);

        assert_eq!(dpi.agreement_name, "billing".to_string());
    }

    #[test]
    fn test_dpi_serialize_ok() {
        let serialized = r#"{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607}"#;
        let dpi = &mut get_dpi()[0];

        assert_eq!(dpi.serialize(), serialized);
    }
}