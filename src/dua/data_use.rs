//! ### Background
//! Data Uses in the taxonomy are designed to support common privacy regulations and standards out of the box, these include GDPR, CCPA, LGPD and ISO 19944.
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
    /// ```
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
            &_ => panic!("Invalid Legal Basis!"),
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
    /// ```
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
            &_ => panic!("Invalid Special Category!"),
        }
    }
}

/// Represents a Data Use
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DataUse {
    /// A UI-friendly label for the Data Subject
    pub name: String,
    /// A human-readable description of the Data Subject
    pub description: String,
    /// The fides key of the Data Subject
    pub fides_key: String,
    /// The fides key of the organization to which this Data Subject belongs.
    pub organization_fides_key: String,
    /// The fides key of the the Data Use's parent.
    pub parent_key: Option<String>,
    /// The legal basis category of which the data use falls under. This field is used as part of the creation of an exportable data map. 
    pub legal_basis: Option<LegalBasis>,
    /// The special category for processing of which the data use falls under. This field is used as part of the creation of an exportable data map. 
    pub special_category : Option<SpecialCategory>,
    /// An array of recipients is applied here when sharing personal data outside of your organization (e.g. Internal Revenue Service, HMRC, etc.)
    pub recipent : Option<Vec<String>>,
    /// A boolean value representing whether the legal basis is a Legitimate Interest. This is validated at run time and looks for a legitimate_interest_impact_assessment to exist if true.
    pub legitimate_interest : bool,
    /// A url to the legitimate interest impact assessment. Can be any valid url (e.g. http, file, etc.)
    pub legitimate_interest_impact_assessment : Option<String>,
    /// List of labels related to the Data Subject
    pub tags: Option<Vec<String>>,
    /// Indicates if the Data Subject is used as a default setting
    pub is_default: bool,
    /// Indicates if the Data Subject is available to be used
    pub active: bool,
}

impl DataUse {
    /// Constructs a new DataUse object
    ///
    /// # Arguments
    ///
    /// * nme: String - A UI-friendly label for the Data Subject.</br>
    /// * descr: String - A human-readable description of the Data Subject.</br>
    /// * fkey: String - The fides key of the Data Subject.</br>
    /// * org_key: String - The fides key of the organization to which this Data Subject belongs.</br>
    /// * prnt_key: Option<String> - The fides key of the the Data Use's parent.
    /// * lgl_basis: Option<LegalBasis> - The legal basis category of which the data use falls under. This field is used as part of the creation of an exportable data map. 
    /// * spc_cat: Option<SpecialCategory> - The special category for processing of which the data use falls under. This field is used as part of the creation of an exportable data map. 
    /// * recs: Option<Vec<String>> - An array of recipients is applied here when sharing personal data outside of your organization (e.g. Internal Revenue Service, HMRC, etc.)
    /// * leg_interest: bool -  boolean value representing whether the legal basis is a Legitimate Interest. This is validated at run time and looks for a legitimate_interest_impact_assessment to exist if true.
    /// * leg_interest_impact: Option<String> - A url to the legitimate interest impact assessment. Can be any valid url (e.g. http, file, etc.)
    /// * ind_default: bool - Indicates if the Data Subject is used as a default setting
    /// * ind_active: bool - Indicates if the Data Subject is available to be used
    ///
    /// #Example
    ///
    /// ```
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
            special_category : spc_cat,
            recipent : recs,
            legitimate_interest : leg_interest,
            legitimate_interest_impact_assessment : leg_interest_impact,
            tags: tag_list,
            is_default: ind_default,
            active: ind_active,
        }
    }

    /// Constructs a Data Use object from a serialized string
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
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;
/*
    fn get_rights() -> Vec<Right> {
        let rights: Vec<Right> = [Right::Access, Right::Informed].to_vec();
        rights
    }

    fn get_data_rights() -> DataRights {
        let data_rights = DataRights::new(Strategy::ALL, get_rights());
        data_rights
    }
*/
/*
    #[test]
    fn test_data_use_from_serialized_ok() {
        let serialized = r#"{"strategy":"ALL","values":["Access","Informed"]}"#;
        let rights = DataUse::from_serialized(serialized);
        assert_eq!(rights.strategy, Strategy::ALL);
        assert_eq!(rights.values, get_rights());
    }
*/
    #[test]
    fn test_data_use_serialize_ok() {
        let serialized = r#"{"name":"Provide the capability","description":"Provide, give, or make available the product, service, application or system.","fides_key":"provide","organization_fides_key":"default_organization","parent_key":null,"legal_basis":"LegitimateInterest","special_category":"VitalInterests","recipent":["marketing team","dog shelter"],"legitimate_interest":false,"legitimate_interest_impact_assessment":"https://example.org/legitimate_interest_assessment","tags":null,"is_default":false,"active":true}"#;
        let mut datause = DataUse::new(
            "Provide the capability".to_string(),
            "Provide, give, or make available the product, service, application or system.".to_string(),
            "provide".to_string(),
            "default_organization".to_string(),
            None,
            Some(LegalBasis::LegitimateInterest),
            Some(SpecialCategory::VitalInterests),
            Some(vec!("marketing team".to_string(), "dog shelter".to_string())),
            false,
            Some("https://example.org/legitimate_interest_assessment".to_string()),
            None,
            false,
            true
        );
        assert_eq!(datause.serialize(), serialized);
    }
/*
    #[test]
    fn test_data_subject_factory_get_subjects_ok() {
        let factory = DataSubjectFactory::new();
        assert_eq!(factory.get_subjects().len(), 16);
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
*/
}
