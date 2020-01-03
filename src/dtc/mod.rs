//! ### Background
//! The practice of implementing Data Tracker Chains addresses the following Privacy Design Strategies:
//! - Inform
//! - Control
//! - Demonstrate
//!
//! Whenever data is passed through Actors (e.g.: data collection between an online portal and the backend service to order the product), 
//! it is important to ensure that data lineage is tracked and retained. 
//! 
//! A privacy engineering practice that supports the real-time recording of data ineage is to implement a Data Tracking Chain that lives with the data.
//!
//! ### Usage

extern crate pow_sha256;

use pow_sha256::PoW;

pub static DIFFICULTY: u128 = 5; 

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
}

impl MarkerIdentifier {
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
    /// The identifying hash of the previous Marker in the Data Tracker Chain
    pub previous_hash: String,
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
    pub fn new(idx: usize, tmstp: u64, act_id: String, dat_id: String, prev_hash: String) -> Marker {
        let idfy = MarkerIdentifier {
            data_id: dat_id,
            index: idx,
            timestamp: tmstp,
            actor_id: act_id,
        };

        Marker {
            identifier: idfy.clone(),
            hash: Marker::calculate_hash(idfy, DIFFICULTY).result,
            previous_hash: prev_hash,            
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
        };
        
        Marker {
            identifier: idfy.clone(),
            hash: Marker::calculate_hash(idfy, DIFFICULTY).result,
            previous_hash: "0".to_string(),            
            nonce: DIFFICULTY,
        }
    }
}

/// Represents a MarkerChain
type Tracker = Vec<Marker>;

trait MarkerChain {
    fn is_valid(&self) -> bool{
        debug!("Validating chain ...");

        for m in 0..self.len() {
            let marker = self.get(m);

            if marker.hash != Marker::calculate_hash(marker.identifier, DIFFICULTY).result {
                return false;
            }
        }

        true
    }
}

impl MarkerChain for Tracker {
}



pub mod error;


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn get_marker() -> Marker {
        Marker::new(1, 1578071239, "notifier~billing~receipt~email".to_string(), "order~clothing~iStore~15150".to_string(), "123456".to_string())
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
    fn test_markerchain_new() {
        let mut mkrchn = Tracker::new();
        mkrchn.push(Marker::genesis("order~clothing~iStore~15150".to_string()));

        assert_eq!(mkrchn.len(), 1);
    }

    #[test]
    fn test_markerchain_invalid() {
        let mut mkrchn = Tracker::new();
        mkrchn.push(Marker::genesis("order~clothing~iStore~15150".to_string()));
        mkrchn.push(get_marker());

        assert!(mkrchn.is_valid());
    }
}