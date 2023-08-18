//! ### Background
//! The practice of implementing Data Usage Agreements addresses the following Privacy Design Strategies:
//! - Inform
//! - Control
//! - Enforce
//! - Demonstrate
//!
//! Whenever data is passed between Actors (e.g.: data collection between an online portal and the backend service to order the product),
//! it is important to ensure that the owners' consent for how the data wil be used doesn't become _lost in translation_.
//!
//! A privacy engineering practice that supports this _promise_ to adhere how the data may be used is defined in the Data Usage Agreements
//! that are sent with the data.
//!
//! ### Usage
//! 1. The requestor adds a HTTP header `Data-Usage-Agreement` with the json array of the DUA objects
//!     
//!     > *JSON Structure*
//!     >
//!     > [
//!     >     {
//!     >         "agreement_name": String,
//!     >         "location": String,
//!     >         "agreed_dtm": Unix Epoch Number
//!     >     }
//!     > ]
//!     >
//!     > *HTTP Header*
//!     >
//!     > Data-Usage-Agreement: [{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]
//!     
//!
//!
//! ---
//!
//! One way is to incorporate the use of DUA objects is directly in the code.
//! ```
//! extern crate pbd;
//!
//! use pbd::dua::DUA;
//!
//! fn main() {
//!     let serialized = r#"{ "agreement_name": "For Billing Purpose", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#;
//!     let dua = DUA::from_serialized(&serialized);
//!
//!     match dua.agreement_name.as_ref() {
//!         "For Billing Purpose" => println!("We can use the data for sending a bill."),
//!          _ => println!("Oops: We can't use the data this way!")
//!      }
//!     
//!     // Addtionally, check which version of the agreement aligns with the agreed_dtm (if the agreement is under version control).
//! }
//! ```
//!    

/// The standard header attribute for list (array) of the Data Usage Agreements
pub static DUA_HEADER: &str = "Data-Usage-Agreement";

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
    /// Constructs a DUA object
    ///
    /// # Arguments
    ///
    /// * agreement: String - The common name of the Data Usage Agreement, (e.g.: For Billing Purpose).</br>
    /// * uri: String - The URI where the version of the DUA can be found, (e.g.: https://iStore.example.org/dua/v2/billing.pdf).</br>
    /// * agreed_on: String - The Unix Epoch time when the DUA was agreed to.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::DUA;
    ///
    /// fn main() {
    ///     let dua = DUA::new("For Billing Purpose".to_string(), "www.dua.org/billing.pdf".to_string(), 1553988607);
    ///     
    ///     match dua.agreement_name.as_ref() {
    ///         "For Billing Purpose" => println!("We can use the data for sending a bill."),
    ///         _ => println!("Oops: We can't use the data this way!")
    ///     }
    /// }
    /// ```
    pub fn new(agreement: String, uri: String, agreed_on: u64) -> DUA {
        DUA {
            agreement_name: agreement,
            location: uri,
            agreed_dtm: agreed_on,
        }
    }

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

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn get_dua() -> Vec<DUA> {
        let mut v = Vec::new();
        v.push(DUA {
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
