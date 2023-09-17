//! ### Background
//! Data Uses in the taxonomy are designed to support common privacy regulations and standards out of the box, these include GDPR, CCPA, LGPD and ISO 19944.
//! Referencing: [data_uses.csv](https://ethyca.github.io/fideslang/csv/data_uses.csv)
//!

use super::data_uses;
use derive_more::Display;

/// The allowed Legal Basis values for applying to a Data Use
/// Current valid options:
#[derive(Debug, Deserialize, Display, Clone, PartialEq, Serialize)]
pub enum LegalBasis {
    #[display(fmt = "Consent")]
    Consent,
    #[display(fmt = "Contract")]
    Contract,
    #[display(fmt = "Legal Obligation")]
    LegalObligation,
    #[display(fmt = "Vital Interest")]
    VitalInterest,
    #[display(fmt = "Public Interest")]
    PublicInterest,
    #[display(fmt = "Legitimate Interest")]
    LegitimateInterest,
}

impl LegalBasis {
    /// Returns the enum LegalBasis from a string
    ///
    /// # Arguments
    ///
    /// * val: &str - The textual representation of the enum value.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::LegalBasis;
    ///
    /// fn main() {
    ///     assert_eq!(LegalBasis::from_str("Legitimate Interest"), LegalBasis::LegitimateInterest);
    /// }
    /// ```
    ///
    pub fn from_str(val: &str) -> LegalBasis {
        match val {
            "Consent" => LegalBasis::Consent,
            "Contract" => LegalBasis::Contract,
            "Legal Obligation" => LegalBasis::LegalObligation,
            "Vital Interest" => LegalBasis::VitalInterest,
            "Public Interest" => LegalBasis::PublicInterest,
            "Legitimate Interest" => LegalBasis::LegitimateInterest,
            &_ => panic!("Invalid Legal Basis: {}", val),
        }
    }
}

/// The allowed Special Category values for applying to a Data Use
/// Current valid options:
#[derive(Debug, Deserialize, Display, Clone, PartialEq, Serialize)]
pub enum SpecialCategory {
    #[display(fmt = "Consent")]
    Consent,
    #[display(fmt = "Employment")]
    Employment,
    #[display(fmt = "Vital Interests")]
    VitalInterests,
    #[display(fmt = "Non-profit Bodies")]
    NonprofitBodies,
    #[display(fmt = "Public by Data Subject")]
    PublicByDataSubject,
    #[display(fmt = "Legal Claims")]
    LegalClaims,
    #[display(fmt = "Substantial Public Interest")]
    SubstantialPublicInterest,
    #[display(fmt = "Medical")]
    Medical,
    #[display(fmt = "Public Health Interest")]
    PublicHealthInterest,
}

impl SpecialCategory {
    /// Returns the enum SpecialCategory from a string
    ///
    /// # Arguments
    ///
    /// * val: &str - The textual representation of the enum value.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::SpecialCategory;
    ///
    /// fn main() {
    ///     assert_eq!(SpecialCategory::from_str("Public Health Interest"), SpecialCategory::PublicHealthInterest);
    /// }
    /// ```
    ///
    pub fn from_str(val: &str) -> SpecialCategory {
        match val {
            "Consent" => SpecialCategory::Consent,
            "Employment" => SpecialCategory::Employment,
            "Vital Interests" => SpecialCategory::VitalInterests,
            "Non-profit Bodies" => SpecialCategory::NonprofitBodies,
            "Public by Data Subject" => SpecialCategory::PublicByDataSubject,
            "Legal Claims" => SpecialCategory::LegalClaims,
            "Substantial Public Interest" => SpecialCategory::SubstantialPublicInterest,
            "Medical" => SpecialCategory::Medical,
            "Public Health Interest" => SpecialCategory::PublicHealthInterest,
            &_ => panic!("Invalid Special Category: {}", val),
        }
    }
}

/// Represents a Data Use
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DataUse {
    /// A UI-friendly label for the Data Use
    pub name: String,
    /// A human-readable description of the Data Use
    pub description: String,
    /// The fides key of the Data Use
    fides_key: String,
    /// The fides key of the organization to which this Data Use belongs.
    pub organization_fides_key: String,
    /// The fides key of the the Data Use's parent.
    pub parent_key: Option<String>,
    /// The legal basis category of which the data use falls under. This field is used as part of the creation of an exportable data map.
    pub legal_basis: Option<LegalBasis>,
    /// The special category for processing of which the data use falls under. This field is used as part of the creation of an exportable data map.
    pub special_category: Option<SpecialCategory>,
    /// An array of recipients is applied here when sharing personal data outside of your organization (e.g. Internal Revenue Service, HMRC, etc.)
    pub recipent: Option<Vec<String>>,
    /// A boolean value representing whether the legal basis is a Legitimate Interest. This is validated at run time and looks for a legitimate_interest_impact_assessment to exist if true.
    pub legitimate_interest: bool,
    /// A url to the legitimate interest impact assessment. Can be any valid url (e.g. http, file, etc.)
    pub legitimate_interest_impact_assessment: Option<String>,
    /// List of labels related to the Data Use
    pub tags: Option<Vec<String>>,
    /// Indicates if the Data Use is used as a default setting
    pub is_default: bool,
    /// Indicates if the Data Use is available to be used
    pub active: bool,
}

impl DataUse {
    /// Constructs a new DataUse object
    ///
    /// # Arguments
    ///
    /// * nme: String - A UI-friendly label for the Data Use.</br>
    /// * descr: String - A human-readable description of the Data Use.</br>
    /// * fkey: String - The fides key of the Data Use.</br>
    /// * org_key: String - The fides key of the organization to which this Data Use belongs.</br>
    /// * prnt_key: Option<String> - The fides key of the the Data Use's parent.
    /// * lgl_basis: Option<LegalBasis> - The legal basis category of which the data use falls under. This field is used as part of the creation of an exportable data map.
    /// * spc_cat: Option<SpecialCategory> - The special category for processing of which the data use falls under. This field is used as part of the creation of an exportable data map.
    /// * recs: Option<Vec<String>> - An array of recipients is applied here when sharing personal data outside of your organization (e.g. Internal Revenue Service, HMRC, etc.)
    /// * leg_interest: bool -  boolean value representing whether the legal basis is a Legitimate Interest. This is validated at run time and looks for a legitimate_interest_impact_assessment to exist if true.
    /// * leg_interest_impact: Option<String> - A url to the legitimate interest impact assessment. Can be any valid url (e.g. http, file, etc.)
    /// * tag_list: Option<Vec<String>> - List of labels related to the Data Use.</br>
    /// * ind_default: bool - Indicates if the Data Use is used as a default setting
    /// * ind_active: bool - Indicates if the Data Use is available to be used
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
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
    /// }
    /// ```
    ///
    pub fn new(
        nme: String,
        descr: String,
        key: String,
        org_key: String,
        prnt_key: Option<String>,
        lgl_basis: Option<LegalBasis>,
        spc_cat: Option<SpecialCategory>,
        recs: Option<Vec<String>>,
        leg_interest: bool,
        leg_interest_impact: Option<String>,
        tag_list: Option<Vec<String>>,
        ind_default: bool,
        ind_active: bool,
    ) -> Self {
        DataUse {
            name: nme,
            description: descr,
            fides_key: key,
            organization_fides_key: org_key,
            parent_key: prnt_key,
            legal_basis: lgl_basis,
            special_category: spc_cat,
            recipent: recs,
            legitimate_interest: leg_interest,
            legitimate_interest_impact_assessment: leg_interest_impact,
            tags: tag_list,
            is_default: ind_default,
            active: ind_active,
        }
    }

    /// Retrieve the unique identifier of the DataUse object
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
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
    ///     assert_eq!(datause.get_key(), "provide".to_string());
    /// }
    /// ```
    pub fn get_key(&self) -> String {
        self.fides_key.clone()
    }

    /// Constructs a Data Use object from a serialized string
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
    /// use pbd::dua::data_use::DataUse;
    ///
    /// fn main() {
    ///     let serialized = r#"{"name":"Provide the capability","description":"Provide, give, or make available the product, service, application or system.","fides_key":"provide","organization_fides_key":"default_organization","parent_key":null,"legal_basis":"LegitimateInterest","special_category":"VitalInterests","recipent":["marketing team","dog shelter"],"legitimate_interest":false,"legitimate_interest_impact_assessment":"https://example.org/legitimate_interest_assessment","tags":null,"is_default":false,"active":true}"#;
    ///     let datause = DataUse::from_serialized(&serialized);
    ///     
    ///     println!("{:?}", datause);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DataUse {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Serialize a Data Use object
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
    /// use pbd::dua::data_use::{DataUse, LegalBasis, SpecialCategory};
    ///
    /// fn main() {
    ///     let mut datause = DataUse::new(
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
    ///     println!("{:?}", datause.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Represents a Data Use Factory
pub struct DataUseFactory {
    /// The entire list of DataUses that are available
    data_uses: Vec<DataUse>,
}
impl DataUseFactory {
    /// Constructs a DataUseFactory object
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    /// }
    /// ```
    pub fn new() -> Self {
        DataUseFactory {
            data_uses: Self::build_data_uses(),
        }
    }

    fn build_data_uses() -> Vec<DataUse> {
        let mut list = Vec::new();
        let data = data_uses::read_json_data_uses();
        let data_array = data.as_array().unwrap();

        for item in data_array.iter() {
            let du_tags = match item["tags"].is_array() {
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
            let parent_key = match item["parent_key"].is_string() {
                true => Some(item["parent_key"].as_str().unwrap().to_string()),
                false => None,
            };
            let legal_basis = match item["legal_basis"].is_string() {
                true => Some(LegalBasis::from_str(item["legal_basis"].as_str().unwrap())),
                false => None,
            };
            let special_category = match item["special_category"].is_string() {
                true => Some(SpecialCategory::from_str(
                    item["special_category"].as_str().unwrap(),
                )),
                false => None,
            };
            let recipients = match item["recipients"].is_object() {
                true => {
                    let mut rcp_list = Vec::new();
                    let rcps = item["recipients"]["values"].as_array().unwrap();
                    for recp in rcps {
                        rcp_list.push(recp.as_str().unwrap().to_string());
                    }
                    Some(rcp_list)
                }
                false => None,
            };
            let legit_interest = match item["legitimate_interest"].is_boolean() {
                true => item["legitimate_interest"].as_bool().unwrap(),
                false => false,
            };
            let legit_interest_impact =
                match item["legitimate_interest_impact_assessment"].is_string() {
                    true => Some(
                        item["legitimate_interest_impact_assessment"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    ),
                    false => None,
                };
            let du_default = match item["is_default"].is_boolean() {
                true => item["is_default"].as_bool().unwrap(),
                false => false,
            };
            let du_active = match item["active"].is_boolean() {
                true => item["active"].as_bool().unwrap(),
                false => true, // if attribute missing, then assume active
            };

            list.push(DataUse::new(
                item["name"].as_str().unwrap().to_string(),
                item["description"].as_str().unwrap().to_string(),
                item["fides_key"].as_str().unwrap().to_string(),
                item["organization_fides_key"].as_str().unwrap().to_string(),
                parent_key,
                legal_basis,
                special_category,
                recipients,
                legit_interest,
                legit_interest_impact,
                du_tags,
                du_default,
                du_active,
            ));
        }

        list
    }

    /// Returns a list of all the active DataUses
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     assert_eq!(factory.get_uses().len(), 52);
    /// }
    /// ```
    pub fn get_uses(&self) -> Vec<DataUse> {
        let filtered: Vec<DataUse> = self
            .data_uses
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.active == true)
            .collect();

        filtered.clone()
    }

    /// Searches the list of active DataUses and retrieves the DataUse object with the specified name
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the DataUse fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     
    ///     let subject = match factory.get_use_by_key("essential.service.operations.support".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_use_by_key(&self, key: String) -> Option<DataUse> {
        let filtered: Vec<DataUse> = self
            .data_uses
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.fides_key == key)
            .collect();
        match filtered.len() {
            0 => None,
            1 => Some(filtered[0].clone()),
            _ => panic!("Duplicate DataUse objects found!"),
        }
    }

    /// Searches the list of active DataUses and retrieves the DataUse object with the specified name
    ///
    /// # Arguments
    ///
    /// * name: String - The string that represents the DataUse name.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     
    ///     let datause = match factory.get_use_by_name("Essential for Operations Support".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_use_by_name(&self, name: String) -> Option<DataUse> {
        let filtered: Vec<DataUse> = self
            .data_uses
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.name == name)
            .collect();
        match filtered.len() {
            0 => None,
            1 => Some(filtered[0].clone()),
            _ => panic!("Duplicate DataUse objects found!"),
        }
    }

    /// Searches the list of active DataUses and retrieves all the children of the DataUse object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the parent DataUse fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     
    ///     let children = factory.get_use_children_by_key("marketing.advertising".to_string());
    ///     assert_eq!(children.len(), 6);
    /// }
    /// ```
    pub fn get_use_children_by_key(&self, key: String) -> Vec<DataUse> {
        let filtered: Vec<DataUse> = self
            .data_uses
            .iter()
            .map(|s| s.clone())
            .filter(|s| match s.parent_key.clone() {
                Some(k) => k == key,
                None => false,
            })
            .collect();
        filtered
    }

    /// Searches the list of active DataUses and retrieves the parent of the DataUse object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the child DataUse fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     
    ///     let parent = factory.get_use_parent_by_key("marketing.advertising".to_string());
    ///     assert_eq!(parent.unwrap().get_key(), "marketing".to_string());
    /// }
    /// ```
    pub fn get_use_parent_by_key(&self, key: String) -> Option<DataUse> {
        let child = self.get_use_by_key(key);
        match child {
            Some(c) => {
                let filtered: Vec<DataUse> = self
                    .data_uses
                    .iter()
                    .map(|s| s.clone())
                    .filter(|s| match c.parent_key.clone() {
                        Some(pk) => s.fides_key == pk,
                        None => false,
                    })
                    .collect();

                match filtered.len() {
                    1 => Some(filtered[0].clone()),
                    _ => None,
                }
            }
            None => None,
        }
    }

    /// Retrieves the reversed heirarchy list (Child -> Parent) of DataUses for the DataUse object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the child DataUse fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_use::DataUseFactory;
    ///
    /// fn main() {
    ///     let factory = DataUseFactory::new();
    ///     
    ///     let heirarchy = factory.get_reverse_heirarchy_by_key("essential.service.notifications.email".to_string(), None);
    ///     assert_eq!(heirarchy.len(), 4);
    /// }
    /// ```
    pub fn get_reverse_heirarchy_by_key(
        &self,
        key: String,
        heirarchy: Option<Vec<DataUse>>,
    ) -> Vec<DataUse> {
        let mut list = match heirarchy {
            Some(h) => h,
            None => Vec::new(),
        };

        let child = match self.get_use_by_key(key.clone()) {
            Some(c) => c,
            None => panic!("Invalid DataUse fides_key {}", key),
        };

        list.push(child.clone());

        match child.parent_key {
            Some(p) => self.get_reverse_heirarchy_by_key(p, Some(list)),
            None => list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_data_use_from_serialized_ok() {
        let serialized = r#"{"name":"Provide the capability","description":"Provide, give, or make available the product, service, application or system.","fides_key":"provide","organization_fides_key":"default_organization","parent_key":null,"legal_basis":"LegitimateInterest","special_category":"VitalInterests","recipent":["marketing team","dog shelter"],"legitimate_interest":false,"legitimate_interest_impact_assessment":"https://example.org/legitimate_interest_assessment","tags":null,"is_default":false,"active":true}"#;
        let datause = DataUse::from_serialized(serialized);
        assert_eq!(
            datause.special_category.unwrap(),
            SpecialCategory::VitalInterests
        );
        assert_eq!(datause.legal_basis.unwrap(), LegalBasis::LegitimateInterest);
    }

    #[test]
    fn test_data_use_serialize_ok() {
        let serialized = r#"{"name":"Provide the capability","description":"Provide, give, or make available the product, service, application or system.","fides_key":"provide","organization_fides_key":"default_organization","parent_key":null,"legal_basis":"LegitimateInterest","special_category":"VitalInterests","recipent":["marketing team","dog shelter"],"legitimate_interest":false,"legitimate_interest_impact_assessment":"https://example.org/legitimate_interest_assessment","tags":null,"is_default":false,"active":true}"#;
        assert_eq!(get_data_use().serialize(), serialized);
    }

    #[test]
    fn test_data_use_factory_get_uses_ok() {
        let factory = DataUseFactory::new();
        assert_eq!(factory.get_uses().len(), 52);
    }

    #[test]
    fn test_data_use_factory_get_use_by_key() {
        let factory = DataUseFactory::new();

        let datause =
            match factory.get_use_by_key("essential.service.operations.support".to_string()) {
                Some(s) => s,
                None => panic!("Data Use not found!"),
            };

        assert_eq!(datause.get_key(), "essential.service.operations.support");
    }

    #[test]
    fn test_data_use_factory_get_use_by_name() {
        let factory = DataUseFactory::new();

        let datause = match factory.get_use_by_name("Essential for Operations Support".to_string())
        {
            Some(s) => s,
            None => panic!("Essential for Operations Support not found!"),
        };

        assert_eq!(datause.get_key(), "essential.service.operations.support");
    }

    #[test]
    fn test_data_use_factory_get_use_children_by_key() {
        let factory = DataUseFactory::new();
        let list = factory.get_use_children_by_key("marketing.advertising".to_string());
        assert_eq!(list.len(), 6);
        assert_eq!(
            list.iter()
                .map(|s| s.get_key().clone())
                .filter(|k| k == "marketing.advertising.frequency_capping")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.get_key().clone())
                .filter(|k| k == "marketing.advertising.negative_targeting")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.get_key().clone())
                .filter(|k| k == "marketing.advertising.profiling")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.get_key().clone())
                .filter(|k| k == "marketing.advertising.serving")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.get_key().clone())
                .filter(|k| k == "marketing.advertising.third_party")
                .collect::<Vec<_>>()
                .len(),
            1
        );
    }

    #[test]
    fn test_data_use_factory_get_use_parent_by_key() {
        let factory = DataUseFactory::new();
        let parent = factory.get_use_parent_by_key("marketing.advertising".to_string());
        assert_eq!(parent.unwrap().get_key(), "marketing".to_string());
    }

    #[test]
    fn test_data_use_factory_get_reverse_heirarchy_by_key() {
        let factory = DataUseFactory::new();
        let heirarchy = factory.get_reverse_heirarchy_by_key(
            "essential.service.notifications.email".to_string(),
            None,
        );
        assert_eq!(heirarchy.len(), 4);
    }
}
