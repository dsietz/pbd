//! ### Background
//! This module implements the `Fideslang` model and taxonomy in an effort to promote a standardized the approach of creating Data Usage Agreements.
//!
//! Credit is to be given to [fides](https://ethyca.github.io/fideslang/) and adheres to the fides licensing:
//! + [license](https://github.com/ethyca/fides/blob/main/LICENSE)
//! + [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)
//!
//! You can use the [Privacy Taxonomy Explorer](https://ethyca.github.io/fideslang/explorer/) for a graphic representation of the Fides classification groups.
//!
//!
use super::data_category::DataCategory;
// use super::data_subject;
// use super::data_use;
use std::collections::BTreeMap;
// use derive_more::Display;

/// Represents a Data Usage Policy (DUP)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DUP {
    /// The common name of the Data Usage Policy, (e.g.: For Billing Purpose)
    pub name: String,
    /// A textual description of the Data Usage Policy
    pub description: String,
    /// The version of the policy, (e.g.: 1.0.0)
    pub version: String,
    // The lists of Data Categories associated with the policy
    categories: BTreeMap<String, DataCategory>,
}

impl DUP {
    /// Constructs a new Data Usage Policy object
    ///
    /// # Arguments
    ///
    /// * nme: String - The textual name of the Data Usage Policy.</br>
    /// * descr: String - A textual description of the Data Usage Policy.</br>
    /// * ver: String - The version of the policy, (e.g.: 1.0.0).</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    ///
    /// fn main() {
    ///     let dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    /// }
    /// ```
    ///
    pub fn new(nme: String, descr: String, ver: String) -> Self {
        DUP {
            name: nme,
            description: descr,
            version: ver,
            categories: BTreeMap::new(),
        }
    }

    /// Associates a DataCategory object to the policy
    ///
    /// # Arguments
    ///
    /// * category: DataCategory - The Data Category to add.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     dup.add_category(DataCategory::new(
    ///        "Authentication Data".to_string(),
    ///        "Data used to manage access to the system.".to_string(),
    ///        "system.authentication".to_string(),
    ///        "default_organization".to_string(),
    ///        Some("system".to_string()),
    ///        None,                       
    ///        false,
    ///        true,
    ///    ));
    /// }
    /// ```
    pub fn add_category(&mut self, category: DataCategory) {
        self.categories.insert(category.fides_key.clone(), category);
    }

    /// Retrieves all the associates a DataCategory object to the policy
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     dup.add_category(DataCategory::new(
    ///        "Authentication Data".to_string(),
    ///        "Data used to manage access to the system.".to_string(),
    ///        "system.authentication".to_string(),
    ///        "default_organization".to_string(),
    ///        Some("system".to_string()),
    ///        None,                       
    ///        false,
    ///        true,
    ///    ));
    ///
    ///    assert_eq!(dup.get_categories().len(), 1);
    /// }
    /// ```
    pub fn get_categories(&mut self) -> Vec<DataCategory> {
        self.categories.clone().into_values().collect()
    }

    /// Disassociates the specified DataCategory object from the policy
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///     let cat = DataCategory::new(
    ///        "Authentication Data".to_string(),
    ///        "Data used to manage access to the system.".to_string(),
    ///        "system.authentication".to_string(),
    ///        "default_organization".to_string(),
    ///        Some("system".to_string()),
    ///        None,                       
    ///        false,
    ///        true,
    ///    );
    ///
    ///    dup.add_category(cat.clone());
    ///
    ///    dup.remove_category(cat);
    /// }
    /// ```
    pub fn remove_category(&mut self, category: DataCategory) {
        self.categories.remove(&category.fides_key);
    }

    /// Disassociates the specified DataCategory object from the policy using the fides key
    ///
    /// # Arguments
    ///
    /// * key: String - The fides kay of the Data Category to remove.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///     let cat = DataCategory::new(
    ///        "Authentication Data".to_string(),
    ///        "Data used to manage access to the system.".to_string(),
    ///        "system.authentication".to_string(),
    ///        "default_organization".to_string(),
    ///        Some("system".to_string()),
    ///        None,                       
    ///        false,
    ///        true,
    ///    );
    ///
    ///    dup.add_category(cat.clone());
    ///
    ///    dup.remove_category_by_key(cat.fides_key);
    /// }
    /// ```
    pub fn remove_category_by_key(&mut self, key: String) {
        self.categories.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data_category() -> DataCategory {
        let category = DataCategory::new(
            "Authentication Data".to_string(),
            "Data used to manage access to the system.".to_string(),
            "system.authentication".to_string(),
            "default_organization".to_string(),
            Some("system".to_string()), // parent key
            None,                       // tags
            false,
            true,
        );
        category
    }

    fn get_dup() -> DUP {
        let dup = DUP::new(
            "General Policy".to_string(),
            "This is a high-level policy.".to_string(),
            "1.0.1".to_string(),
        );
        dup
    }

    #[test]
    fn test_dup_add_remove_category_ok() {
        let mut dup = get_dup();
        dup.add_category(get_data_category());
        assert_eq!(dup.get_categories().len(), 1);

        dup.remove_category(get_data_category());
        assert_eq!(dup.get_categories().len(), 0);
    }

    #[test]
    fn test_dup_remove_category_by_key_ok() {
        let mut dup = get_dup();
        dup.add_category(get_data_category());
        assert_eq!(dup.get_categories().len(), 1);

        dup.remove_category_by_key(get_data_category().fides_key);
        assert_eq!(dup.get_categories().len(), 0);
    }
}
