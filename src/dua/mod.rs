
//! The practice of implementing Data Usage Agreements addresses the following Privacy Design Strategies:
//! - Inform
//! - Control
//! - Enforce
//! - Deomnstrate
//!
//! Whenever data is passed between Actors (e.g.: data collection between an online portal and the backend service to order the product), 
//! it is important to ensure that the owners' consent for how the data wil be used doesn't become _lost in translation_. 
//! 
//! A privacy engineering practice that supports this _promise_ to adhere the data intent is the use of Data Usage Agreements.
//! 
//! ### Usage   
//!
//!    

/// Represents a Data Usage Agreement (DUA)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DUA {
    /// The common name of the Data Usage Agreement, (e.g.: For Billing Purpose)
    pub agreement_name: String,
    /// The URI where the version of the DUA can be found, (e.g.: https://iStore.example.org/dua/v2/billing.pdf)
    pub location: String,
    /// The Unix Epoch time when the DUA was agreed to
    pub agreed_dtm: u64,
}

impl DUA {
    /// Constructs a DUA object from a serialized string
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
    /// use pbd::dua::DUA;
    ///
    /// fn main() {
    ///     let serialized = r#"{ "agreement_name": "billing", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#;
    ///     let usage_agreement = DUA::from_serialized(&serialized);
    ///     
    ///     println!("{:?}", usage_agreement);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DUA {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Serialize a DUA object
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
    /// use pbd::dua::DUA;
    ///
    /// fn main() {
    ///     let serialized = r#"{ "agreement_name": "billing", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#;
    ///     let mut dua = DUA {
    ///         agreement_name: "billing".to_string(),
    ///         location: "www.dua.org/billing.pdf".to_string(),
    ///         agreed_dtm: 1553988607,
    ///     };
    ///
    ///     let usage_agreement = dua.serialize();
    ///     
    ///     println!("{:?}", usage_agreement);
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
		serde_json::to_string(&self).unwrap()
    }
}

pub mod error;
pub mod extractor;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn get_dua() -> Vec<DUA>{
        let mut v = Vec::new();
        v.push( DUA {
                    agreement_name: "billing".to_string(),
                    location: "www.dua.org/billing.pdf".to_string(),
                    agreed_dtm: 1553988607,
                });
        v
    }

    #[test]
    fn test_dua_from_serialized_ok() {
        let serialized = r#"{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607}"#;
        let dua = DUA::from_serialized(serialized);

        assert_eq!(dua.agreement_name, "billing".to_string());
        assert_eq!(dua.location, "www.dua.org/billing.pdf".to_string());
        assert_eq!(dua.agreed_dtm, 1553988607);
    }

    #[test]
    fn test_dua_serialize_ok() {
        let serialized = r#"{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607}"#;
        let dua = &mut get_dua()[0];

        assert_eq!(dua.serialize(), serialized);
    }
}