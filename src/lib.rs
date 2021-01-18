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
//! >version = "0.3"
//! >default-features = false
//! >features = ["dua"]
//!  
//!
//! ##### Feature List
//!
//! | Feature              | Package  | Default | Descripotion                                 |
//! | :------------------- | :------: | :-----: | :------------------------------------------- |
//! | Data Privacy Inspector | dpi    | true    | Inspects data to determine if it contains sensative content and requires data privacy handling |
//! | Data Tracker Chain   | dtc      | true    | Auditing of the data lineage                 |
//! | Data Security Guard  | dsg      | true    | Encryption and decryption of the data        |
//! | Data Usage Agreement | dua      | true    | Management of how data is allowed to be used |
//!
//!
//!
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate derive_more;
extern crate json;
extern crate serde_json;

#[allow(dead_code)]
fn add(u: usize, i: i8) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u8 as usize
    } else {
        u + i as usize
    }
}

/// Takes a list of &str and returns a list of String
///
/// # Arguments
///
/// * list: Vec<&str> - The list of &str to convert to String.</br>
///
/// #Example
///
/// ```rust
/// use pbd::to_vec_string;
///
/// assert_eq!(
///     to_vec_string(vec!["one", "two", "three"]),
///     vec!["one".to_string(), "two".to_string(), "three".to_string()]
/// );
/// ```
#[allow(dead_code)]
pub fn to_vec_string(list: Vec<&str>) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

// Modules
#[cfg(feature = "dpi")]
pub mod dpi;
#[cfg(feature = "dsg")]
pub mod dsg;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "dua")]
pub mod dua;

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x: usize = 2;
        let y: i8 = -1;
        assert_eq!(add(x, y), 1);
    }

    #[test]
    fn test_to_vec_string() {
        assert_eq!(
            to_vec_string(vec!["one", "two", "three"]),
            vec!["one".to_string(), "two".to_string(), "three".to_string()]
        );
    }
}
