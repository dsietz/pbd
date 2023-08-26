//! ### Background
//! Data Subjects in the taxonomy are designed to support common privacy regulations and standards out of the box, these include GDPR, CCPA, LGPD and ISO 19944.
//! Referencing: [data_uses.csv](https://ethyca.github.io/fideslang/csv/data_subjects.csv)
//! 

use super::data_subjects;
use derive_more::Display;

/// The allowed Data Rights values for applying to a Data Subject
/// Available values coupled with Chapter 3 of the GDPR
#[derive(Debug, Deserialize, Display, Clone, PartialEq, Serialize)]
pub enum Right {
    #[display(fmt = "Informed")]
    Informed,
    #[display(fmt = "Access")]
    Access,
    #[display(fmt = "Rectification")]
    Rectification,
    #[display(fmt = "Erasure")]
    Erasure,
    #[display(fmt = "Portability")]
    Portability,
    #[display(fmt = "Restrict Processing")]
    RestrictProcessing,
    #[display(fmt = "Withdraw Consent")]
    WithdrawConsent,
    #[display(fmt = "Object")]
    Object,
    #[display(fmt = "Object To Automated Processing")]
    ObjectToAutomatedProcessing,
}

impl Right {
    /// Returns the enum Right from a string
    ///
    /// # Arguments
    ///
    /// * val: &str - The textual representation of the enum value.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::Right;
    ///
    /// fn main() {
    ///     assert_eq!(Right::from_str("Object To Automated Processing"), Right::ObjectToAutomatedProcessing);
    /// }
    /// ```
    ///
    pub fn from_str(val: &str) -> Right {
        match val {
            "Informed" => Right::Informed,
            "Access" => Right::Access,
            "Rectification" => Right::Rectification,
            "Erasure" => Right::Erasure,
            "Portability" => Right::Portability,
            "Restrict Processing" => Right::RestrictProcessing,
            "Withdraw Consent" => Right::WithdrawConsent,
            "Object" => Right::Object,
            "Object To Automated Processing" => Right::ObjectToAutomatedProcessing,
            &_ => panic!("Invalid Right!"),
        }
    }
}

/// The allowed Strategy values for applying Data Rights
#[derive(Debug, Deserialize, Display, Clone, PartialEq, Serialize)]
pub enum Strategy {
    #[display(fmt = "ALL")]
    ALL,
    #[display(fmt = "EXCLUDE")]
    EXCLUDE,
    #[display(fmt = "INCLUDE")]
    INCLUDE,
    #[display(fmt = "NONE")]
    NONE,
}

impl Strategy {
    /// Returns the enum Strategy from a string
    ///
    /// # Arguments
    ///
    /// * val: &str - The textual representation of the enum value.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::Strategy;
    ///
    /// fn main() {
    ///     assert_eq!(Strategy::from_str("INCLUDE"), Strategy::INCLUDE);
    /// }
    /// ```
    ///
    pub fn from_str(val: &str) -> Strategy {
        match val {
            "ALL" => Strategy::ALL,
            "EXCLUDE" => Strategy::EXCLUDE,
            "INCLUDE" => Strategy::INCLUDE,
            "NONE" => Strategy::NONE,
            &_ => panic!("Invalid Strategy!"),
        }
    }
}

/// Represents the Data Rights that can be applied to a Data Subject
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataRights {
    /// A strategy for selecting the rights available to the data subject (Strategy::All, Strategy::Exclude, Strategy::Include, Strategy::None)
    pub strategy: Strategy,
    /// An array of rights available to the data subject, made of available values coupled with Chapter 3 of the GDPR.
    /// The output of a data map is based upon the strategy for applying rights and the selections made from the following valid options:
    /// - Right::Informed
    /// - Right::Access
    /// - Right::Rectification
    /// - Right::Erasure
    /// - Right::Portability
    /// - Right::RestrictProcessing
    /// - Right::WithdrawConsent
    /// - Right::Object
    /// - Right::ObjectToAutomatedProcessing
    pub values: Vec<Right>,
}

impl DataRights {
    /// Constructs a DataRights object
    ///
    /// # Arguments
    ///
    /// * selection_strategy: Strategy - The enum Strategy to apply to the rights.</br>
    /// * rights: Vec<Right> - List of enum Right.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataRights, Right, Strategy};
    ///
    /// fn main() {
    ///     let rights = DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access]);
    /// }
    /// ```
    pub fn new(selection_strategy: Strategy, rights: Vec<Right>) -> Self {
        DataRights {
            strategy: selection_strategy,
            values: rights,
        }
    }

    /// Retrieves the enum Strategy that is applied to the rights
    ///
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataRights, Right, Strategy};
    ///
    /// fn main() {
    ///     let data_rights = DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access]);
    ///     assert_eq!(data_rights.get_strategy(), Strategy::ALL);
    /// }
    /// ```
    ///
    pub fn get_strategy(&self) -> Strategy {
        self.strategy.clone()
    }

    /// Retrieves the list of enum Right
    ///
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataRights, Right, Strategy};
    ///
    /// fn main() {
    ///     let data_rights = DataRights::new(Strategy::ALL, vec![Right::Informed, Right::Access]);
    ///     assert_eq!(data_rights.get_rights(), vec![Right::Informed, Right::Access]);
    /// }
    /// ```
    ///
    pub fn get_rights(&self) -> Vec<Right> {
        self.values.clone()
    }
    /// Constructs a Data Rights object from a serialized string
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
    pub fn from_serialized(serialized: &str) -> DataRights {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Serialize a Data Rights object
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

/// Represents a Data Subject
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataSubject {
    /// A UI-friendly label for the Data Subject
    pub name: String,
    /// A human-readable description of the Data Subject
    pub description: String,
    /// The fides key of the Data Subject
    pub fides_key: String,
    /// The fides key of the organization to which this Data Subject belongs.
    pub organization_fides_key: String,
    /// List of labels related to the Data Subject
    pub tags: Option<Vec<String>>,
    /// The Data Rights related to the Data Subject
    pub rights: Option<DataRights>,
    /// Indicates whether or not automated decision-making or profiling exists. Tied to article 22 of the GDPR.
    pub automated_decisions_or_profiling: bool,
    /// Indicates if the Data Subject is used as a default setting
    pub is_default: bool,
    /// Indicates if the Data Subject is available to be used
    pub active: bool,
}

impl DataSubject {
    /// Constructs a new DataSubject object
    ///
    /// # Arguments
    ///
    /// * nme: String - A UI-friendly label for the Data Subject.</br>
    /// * descr: String - A human-readable description of the Data Subject.</br>
    /// * fkey: String - The fides key of the Data Subject.</br>
    /// * org_key: String - The fides key of the organization to which this Data Subject belongs.</br>
    /// * tag_list: Option<Vec<String>> - List of labels related to the Data Subject.</br>
    /// * rights_list: Option<DataRights> - The Data Rights related to the Data Subject.</br>
    /// * auto_decide: bool - Indicates whether or not automated decision-making or profiling exists. Tied to article 22 of the GDPR.</br>
    /// * ind_default: bool - Indicates if the Data Subject is used as a default setting
    /// * ind_active: bool - Indicates if the Data Subject is available to be used
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataSubject, DataRights, Right, Strategy};
    ///
    /// fn main() {
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
    /// }
    /// ```
    ///
    pub fn new(
        nme: String,
        descr: String,
        key: String,
        org_key: String,
        tag_list: Option<Vec<String>>,
        rights_list: Option<DataRights>,
        auto_decide: bool,
        ind_default: bool,
        ind_active: bool,
    ) -> Self {
        DataSubject {
            name: nme,
            description: descr,
            fides_key: key,
            organization_fides_key: org_key,
            tags: tag_list,
            rights: rights_list,
            automated_decisions_or_profiling: auto_decide,
            is_default: ind_default,
            active: ind_active,
        }
    }

    /// Retrieves the list of enum Right that are related to the DataSubject object
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataSubject, DataRights, Right, Strategy};
    ///
    /// fn main() {
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
    ///     assert_eq!(subject.get_data_rights().unwrap(), vec![Right::Informed, Right::Access]);
    /// }
    /// ```
    pub fn get_data_rights(&self) -> Option<Vec<Right>> {
        match self.rights.as_ref() {
            Some(r) => Some(r.clone().get_rights()),
            None => None,
        }
    }

    /// Retrieves the enum Strategy that is applied to the Rights related to the DataSubject object
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::{DataSubject, DataRights, Right, Strategy};
    ///
    /// fn main() {
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
    ///     assert_eq!(subject.get_data_strategy().unwrap(), Strategy::ALL);
    /// }
    /// ```
    pub fn get_data_strategy(&self) -> Option<Strategy> {
        match self.rights.as_ref() {
            Some(r) => Some(r.clone().get_strategy()),
            None => None,
        }
    }
}

/// Represents a Data Subject Factory
pub struct DataSubjectFactory {
    /// The entire list of DataSubjects that are available
    subjects: Vec<DataSubject>,
}
impl DataSubjectFactory {
    /// Constructs a DataSubjectFactory object
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::DataSubjectFactory;
    ///
    /// fn main() {
    ///     let factory = DataSubjectFactory::new();
    /// }
    /// ```
    pub fn new() -> Self {
        DataSubjectFactory {
            subjects: Self::build_subjects(),
        }
    }

    fn build_subjects() -> Vec<DataSubject> {
        let mut list = Vec::new();
        let data = data_subjects::read_json_data_subjects();
        let data_array = data.as_array().unwrap();

        for item in data_array.iter() {
            let subject_tags = match item["tags"].is_array() {
                true => {
                    let mut tag_list = Vec::new();
                    let tags = item["tags"].as_array().unwrap();
                    for tag in tags {
                        tag_list.push(tag.as_str().unwrap().to_string());
                    }
                    Some(tag_list)
                }
                false => None,
            };
            let subject_rights = match item["rights"].is_object() {
                true => {
                    let mut rights_list = Vec::new();
                    let rights = item["rights"]["values"].as_array().unwrap();
                    for right in rights {
                        rights_list.push(Right::from_str(right.as_str().unwrap()));
                    }
                    Some(DataRights::new(
                        Strategy::from_str(item["rights"]["strategy"].as_str().unwrap()),
                        rights_list,
                    ))
                }
                false => None,
            };
            let subject_auto = match item["automated_decisions_or_profiling"].is_boolean() {
                true => item["automated_decisions_or_profiling"].as_bool().unwrap(),
                false => false,
            };
            let subject_default = match item["is_default"].is_boolean() {
                true => item["is_default"].as_bool().unwrap(),
                false => false,
            };
            let subject_active = match item["active"].is_boolean() {
                true => item["active"].as_bool().unwrap(),
                false => false,
            };

            list.push(DataSubject::new(
                item["name"].as_str().unwrap().to_string(),
                item["description"].as_str().unwrap().to_string(),
                item["fides_key"].as_str().unwrap().to_string(),
                item["organization_fides_key"].as_str().unwrap().to_string(),
                subject_tags,
                subject_rights,
                subject_auto,
                subject_default,
                subject_active,
            ));
        }

        list
    }

    /// Returns a list of all the active DataSubjects
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::DataSubjectFactory;
    ///
    /// fn main() {
    ///     let factory = DataSubjectFactory::new();
    ///     assert_eq!(factory.get_subjects().len(), 15);
    /// }
    /// ```
    pub fn get_subjects(&self) -> Vec<DataSubject> {
        let filtered: Vec<DataSubject> = self
            .subjects
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.active == true)
            .collect();

        filtered.clone()
    }

    /// Searches the list of active DataSubjects and retrieves the DataSubject object with the specified name
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the DataSubject fides_key.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::DataSubjectFactory;
    ///
    /// fn main() {
    ///     let factory = DataSubjectFactory::new();
    ///     
    ///     let subject = match factory.get_subject_by_key("customer".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_subject_by_key(&self, key: String) -> Option<DataSubject> {
        let filtered: Vec<DataSubject> = self
            .subjects
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.fides_key == key)
            .collect();
        match filtered.len() {
            0 => None,
            1 => Some(filtered[0].clone()),
            _ => panic!("Duplicate DataSubject objects found!"),
        }
    }

    /// Searches the list of active DataSubjects and retrieves the DataSubject object with the specified name
    /// 
    /// # Arguments
    ///
    /// * name: String - The string that represents the DataSubject name.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_subject::DataSubjectFactory;
    ///
    /// fn main() {
    ///     let factory = DataSubjectFactory::new();
    ///     
    ///     let subject = match factory.get_subject_by_name("Customer".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_subject_by_name(&self, name: String) -> Option<DataSubject> {
        let filtered: Vec<DataSubject> = self
            .subjects
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.name == name)
            .collect();
        match filtered.len() {
            0 => None,
            1 => Some(filtered[0].clone()),
            _ => panic!("Duplicate DataSubject objects found!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rights() -> Vec<Right> {
        let rights: Vec<Right> = [Right::Access, Right::Informed].to_vec();
        rights
    }

    fn get_data_rights() -> DataRights {
        let data_rights = DataRights::new(Strategy::ALL, get_rights());
        data_rights
    }

    #[test]
    fn test_data_rights_from_serialized_ok() {
        let serialized = r#"{"strategy":"ALL","values":["Access","Informed"]}"#;
        let rights = DataRights::from_serialized(serialized);
        assert_eq!(rights.strategy, Strategy::ALL);
        assert_eq!(rights.values, get_rights());
    }

    #[test]
    fn test_data_rights_serialize_ok() {
        let serialized = r#"{"strategy":"ALL","values":["Access","Informed"]}"#;
        let mut rights = get_data_rights();
        assert_eq!(rights.serialize(), serialized);
    }

    #[test]
    fn test_data_subject_factory_get_subjects_ok() {
        let factory = DataSubjectFactory::new();
        assert_eq!(factory.get_subjects().len(), 15);
    }

    #[test]
    fn test_data_subject_factory_get_subject_by_key() {
        let factory = DataSubjectFactory::new();

        let subject = match factory.get_subject_by_key("customer".to_string()) {
            Some(s) => s,
            None => panic!("Customer not found!"),
        };

        assert_eq!(subject.fides_key, "customer");
        assert_eq!(subject.get_data_strategy(), None);
        assert_eq!(subject.get_data_rights(), None);
    }

    #[test]
    fn test_data_subject_factory_get_subject_by_name_with_rights() {
        let factory = DataSubjectFactory::new();

        let subject = match factory.get_subject_by_name("Citizen Voter".to_string()) {
            Some(s) => s,
            None => panic!("Citizen Voter not found!"),
        };

        assert_eq!(subject.fides_key, "citizen_voter");
        assert_eq!(subject.get_data_strategy().unwrap(), Strategy::INCLUDE);
        assert_eq!(subject.get_data_rights().unwrap().len(), 5);
    }

    #[test]
    fn test_data_subject_factory_get_subject_by_name_without_rights() {
        let factory = DataSubjectFactory::new();

        let subject = match factory.get_subject_by_name("Commuter".to_string()) {
            Some(s) => s,
            None => panic!("Citizen Voter not found!"),
        };

        assert_eq!(subject.fides_key, "commuter");
        assert_eq!(subject.get_data_strategy(), None);
        assert_eq!(subject.get_data_rights(), None);
    }
}
