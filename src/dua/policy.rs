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
use super::data_subject::DataSubject;
use super::data_use::DataUse;
use derive_more::Display;
use std::collections::BTreeMap;

/// An Enum of any possible item keys that can be associated to a policy
#[derive(Display, Clone)]
pub enum Condition {
    Category(String),
    Subject(String),
    Use(String),
}

// impl Condition {
//     ///
//     pub fn get_key(&self) -> String {
//         match self {
//             Condition::Category(String) => self.fides_key,
//             Condition::Subject(String) => self.get_key(),
//             Condition::Use(String) => self.get_key(),
//         }
//     }
// }
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
    // The lists of Data Subjects associated with the policy
    subjects: BTreeMap<String, DataSubject>,
    // The lists of Data Uses associated with the policy
    uses: BTreeMap<String, DataUse>,
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
    /// ```rust
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
            subjects: BTreeMap::new(),
            uses: BTreeMap::new(),
        }
    }

    /// Associates a DataCategory object to the policy
    /// __NOTE__: Call this function to associate a new DataCategory objects or replace pre-associated DataCategory objects
    ///
    /// # Arguments
    ///
    /// * category: DataCategory - The Data Category to associate.</br>
    ///
    /// #Example
    ///
    /// ```rust
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
    ///     dup.associate_category(DataCategory::new(
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
    pub fn associate_category(&mut self, category: DataCategory) {
        self.categories.insert(category.get_key().clone(), category);
    }

    /// Associates a DataSubject object to the policy
    /// __NOTE__: Call this function to associate a new DataSubject objects or replace pre-associated DataSubject objects
    ///
    /// # Arguments
    ///
    /// * subject: DataSubject - The Data Subject to associate.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///
    ///     dup.associate_subject(subject);
    /// }
    /// ```
    pub fn associate_subject(&mut self, subject: DataSubject) {
        self.subjects.insert(subject.get_key().clone(), subject);
    }

    /// Associates a DataUse object to the policy
    /// __NOTE__: Call this function to associate a new DataUse objects or replace pre-associated DataUse objects
    ///
    /// # Arguments
    ///
    /// * usage: DataUse - The Data Use to associate.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///     dup.associate_use(datause);
    /// }
    /// ```
    pub fn associate_use(&mut self, usage: DataUse) {
        self.uses.insert(usage.get_key().clone(), usage);
    }

    /// Disassociates the specified DataCategory object from the policy using the key
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Category to disassociate.</br>
    ///
    /// #Example
    ///
    /// ```rust
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
    ///    dup.associate_category(cat.clone());
    ///
    ///    dup.disassociate_category(cat.get_key());
    /// }
    /// ```
    pub fn disassociate_category(&mut self, key: String) {
        self.categories.remove(&key);
    }

    /// Disassociates the specified DataSubject object from the policy using the key
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Subject to disassociate.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_subject(subject.clone());
    ///
    ///    dup.disassociate_subject(subject.get_key());
    /// }
    /// ```
    pub fn disassociate_subject(&mut self, key: String) {
        self.subjects.remove(&key);
    }

    /// Disassociates the specified DataUse object from the policy using the key
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Use to disassociate.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_use(datause.clone());
    ///
    ///    dup.disassociate_use(datause.get_key());
    /// }
    /// ```
    pub fn disassociate_use(&mut self, key: String) {
        self.uses.remove(&key);
    }

    /// Retrieves all the associated DataCategory objects
    ///
    /// #Example
    ///
    /// ```rust
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
    ///     dup.associate_category(DataCategory::new(
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

    /// Retrieves all the associated DataSubject objects
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///
    ///     dup.associate_subject(subject);
    ///
    ///    assert_eq!(dup.get_subjects().len(), 1);
    /// }
    /// ```
    pub fn get_subjects(&mut self) -> Vec<DataSubject> {
        self.subjects.clone().into_values().collect()
    }

    /// Retrieves all the associated DataUse objects
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///     dup.associate_use(datause);
    ///
    ///    assert_eq!(dup.get_uses().len(), 1);
    /// }
    /// ```
    pub fn get_uses(&mut self) -> Vec<DataUse> {
        self.uses.clone().into_values().collect()
    }

    /// Retrieves a reference to the specified DataCategory that is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Category to retrieve.</br>
    ///
    /// #Example
    ///
    /// ```rust
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
    ///    dup.associate_category(cat.clone());
    ///
    ///    let retrieved_category = dup.get_category(cat.get_key()).unwrap();
    ///    println!("{}", retrieved_category.description);
    /// }
    /// ```
    pub fn get_category(&mut self, key: String) -> Option<&DataCategory> {
        self.categories.get(&key)
    }

    /// Retrieves a reference to the specified DataSubject that is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Subject to retrieve.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_subject(subject.clone());
    ///
    ///    let retrieved_subject = dup.get_subject(subject.get_key()).unwrap();
    ///    println!("{}", retrieved_subject.description);
    /// }
    /// ```
    pub fn get_subject(&mut self, key: String) -> Option<&DataSubject> {
        self.subjects.get(&key)
    }

    /// Retrieves a reference to the specified DataUse that is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Use to retrieve.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_use(datause.clone());
    ///
    ///    let retrieved_use = dup.get_use(datause.get_key()).unwrap();
    ///    println!("{}", retrieved_use.description);
    /// }
    /// ```
    pub fn get_use(&mut self, key: String) -> Option<&DataUse> {
        self.uses.get(&key)
    }

    /// Determines if the specified DataCategory key is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Category to check.</br>
    ///
    /// #Example
    ///
    /// ```rust
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
    ///    dup.associate_category(cat.clone());
    ///
    ///    assert_eq!(dup.has_category(cat.get_key()), true);
    /// }
    /// ```
    pub fn has_category(&mut self, key: String) -> bool {
        self.categories.contains_key(&key)
    }

    /// Determines if the specified DataSubejct key is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Subject to check.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_subject(subject.clone());
    ///
    ///    assert_eq!(dup.has_subject(subject.get_key()), true);
    /// }
    /// ```
    pub fn has_subject(&mut self, key: String) -> bool {
        self.subjects.contains_key(&key)
    }

    /// Determines if the specified DataUse key is associated with the policy
    ///
    /// # Arguments
    ///
    /// * key: String - The key of the Data Use to check.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_use(datause.clone());
    ///
    ///    assert_eq!(dup.has_use(datause.get_key()), true);
    /// }
    /// ```
    pub fn has_use(&mut self, key: String) -> bool {
        self.uses.contains_key(&key)
    }

    /// Determines if the specified Conditions can be met by the policy and returns a list of conditions that conflict wiht the policy.
    ///
    /// # Arguments
    ///
    /// * conditions: Vec<Condition> - The list of Conditions to check against the policy.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::{Condition, DUP};
    /// use pbd::dua::data_category::DataCategory;
    /// use pbd::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///     let category = DataCategory::new(
    ///        "Authentication Data".to_string(),
    ///        "Data used to manage access to the system.".to_string(),
    ///        "system.authentication".to_string(),
    ///        "default_organization".to_string(),
    ///        Some("system".to_string()),
    ///        None,                       
    ///        false,
    ///        true,
    ///     );
    ///     let subject = DataSubject::new(
    ///         "Consultant".to_string(),
    ///         "An individual employed in a consultative/temporary capacity by the organization.".to_string(),
    ///         "consultant".to_string(),
    ///         "default_organization".to_string(),
    ///         Some(vec!["work".to_string(), "temporary".to_string()]),
    ///         Some(DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access])),
    ///         false,
    ///         false,
    ///         true
    ///     );
    ///     let datause = DataUse::new(
    ///         "Provide the capability".to_string(),
    ///         "Provide, give, or make available the product, service, application or system.".to_string(),
    ///         "provide".to_string(),
    ///         "default_organization".to_string(),
    ///         None,
    ///         Some(LegalBasis::LegitimateInterest),
    ///         Some(SpecialCategory::VitalInterests),
    ///         Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
    ///         false,
    ///         Some("https://example.org/legitimate_interest_assessment".to_string()),
    ///         None,
    ///         false,
    ///         true
    ///     );
    ///
    ///    dup.associate_category(category.clone());
    ///    dup.associate_use(datause.clone());
    ///
    ///    let mut conditions: Vec<Condition> = Vec::new();
    ///    conditions.push(Condition::Category(category.get_key()));
    ///    conditions.push(Condition::Subject(subject.get_key()));
    ///    conditions.push(Condition::Use(datause.get_key()));
    ///    let conflicts = dup.match_conditions(conditions);
    ///
    ///    assert_eq!(conflicts.len(), 1);
    ///    assert_eq!(conflicts[0].to_string(), subject.get_key());
    /// }
    /// ```
    pub fn match_conditions(&mut self, conditions: Vec<Condition>) -> Vec<Condition> {
        let mut conflicts = Vec::new();
        for condition in conditions.into_iter() {
            match condition.clone() {
                Condition::Category(String) => {
                    match self.has_category(condition.to_string()) {
                        false => conflicts.push(condition),
                        true => {}
                    };
                }
                Condition::Subject(String) => {
                    match self.has_subject(condition.to_string()) {
                        false => conflicts.push(condition),
                        true => {}
                    };
                }
                Condition::Use(String) => {
                    match self.has_use(condition.to_string()) {
                        false => conflicts.push(condition),
                        true => {}
                    };
                }
            }
        }

        conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dua::data_subject::{DataRights, DataSubject, Right, Strategy};
    use crate::dua::data_use::{DataUse, LegalBasis, SpecialCategory};

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

    fn get_data_subject() -> DataSubject {
        let subject = DataSubject::new(
            "Consultant".to_string(),
            "An individual employed in a consultative/temporary capacity by the organization."
                .to_string(),
            "consultant".to_string(),
            "default_organization".to_string(),
            Some(vec!["work".to_string(), "temporary".to_string()]),
            Some(DataRights::new(
                Strategy::ALL,
                vec![Right::Informed, Right::Access],
            )),
            false,
            false,
            true,
        );
        subject
    }

    fn get_data_use() -> DataUse {
        let datause = DataUse::new(
            "Provide the capability".to_string(),
            "Provide, give, or make available the product, service, application or system."
                .to_string(),
            "provide".to_string(),
            "default_organization".to_string(),
            None,
            Some(LegalBasis::LegitimateInterest),
            Some(SpecialCategory::VitalInterests),
            Some(vec![
                "marketing team".to_string(),
                "dog shelter".to_string(),
            ]),
            false,
            Some("https://example.org/legitimate_interest_assessment".to_string()),
            None,
            false,
            true,
        );
        datause
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
    fn test_dup_associate_category_ok() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());
        assert_eq!(dup.get_categories().len(), 1);
    }

    #[test]
    fn test_dup_associate_subject_ok() {
        let mut dup = get_dup();
        dup.associate_subject(get_data_subject());
        assert_eq!(dup.get_subjects().len(), 1);
    }

    #[test]
    fn test_dup_associate_use_ok() {
        let mut dup = get_dup();
        dup.associate_use(get_data_use());
        assert_eq!(dup.get_uses().len(), 1);
    }

    #[test]
    fn test_dup_disassociate_category_ok() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());
        assert_eq!(dup.get_categories().len(), 1);

        dup.disassociate_category(get_data_category().get_key());
        assert_eq!(dup.get_categories().len(), 0);
    }

    #[test]
    fn test_dup_disassociate_subject_ok() {
        let mut dup = get_dup();
        dup.associate_subject(get_data_subject());
        assert_eq!(dup.get_subjects().len(), 1);

        dup.disassociate_subject(get_data_subject().get_key());
        assert_eq!(dup.get_subjects().len(), 0);
    }

    #[test]
    fn test_dup_disassociate_use_ok() {
        let mut dup = get_dup();
        dup.associate_use(get_data_use());
        assert_eq!(dup.get_uses().len(), 1);

        dup.disassociate_use(get_data_use().get_key());
        assert_eq!(dup.get_uses().len(), 0);
    }

    #[test]
    fn test_dup_get_category_ok() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());

        let cat2 = dup.get_category(get_data_category().get_key()).unwrap();
        assert_eq!(cat2.description, get_data_category().description);
    }

    #[test]
    fn test_dup_get_subject_ok() {
        let mut dup = get_dup();
        dup.associate_subject(get_data_subject());

        let sub2 = dup.get_subject(get_data_subject().get_key()).unwrap();
        assert_eq!(sub2.description, get_data_subject().description);
    }

    #[test]
    fn test_dup_get_use_ok() {
        let mut dup = get_dup();
        dup.associate_use(get_data_use());

        let use2 = dup.get_use(get_data_use().get_key()).unwrap();
        assert_eq!(use2.description, get_data_use().description);
    }

    #[test]
    fn test_dup_has_category_ok() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());
        assert_eq!(dup.has_category(get_data_category().get_key()), true);
    }

    #[test]
    fn test_dup_has_subject_ok() {
        let mut dup = get_dup();
        dup.associate_subject(get_data_subject());
        assert_eq!(dup.has_subject(get_data_subject().get_key()), true);
    }

    #[test]
    fn test_dup_has_use_ok() {
        let mut dup = get_dup();
        dup.associate_use(get_data_use());
        assert_eq!(dup.has_use(get_data_use().get_key()), true);
    }

    #[test]
    fn test_dup_match_conditions_all_found() {
        let mut dup = get_dup();
        let mut conditions: Vec<Condition> = Vec::new();
        conditions.push(Condition::Category(get_data_category().get_key()));
        conditions.push(Condition::Subject(get_data_subject().get_key()));
        conditions.push(Condition::Use(get_data_use().get_key()));
        let conflicts = dup.match_conditions(conditions);
        assert_eq!(conflicts.len(), 3);
    }

    #[test]
    fn test_dup_match_conditions_none_found() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());
        dup.associate_subject(get_data_subject());
        dup.associate_use(get_data_use());

        let mut conditions: Vec<Condition> = Vec::new();
        conditions.push(Condition::Category(get_data_category().get_key()));
        conditions.push(Condition::Subject(get_data_subject().get_key()));
        conditions.push(Condition::Use(get_data_use().get_key()));
        let conflicts = dup.match_conditions(conditions);
        assert_eq!(conflicts.len(), 0);
    }

    #[test]
    fn test_dup_match_conditions_some_found() {
        let mut dup = get_dup();
        dup.associate_category(get_data_category());
        dup.associate_use(get_data_use());

        let mut conditions: Vec<Condition> = Vec::new();
        conditions.push(Condition::Category(get_data_category().get_key()));
        conditions.push(Condition::Subject(get_data_subject().get_key()));
        conditions.push(Condition::Use(get_data_use().get_key()));
        let conflicts = dup.match_conditions(conditions);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].to_string(), get_data_subject().get_key());
    }
}
