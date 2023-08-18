//! ### Background
//! The practice of implementing Data Tracker Chains addresses the following Privacy Design Strategies:
//! - Inform
//! - Control
//! - Demonstrate
//!
//! Whenever data is passed through Actors (e.g.: data collection between an online portal and the backend service to order the product),
//! it is important to ensure that data lineage is tracked and retained.
//!
//! A privacy engineering practice that supports the real-time recording of data lineage is to implement a Data Tracking Chain that lives with the data.
//!
//! ### Usage
//! Whenever the data is touched by a processor or landed in a location, we have the Tracker add a Marker its MarkerChain.
//!
//! ```
//! extern crate pbd;
//!
//! use pbd::dtc::Tracker;
//!
//! fn main() {
//!     let mut tracker = Tracker::new("purchaseId=12345".to_string());
//!     tracker.add(1578071239, "payment-validator".to_string(), "purchaseId=12345".to_string());
//!     tracker.add(1578071245, "credit-card-transaction-processor".to_string(), "purchaseId=12345".to_string());
//!
//!     println!("{}", tracker.serialize());
//! }
//! ```
//!
//! ---
//!
//! We can ensure that the MarkerChain has been tampered with outside of the Tracker's control (e.g.: `tracker.serialize()` => change the JSON => `Tracker::from_serialize()`)
//! by calling the `is_valid()` method.
//! ```
//! extern crate pbd;
//! extern crate json;
//!
//! use pbd::dtc::{Marker, Tracker};
//!
//! fn main() {
//!     let mut tracker = Tracker::new("purchaseId=12345".to_string());
//!     tracker.add(1578071239, "payment-validator".to_string(), "purchaseId=12345".to_string());
//!     tracker.add(1578071245, "credit-card-transaction-processor".to_string(), "purchaseId=12345".to_string());
//!     
//!     let mut markerchain: Vec<Marker> = serde_json::from_str(&tracker.serialize()).unwrap();
//!     markerchain[1].identifier.actor_id = "tampered data".to_string();
//!     let serialized = serde_json::to_string(&markerchain).unwrap();
//!     let tracker_tampered = Tracker::from_serialized(&serialized).unwrap();
//!     
//!     assert_eq!(Tracker::is_valid(&tracker_tampered), false);
//! }
//! ```
//!
//! ---
//!
//! We can also ensure that Data Tracker Chains are passed when working with RESTful APIs by implementing the `middleware` and `extractor` modules.
//!

extern crate base64;
extern crate pow_sha256;

use crate::dtc::error::*;
use pow_sha256::PoW;

/// The nonce value for adding complexity to the hash
pub static DIFFICULTY: u128 = 5;
/// The standard header attribute for list (array) of the Data Usage Agreements
pub static DTC_HEADER: &str = "Data-Tracker-Chain";

/// Represents a MarkerIdentifier
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkerIdentifier {
    /// The unique identifier of the the data being tracked, (e.g.: order~clothing~iStore~15150)
    pub data_id: String,
    /// The sequanece number of the Marker in the Data Tracker Chain, (e.g.: 0,1,2,3)
    pub index: usize,
    // The date and time (Unix timestamp) the data came into posession of the Actor, (1578071239)
    pub timestamp: u64,
    /// The unique identifier of the Actor who touched the data, (e.g.: notifier~billing~receipt~email)
    pub actor_id: String,
    /// The identifying hash of the previous Marker in the Data Tracker Chain
    pub previous_hash: String,
}

impl MarkerIdentifier {
    /// Serializes the MarkerIdenifier.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Marker;
    ///
    /// fn main() {
    ///     let marker = Marker::new(1, 1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string(), "hash-12345".to_string());
    ///     
    ///     println!("{}", marker.identifier.serialize());
    /// }
    /// ```
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Represents a Marker
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    /// The unique identifier of the the data being tracked, (e.g.: order~clothing~iStore~15150)
    pub identifier: MarkerIdentifier,
    /// The identifying hash of the Marker
    pub hash: String,
    /// The difficulty of the Proof of Work
    nonce: u128,
}

impl Marker {
    /// Constructs a Marker object
    ///
    /// # Arguments
    ///
    /// * idx: usize - The sequanece number of the Marker in the Data Tracker Chain, (e.g.: 0,1,2,3).</br>
    /// * tmstp: String - The date and time (Unix timestamp) the data came into posession of the Actor.</br>
    /// * act_id: String - The Unix Epoch time when the DUA was agreed to.</br>
    /// * dat_id: String - The unique identifier of the the data being tracked.</br>
    /// * prev_hash: String - The identifying hash of the previous Marker in the Data Tracker Chain</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Marker;
    ///
    /// fn main() {
    ///     let marker = Marker::new(1, 1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string(), "123456".to_string());
    ///     
    ///     println!("{} has touched the data object {}", marker.identifier.actor_id, marker.identifier.data_id);
    /// }
    /// ```
    pub fn new(
        idx: usize,
        tmstp: u64,
        act_id: String,
        dat_id: String,
        prev_hash: String,
    ) -> Marker {
        let idfy = MarkerIdentifier {
            data_id: dat_id,
            index: idx,
            timestamp: tmstp,
            actor_id: act_id,
            previous_hash: prev_hash,
        };

        Marker {
            identifier: idfy.clone(),
            hash: Marker::calculate_hash(idfy, DIFFICULTY).result,
            nonce: DIFFICULTY,
        }
    }

    fn calculate_hash(idfy: MarkerIdentifier, difficulty: u128) -> PoW<MarkerIdentifier> {
        PoW::prove_work(&idfy, difficulty).unwrap()
    }

    /// Constructs the first Marker (a.k.a. Genesis Black)
    ///
    /// # Arguments
    ///
    /// * dat_id: String - The unique identifier of the the data being tracked.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Marker;
    ///
    /// fn main() {
    ///     let marker = Marker::genesis("order~clothing~iStore~15150".to_string());
    ///     
    ///     assert_eq!(marker.identifier.index, 0);
    /// }
    /// ```
    pub fn genesis(dat_id: String) -> Marker {
        let idfy = MarkerIdentifier {
            data_id: dat_id,
            index: 0,
            timestamp: 0,
            actor_id: "".to_string(),
            previous_hash: "0".to_string(),
        };

        Marker {
            identifier: idfy.clone(),
            hash: Marker::calculate_hash(idfy, DIFFICULTY).result,
            nonce: DIFFICULTY,
        }
    }

    /// Serializes the Marker.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Marker;
    ///
    /// fn main() {
    ///     let marker = Marker::new(1, 1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string(), "hash-12345".to_string());
    ///     
    ///     println!("{}", marker.serialize());
    /// }
    /// ```
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Represents a Tacker (a.k.a. MarkerChain)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tracker {
    chain: Vec<Marker>,
}

impl Tracker {
    /// Constructs a Tracker (a.k.a. MarkerChain)
    ///
    /// # Arguments
    ///
    /// * dat_id: String - The unique identifier of the the data being tracked.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     
    ///     // The genesis Marker is automaically created for you
    ///     assert_eq!(tracker.len(), 1);
    /// }
    /// ```
    pub fn new(dat_id: String) -> Tracker {
        let mut tracker = Tracker { chain: Vec::new() };

        tracker.chain.push(Marker::genesis(dat_id));

        tracker
    }

    /// Appends a new Marker to the end of the Marker Chain.
    /// The index of the Marker and hash from the previous Marker are automatically defined when added.
    ///
    /// # Arguments
    ///
    /// * tmstp: String - The date and time (Unix timestamp) the data came into posession of the Actor.</br>
    /// * act_id: String - The unique identifier of the Actor touching the data.</br>
    /// * dat_id: String - The unique identifier of the data being tracked.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     tracker.add(1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string());
    ///     
    ///     println!("There are {} items in the Marker Chain.", tracker.len());
    /// }
    /// ```
    pub fn add(&mut self, tmstp: u64, act_id: String, dat_id: String) {
        let prior_marker = self.chain[self.chain.len() - 1].clone();
        let marker = Marker::new(self.chain.len(), tmstp, act_id, dat_id, prior_marker.hash);

        self.chain.push(marker);
    }

    /// Constructs a Tracker (a.k.a. MarkerChain) from a serialized chain
    ///
    /// # Arguments
    ///
    /// * serialized: &str - The serialized Vec of Markers.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let tracker = Tracker::from_serialized(r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":"","previous_hash":"0"},"hash":"272081696611464773728024926793703167782","nonce":5},{"identifier":{"data_id":"order~clothing~iStore~15150","index":1,"timestamp":1578071239,"actor_id":"notifier~billing~receipt~email","previous_hash":"272081696611464773728024926793703167782"},"hash":"50104149701098700632511144125867736193","nonce":5}]"#);
    ///     
    ///     // unwrap() to get the Tracker is Result is Ok
    ///     assert!(tracker.is_ok());
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> Result<Tracker, Error> {
        match serde_json::from_str(&serialized) {
            Ok(v) => Ok(Tracker { chain: v }),
            Err(_e) => Err(Error::BadChain),
        }
    }

    /// Returns the Marker from the Marker Chain at the specified index.
    ///
    /// # Arguments
    ///
    /// * index: usize - The index of the Marker.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     let marker = tracker.get(0).unwrap();
    ///     
    ///     println!("{}", marker.identifier.data_id);
    /// }
    /// ```
    pub fn get(&self, index: usize) -> Option<Marker> {
        if index < self.chain.len() {
            return Some(self.chain[index].clone());
        }

        None
    }

    /// Indicates if the Tracker's Marker Chain is empty.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     tracker.add(1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string());
    ///     
    ///     // The Tracker has two Markers: the genesis Marker when new() was called, and the one that was added
    ///     assert!(!tracker.is_empty());
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.chain.len() == 0
    }

    /// Determines if the Tracker has a valid Marker Chain, (a.k.a. not been tampered with).
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut mkrchn = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     mkrchn.add(1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string());
    ///
    ///     assert!(Tracker::is_valid(&mkrchn));
    /// }
    /// ```
    pub fn is_valid(&self) -> bool {
        debug!("Validating chain ...");

        for (m, marker) in self.chain.clone().iter().enumerate() {
            debug!("Checking Marker #{}", m);
            // make sure the Marker hasn't been altered
            if marker.hash != Marker::calculate_hash(marker.clone().identifier, DIFFICULTY).result {
                return false;
            }

            // make sure the relationship with the prior Marker hasn't been altered
            if m > 0 && marker.identifier.previous_hash != self.chain.clone()[m - 1].hash {
                return false;
            }
        }

        true
    }

    /// Returns the length of the Tracker's Marker Chain.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     tracker.add(1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string());
    ///     
    ///     // The Tracker has two Markers: the genesis Marker when new() was called, and the one that was added
    ///     assert_eq!(tracker.len(), 2);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Serializes the Tracker's Marker Chain.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dtc::Tracker;
    ///
    /// fn main() {
    ///     let mut tracker = Tracker::new("order~clothing~iStore~15150".to_string());
    ///     tracker.add(1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string());
    ///     
    ///     println!("{}", tracker.serialize());
    /// }
    /// ```
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.chain.clone()).unwrap()
    }
}

pub mod error;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn get_marker() -> Marker {
        Marker::new(
            1,
            1578071239,
            "notifier~billing~receipt~email".to_string(),
            "order~clothing~iStore~15150".to_string(),
            "123456".to_string(),
        )
    }

    #[test]
    fn test_calc_hash() {
        let _ = env_logger::builder().is_test(true).try_init();
        let marker = get_marker();
        let pw = Marker::calculate_hash(marker.identifier, marker.nonce);

        assert!(pw.is_sufficient_difficulty(marker.nonce));
    }

    #[test]
    fn test_marker_new() {
        let mkr = get_marker();
        assert_eq!(mkr.identifier.index, 1);
    }

    #[test]
    fn test_marker_genesis() {
        let mkr = Marker::genesis("order~clothing~iStore~15150".to_string());
        assert_eq!(mkr.identifier.index, 0);
    }

    #[test]
    fn test_markerchain_get() {
        let mut mkrchn = Tracker::new("order~clothing~iStore~15150".to_string());
        mkrchn.add(
            1578071239,
            "notifier~billing~receipt~email".to_string(),
            "order~clothing~iStore~15150".to_string(),
        );

        assert!(mkrchn.get(0).is_some());
        assert!(mkrchn.get(1).is_some());
        assert!(mkrchn.get(2).is_none());
    }

    #[test]
    fn test_markerchain_from_serialized() {
        let mkrchn = Tracker::from_serialized(
            r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":"","previous_hash":"0"},"hash":"272081696611464773728024926793703167782","nonce":5},{"identifier":{"data_id":"order~clothing~iStore~15150","index":1,"timestamp":1578071239,"actor_id":"notifier~billing~receipt~email","previous_hash":"272081696611464773728024926793703167782"},"hash":"50104149701098700632511144125867736193","nonce":5}]"#,
        );

        assert!(mkrchn.is_ok());
    }

    #[test]
    fn test_markerchain_new() {
        let mkrchn = Tracker::new("order~clothing~iStore~15150".to_string());
        assert_eq!(mkrchn.len(), 1);
    }

    #[test]
    fn test_markerchain_serialize() {
        let mut mkrchn = Tracker::new("order~clothing~iStore~15150".to_string());
        mkrchn.add(
            1578071239,
            "notifier~billing~receipt~email".to_string(),
            "order~clothing~iStore~15150".to_string(),
        );

        assert!(mkrchn.serialize().len() > 0);
    }

    #[test]
    fn test_markerchain_valid() {
        let mut mkrchn = Tracker::new("order~clothing~iStore~15150".to_string());

        mkrchn.add(
            1578071239,
            "notifier~billing~receipt~email".to_string(),
            "order~clothing~iStore~15150".to_string(),
        );

        assert!(Tracker::is_valid(&mkrchn));
    }

    #[test]
    fn test_markerchain_invalid() {
        let mut tracker = Tracker::new("purchaseId=12345".to_string());
        tracker.add(
            1578071239,
            "payment-validator".to_string(),
            "purchaseId=12345".to_string(),
        );
        tracker.add(
            1578071245,
            "credit-card-transaction-processor".to_string(),
            "purchaseId=12345".to_string(),
        );

        let mut markerchain: Vec<Marker> = serde_json::from_str(&tracker.serialize()).unwrap();
        markerchain[1].identifier.actor_id = "tampered data".to_string();
        let serialized = serde_json::to_string(&markerchain).unwrap();
        let tracker_tampered = Tracker::from_serialized(&serialized).unwrap();

        assert_eq!(Tracker::is_valid(&tracker_tampered), false);
    }
}
