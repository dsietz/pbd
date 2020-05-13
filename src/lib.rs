//! 
//! Privacy by Design (PbD) is more important than ever in the industry. 
//! No matter if you're an architects, software engineers, test engineer, release manager, or business analyst,   
//! designing systems with privacy in mind is a critical part of your work. For this reason, this library provides 
//! functionality and components that help you implement PbD best practices.
//!
//!
//! #### Usage
//!
//! This crate follows the [privacy design strategies and tactics](https://github.com/dsietz/pbd/blob/master/docs/DESIGN-STRATEGIES.md) and is broken down into aligned features.
//! These features can be specified in Cargo.toml as a dependency.
//!
//! >[dependencies.pbd]
//! >version = "0.0.5"
//! >default-features = false
//! >features = ["dua"]
//!  
//!
//! ##### Feature List 
//! 
//! | Feature              | Package  | Default | Descripotion                                 | 
//! | :------------------- | :------: | :-----: | :------------------------------------------- |
//! | Data Tracker Chain   | dtc      | true    | Auditing of the data lineage                 |
//! | Data Security Guard  | dsg      | true    | Encryption and decryption of the data        |
//! | Data Usage Agreement | dua      | true    | Management of how data is allowed to be used |
//! 
//!
//!
extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate derive_more;
extern crate json;


// Modules
#[cfg(feature = "dua")]
pub mod dua;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "dsg")]
pub mod dsg;

// Unit Tests
#[cfg(test)]
mod tests {
}
