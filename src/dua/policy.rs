//! ### Background
//! This module implements the `Fideslang` model and taxonomy in an effort to promote a standardized the approach of creating Data Usage Agreements.
//!
//! Credit is to be given to [fides](https://ethyca.github.io/fideslang/) and adheres to the fides licensing:
//! + [license](https://github.com/ethyca/fides/blob/main/LICENSE)
//! + [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)
//!
//! You can use the [Privacy Taxonomy Explorer](https://ethyca.github.io/fideslang/explorer/) for a graphic representation of the Fides classification groups.
//!
//! ### Usage
//! A DUP (Data Usage Policy) allows us to define standardized usage policies that can be easily understood by a user, while also being able to apply it programmatically during the application runtime.
//! These "Opt-In" policies can then be added to the DUA (Data Usage Agreement) so that applications and processors (e.g.: microservices) can dynamically determine if and how they are permitted to utilize the data prior to processing it.
//!
//! ```rust
//! extern crate pbd;
//!
//! use pbd::dua::policy::{Condition, DUP};
//! use pbd::dua::data_category::DataCategoryFactory;
//! use pbd::dua::data_subject::DataSubjectFactory;
//! use pbd::dua::data_use::DataUseFactory;
//!
//! fn get_defined_policy() -> DUP {
//!    let category_factory = DataCategoryFactory::new();
//!    let subject_factory = DataSubjectFactory::new();
//!    let use_factory = DataUseFactory::new();
//!
//!    let mut dup = DUP::new(
//!        "General Policy".to_string(),
//!        "This is a high-level policy.".to_string(),
//!        "1.0.1".to_string()
//!    );    
//!
//!    // associate some classifications to the policy
//!    dup.associate_category(category_factory.get_category_by_key("system.authentication".to_string()).unwrap());
//!    dup.associate_category(category_factory.get_category_by_key("user.contact.email".to_string()).unwrap());
//!    dup.associate_category(category_factory.get_category_by_key("user.contact.phone_number".to_string()).unwrap());
//!    dup.associate_subject(subject_factory.get_subject_by_key("customer".to_string()).unwrap());
//!    dup.associate_use(use_factory.get_use_by_key("essential.service.authentication".to_string()).unwrap());
//!
//!    dup
//! }
//!
//! fn get_processor_conditions() -> Vec<Condition> {
//!    let mut conditions: Vec<Condition> = Vec::new();
//!    let category_factory = DataCategoryFactory::new();
//!    let subject_factory = DataSubjectFactory::new();
//!    let use_factory = DataUseFactory::new();
//!    
//!    conditions.push(Condition::Category(category_factory.get_category_by_key("user.contact.email".to_string()).unwrap().get_key()));
//!    conditions.push(Condition::Subject(subject_factory.get_subject_by_key("customer".to_string()).unwrap().get_key()));
//!    conditions.push(Condition::Use(use_factory.get_use_by_key("marketing.advertising.profiling".to_string()).unwrap().get_key()));
//!
//!    conditions
//! }
//!
//! fn main() {
//!    // A policy that defines the acceptable conditions for using the data
//!    let mut policy = get_defined_policy();
//!
//!    // A list of conditions that the processor has been configured to apply to data
//!    let conditions = get_processor_conditions();
//!
//!    // Check to see if the processor is permitted to use the data based on its privacy configurations
//!    let conflicts = policy.match_conditions(conditions);
//!
//!    match conflicts.len() > 0 {
//!       true => {
//!          for conflict in conflicts.iter() {
//!             println!("Blocked due to Condition key {}", conflict.to_string());
//!          }
//!       },
//!       false => println!("Allowed - Process the data."),
//!    }
//! }
//! ```
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

    fn readable_description(&mut self, mut policy: String, line_feed: &str) -> String {
        // Data Subjects
        policy.push_str("Data will be collected from ");
        match self.get_subjects().len() {
            0 => {
                policy.push_str("all types of users.");
            }
            _ => {
                policy.push_str("the following types of users: ");
                let count = self.get_subjects().len();
                for (idx, subject) in self.get_subjects().iter().enumerate() {
                    policy.push_str(&subject.name);
                    let delimiter = match idx < count - 2 {
                        true => ", ",
                        false => match idx == count - 1 {
                            true => ".",
                            false => " and ",
                        },
                    };
                    policy.push_str(delimiter);
                }
                policy.push_str(line_feed);
            }
        }

        // Data Categories
        policy.push_str("The data being collected will be ");
        match self.get_categories().len() {
            0 => {
                policy.push_str("include all types of data.");
            }
            _ => {
                policy.push_str("limited to the following data: ");
                let count = self.get_categories().len();
                for (idx, category) in self.get_categories().iter().enumerate() {
                    policy.push_str(&category.name);
                    let delimiter = match idx < count - 2 {
                        true => ", ",
                        false => match idx == count - 1 {
                            true => ".",
                            false => " and ",
                        },
                    };
                    policy.push_str(delimiter);
                }
                policy.push_str(line_feed);
            }
        }

        // Data Uses
        policy.push_str("The data collected can be used for ");
        match self.get_uses().len() {
            0 => {
                policy.push_str("various purposes.");
            }
            _ => {
                policy.push_str("the following purposes: ");
                let count = self.get_uses().len();
                for (idx, usage) in self.get_uses().iter().enumerate() {
                    policy.push_str(&usage.name);
                    let delimiter = match idx < count - 2 {
                        true => ", ",
                        false => match idx == count - 1 {
                            true => ".",
                            false => " and ",
                        },
                    };
                    policy.push_str(delimiter);
                }
                policy.push_str(line_feed);
            }
        }

        return policy;
    }

    /// Converts the policy to a human readable format as html
    /// _NOTE:_ You can apply custom styling by referencing the `class` attribute of the elements.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategoryFactory;
    /// use pbd::dua::data_subject::DataSubjectFactory;
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let cfactory = DataCategoryFactory::new();
    ///     let sfactory = DataSubjectFactory::new();
    ///     let ufactory = DataUseFactory::new();
    ///     let mut dup = DUP::new(
    ///         "General Marketing Policy".to_string(),
    ///         "This policy explains the manner in which your data will be used for marketing purposes.".to_string(),
    ///         "1.0.0".to_string()
    ///     );
    ///
    ///     dup.associate_category(
    ///         cfactory
    ///             .get_category_by_key("user.behavior.browsing_history".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_category(
    ///         cfactory
    ///             .get_category_by_key("user.behavior.media_consumption".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_subject(sfactory.get_subject_by_key("customer".to_string()).unwrap());
    ///     dup.associate_subject(sfactory.get_subject_by_key("prospect".to_string()).unwrap());
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.advertising.profiling".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.advertising.serving".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.communications.email".to_string())
    ///             .unwrap(),
    ///     );
    ///     
    ///     print!("{}", dup.as_html());
    ///
    ///     /* <div class='dup' name='General Policy'>General Policy</div></br>
    ///      * <div class='dup-verion' name='1.0.1'>(version: 1.0.1)</div></br></br>
    ///      * <div class='dup-description' name='General Policy Description'><b>This is a high-level policy.</b></br></br>
    ///      * <p>
    ///      * Data will be collected from the following types of users: Customer and Prospect.</br>
    ///      * The data being collected will be limited to the following data: Browsing History and Media Consumption.</br>
    ///      * The data collected can be used for the following purposes: Profiling for Advertising, Essential for Serving Ads and Marketing Email Communications.</br>
    ///      * </p></div>
    ///     */
    /// }
    /// ```
    pub fn as_html(&mut self) -> String {
        let line_feed = "</br>";
        let mut policy = String::new();
        policy.push_str("<div class='dup' name='");
        policy.push_str(&self.name);
        policy.push_str("'>");
        policy.push_str(&self.name);
        policy.push_str("</div>");
        policy.push_str(line_feed);
        policy.push_str("<div class='dup-verion' name='");
        policy.push_str(&self.version);
        policy.push_str("'>");
        policy.push_str("(version: ");
        policy.push_str(&self.version);
        policy.push_str(")");
        policy.push_str("</div>");
        policy.push_str(line_feed);
        policy.push_str(line_feed);
        policy.push_str("<div class='dup-description' name='");
        policy.push_str(&self.name);
        policy.push_str(" Description'><b>");
        policy.push_str(&self.description);
        policy.push_str("</b>");
        policy.push_str(line_feed);
        policy.push_str(line_feed);
        policy.push_str("<p>");

        policy = self.readable_description(policy, line_feed);
        policy.push_str("</p>");
        policy.push_str("</div>");

        policy
    }

    /// Converts the policy to a human readable format as text
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    /// use pbd::dua::data_category::DataCategoryFactory;
    /// use pbd::dua::data_subject::DataSubjectFactory;
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let cfactory = DataCategoryFactory::new();
    ///     let sfactory = DataSubjectFactory::new();
    ///     let ufactory = DataUseFactory::new();
    ///     let mut dup = DUP::new(
    ///         "General Marketing Policy".to_string(),
    ///         "This policy explains the manner in which your data will be used for marketing purposes.".to_string(),
    ///         "1.0.0".to_string()
    ///     );
    ///
    ///     dup.associate_category(
    ///         cfactory
    ///             .get_category_by_key("user.behavior.browsing_history".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_category(
    ///         cfactory
    ///             .get_category_by_key("user.behavior.media_consumption".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_subject(sfactory.get_subject_by_key("customer".to_string()).unwrap());
    ///     dup.associate_subject(sfactory.get_subject_by_key("prospect".to_string()).unwrap());
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.advertising.profiling".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.advertising.serving".to_string())
    ///             .unwrap(),
    ///     );
    ///     dup.associate_use(
    ///         ufactory
    ///             .get_use_by_key("marketing.communications.email".to_string())
    ///             .unwrap(),
    ///     );
    ///     
    ///     print!("{}", dup.as_text());
    ///
    ///     /* General Marketing Policy
    ///      * (version: 1.0.0)
    ///      *
    ///      * This policy explains the manner in which your data will be used for marketing purposes.
    ///      *
    ///      * Data will be collected from the following types of users: Customer and Prospect.
    ///      * The data being collected will be limited to the following data: Browsing History and Media Consumption.
    ///      * The data collected can be used for the following purposes: Profiling for Advertising, Essential for Serving Ads and Marketing Email Communications.
    ///     */
    /// }
    /// ```
    pub fn as_text(&mut self) -> String {
        let line_feed = "\r\n";
        let mut policy = String::new();
        policy.push_str(&self.name);
        policy.push_str(line_feed);
        policy.push_str("(version: ");
        policy.push_str(&self.version);
        policy.push_str(")");
        policy.push_str(line_feed);
        policy.push_str(line_feed);

        policy.push_str(&self.description);
        policy.push_str(line_feed);
        policy.push_str(line_feed);

        policy = self.readable_description(policy, line_feed);

        policy
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

    /// Constructs a DUP object from a serialized string
    ///
    /// # Arguments
    ///
    /// * serialized: &str - The string that represents the serialized object.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::DUP;
    ///
    /// fn main() {
    ///     let serialized = r#"{"name":"General Policy","description":"This is a high-level policy.","version":"1.0.1","categories":{"system.authentication":{"name":"Authentication Data","description":"Data used to manage access to the system.","fides_key":"system.authentication","organization_fides_key":"default_organization","parent_key":"system","tags":null,"is_default":true,"active":true}},"subjects":{"consultant":{"name":"Consultant","description":"An individual employed in a consultative/temporary capacity by the organization.","fides_key":"consultant","organization_fides_key":"default_organization","tags":null,"rights":null,"automated_decisions_or_profiling":false,"is_default":true,"active":true}},"uses":{"essential.service.authentication":{"name":"Essential Service Authentication","description":"Authenticate users to the product, service, application or system.","fides_key":"essential.service.authentication","organization_fides_key":"default_organization","parent_key":"essential.service","legal_basis":null,"special_category":null,"recipent":null,"legitimate_interest":false,"legitimate_interest_impact_assessment":null,"tags":null,"is_default":true,"active":true}}}"#;
    ///     let mut dup = DUP::from_serialized(&serialized);
    ///     
    ///     assert_eq!(dup.get_categories().len(), 1);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DUP {
        serde_json::from_str(&serialized).unwrap()
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

    /// Serialize a DUP object
    ///
    /// # Arguments
    ///
    /// * serialized: &str - The string that represents the serialized object.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::policy::{Condition, DUP};
    /// use pbd::dua::data_category::DataCategoryFactory;
    /// use pbd::dua::data_subject::DataSubjectFactory;
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let mut dup = DUP::new(
    ///         "General Policy".to_string(),
    ///         "This is a high-level policy.".to_string(),
    ///         "1.0.1".to_string()
    ///     );
    ///     let category_factory = DataCategoryFactory::new();
    ///     let subject_factory = DataSubjectFactory::new();
    ///     let use_factory = DataUseFactory::new();
    ///
    ///    dup.associate_category(category_factory.get_category_by_key("system.authentication".to_string()).unwrap());
    ///    dup.associate_subject(subject_factory.get_subject_by_key("consultant".to_string()).unwrap());
    ///    dup.associate_use(use_factory.get_use_by_key("analytics.reporting".to_string()).unwrap());
    ///     
    ///     println!("{:?}", dup.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dua::data_category::DataCategoryFactory;
    use crate::dua::data_subject::DataSubjectFactory;
    use crate::dua::data_use::DataUseFactory;
    use std::fs::File;
    use std::io::prelude::*;

    fn get_data_category() -> DataCategory {
        let factory = DataCategoryFactory::new();
        factory
            .get_category_by_key("system.authentication".to_string())
            .unwrap()
    }

    fn get_data_subject() -> DataSubject {
        let factory = DataSubjectFactory::new();
        factory
            .get_subject_by_key("consultant".to_string())
            .unwrap()
    }

    fn get_data_use() -> DataUse {
        let factory = DataUseFactory::new();
        factory
            .get_use_by_key("essential.service.authentication".to_string())
            .unwrap()
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
    fn test_dup_as_html() {
        let cfactory = DataCategoryFactory::new();
        let sfactory = DataSubjectFactory::new();
        let ufactory = DataUseFactory::new();
        let mut dup = get_dup();

        dup.associate_category(
            cfactory
                .get_category_by_key("user.behavior.browsing_history".to_string())
                .unwrap(),
        );
        dup.associate_category(
            cfactory
                .get_category_by_key("user.behavior.media_consumption".to_string())
                .unwrap(),
        );
        dup.associate_subject(sfactory.get_subject_by_key("customer".to_string()).unwrap());
        dup.associate_subject(sfactory.get_subject_by_key("prospect".to_string()).unwrap());
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.advertising.profiling".to_string())
                .unwrap(),
        );
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.advertising.serving".to_string())
                .unwrap(),
        );
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.communications.email".to_string())
                .unwrap(),
        );

        print!("{}", dup.as_html());
        let mut file = File::create("./tests/output/policy.html").unwrap();
        file.write_all(dup.as_html().as_bytes()).unwrap();
    }

    #[test]
    fn test_dup_as_text() {
        let cfactory = DataCategoryFactory::new();
        let sfactory = DataSubjectFactory::new();
        let ufactory = DataUseFactory::new();
        let mut dup = get_dup();

        dup.associate_category(
            cfactory
                .get_category_by_key("user.behavior.browsing_history".to_string())
                .unwrap(),
        );
        dup.associate_category(
            cfactory
                .get_category_by_key("user.behavior.media_consumption".to_string())
                .unwrap(),
        );
        dup.associate_subject(sfactory.get_subject_by_key("customer".to_string()).unwrap());
        dup.associate_subject(sfactory.get_subject_by_key("prospect".to_string()).unwrap());
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.advertising.profiling".to_string())
                .unwrap(),
        );
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.advertising.serving".to_string())
                .unwrap(),
        );
        dup.associate_use(
            ufactory
                .get_use_by_key("marketing.communications.email".to_string())
                .unwrap(),
        );

        print!("{}", dup.as_text());
        let mut file = File::create("./tests/output/policy.txt").unwrap();
        file.write_all(dup.as_text().as_bytes()).unwrap();
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

    #[test]
    fn test_dup_serialize_ok() {
        let serialized = r#"{"name":"General Policy","description":"This is a high-level policy.","version":"1.0.1","categories":{"system.authentication":{"name":"Authentication Data","description":"Data used to manage access to the system.","fides_key":"system.authentication","organization_fides_key":"default_organization","parent_key":"system","tags":null,"is_default":true,"active":true}},"subjects":{"consultant":{"name":"Consultant","description":"An individual employed in a consultative/temporary capacity by the organization.","fides_key":"consultant","organization_fides_key":"default_organization","tags":null,"rights":null,"automated_decisions_or_profiling":false,"is_default":true,"active":true}},"uses":{"essential.service.authentication":{"name":"Essential Service Authentication","description":"Authenticate users to the product, service, application or system.","fides_key":"essential.service.authentication","organization_fides_key":"default_organization","parent_key":"essential.service","legal_basis":null,"special_category":null,"recipent":null,"legitimate_interest":false,"legitimate_interest_impact_assessment":null,"tags":null,"is_default":true,"active":true}}}"#;
        let mut dup = get_dup();

        dup.associate_category(get_data_category());
        dup.associate_subject(get_data_subject());
        dup.associate_use(get_data_use());

        assert_eq!(dup.serialize(), serialized);
    }
}
