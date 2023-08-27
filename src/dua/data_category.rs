//! ### Background
//! Data Uses in the taxonomy are designed to support common privacy regulations and standards out of the box, these include GDPR, CCPA, LGPD and ISO 19944.
//! Referencing: [data_uses.csv](https://ethyca.github.io/fideslang/csv/data_uses.csv)
//!

use super::data_categories;
use derive_more::Display;

/// Represents a Data Category
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DataCategory {
    /// A UI-friendly label for the Data Category
    pub name: String,
    /// A human-readable description of the Data Category
    pub description: String,
    /// The fides key of the Data Category
    fides_key: String,
    /// The fides key of the organization to which this Data Category belongs.
    pub organization_fides_key: String,
    /// The fides key of the the Data Category's parent.
    pub parent_key: Option<String>,
    /// List of labels related to the Data Category
    pub tags: Option<Vec<String>>,
    /// Indicates if the Data Category is used as a default setting
    pub is_default: bool,
    /// Indicates if the Data Category is available to be used
    pub active: bool,
}

impl DataCategory {
    /// Constructs a new DataCatgegory object
    ///
    /// # Arguments
    ///
    /// * nme: String - A UI-friendly label for the Data Category.</br>
    /// * descr: String - A human-readable description of the Data Category.</br>
    /// * fkey: String - The fides key of the Data Category.</br>
    /// * org_key: String - The fides key of the organization to which this Data Category belongs.</br>
    /// * prnt_key: Option<String> - The fides key of the the Data Use's parent.
    /// * tag_list: Option<Vec<String>> - List of labels related to the Data Category.</br>
    /// * ind_default: bool - Indicates if the Data Category is used as a default setting
    /// * ind_active: bool - Indicates if the Data Category is available to be used
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let category = DataCategory::new(
    ///         "Authentication Data".to_string(),
    ///         "Data used to manage access to the system.".to_string(),
    ///         "system.authentication".to_string(),
    ///         "default_organization".to_string(),
    ///         Some("system".to_string()), // parent key
    ///         None, // tags
    ///         false,
    ///         true
    ///     );
    /// }
    /// ```
    pub fn new(
        nme: String,
        descr: String,
        key: String,
        org_key: String,
        prnt_key: Option<String>,
        tag_list: Option<Vec<String>>,
        ind_default: bool,
        ind_active: bool,
    ) -> Self {
        DataCategory {
            name: nme,
            description: descr,
            fides_key: key,
            organization_fides_key: org_key,
            parent_key: prnt_key,
            tags: tag_list,
            is_default: ind_default,
            active: ind_active,
        }
    }

    /// Retrieve the unique identifier of the DataCatgegory object
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let category = DataCategory::new(
    ///         "Authentication Data".to_string(),
    ///         "Data used to manage access to the system.".to_string(),
    ///         "system.authentication".to_string(),
    ///         "default_organization".to_string(),
    ///         Some("system".to_string()), // parent key
    ///         None, // tags
    ///         false,
    ///         true
    ///     );
    ///     
    ///     assert_eq!(category.get_key(), "system.authentication".to_string());
    /// }
    /// ```
    pub fn get_key(&self) -> String {
        self.fides_key.clone()
    }

    /// Constructs a Data Category object from a serialized string
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
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let serialized = r#"{"name":"Provide the capability","description":"Provide, give, or make available the product, service, application or system.","fides_key":"provide","organization_fides_key":"default_organization","parent_key":null,"legal_basis":"LegitimateInterest","special_category":"VitalInterests","recipent":["marketing team","dog shelter"],"legitimate_interest":false,"legitimate_interest_impact_assessment":"https://example.org/legitimate_interest_assessment","tags":null,"is_default":false,"active":true}"#;
    ///     let category = DataCategory::from_serialized(&serialized);
    ///     
    ///     println!("{:?}", category);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DataCategory {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Serialize a Data Category object
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
    /// use pbd::dua::data_category::DataCategory;
    ///
    /// fn main() {
    ///     let mut category = DataCategory::new(
    ///         "Authentication Data".to_string(),
    ///         "Data used to manage access to the system.".to_string(),
    ///         "system.authentication".to_string(),
    ///         "default_organization".to_string(),
    ///         Some("system".to_string()), // parent key
    ///         None, // tags
    ///         false,
    ///         true
    ///     );
    ///     
    ///     println!("{}", category.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Represents a Data Use Factory
pub struct DataCategoryFactory {
    /// The entire list of DataUses that are available
    data_categories: Vec<DataCategory>,
}
impl DataCategoryFactory {
    /// Constructs a DataCategoryFactory object
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    /// }
    /// ```
    pub fn new() -> Self {
        DataCategoryFactory {
            data_categories: Self::build_data_categories(),
        }
    }

    fn build_data_categories() -> Vec<DataCategory> {
        let mut list = Vec::new();
        let data = data_categories::read_json_data_categories();
        let data_array = data.as_array().unwrap();

        for item in data_array.iter() {
            let dc_tags = match item["tags"].is_array() {
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
            let dc_default = match item["is_default"].is_boolean() {
                true => item["is_default"].as_bool().unwrap(),
                false => false,
            };
            let dc_active = match item["active"].is_boolean() {
                true => item["active"].as_bool().unwrap(),
                false => true, // if attribute missing, then assume active
            };

            list.push(DataCategory::new(
                item["name"].as_str().unwrap().to_string(),
                item["description"].as_str().unwrap().to_string(),
                item["fides_key"].as_str().unwrap().to_string(),
                item["organization_fides_key"].as_str().unwrap().to_string(),
                parent_key,
                dc_tags,
                dc_default,
                dc_active,
            ));
        }

        list
    }

    /// Returns a list of all the active DataCategory objects
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///     assert_eq!(factory.get_data_categories().len(), 85);
    /// }
    /// ```
    pub fn get_data_categories(&self) -> Vec<DataCategory> {
        let filtered: Vec<DataCategory> = self
            .data_categories
            .iter()
            .map(|s| s.clone())
            .filter(|s| s.active == true)
            .collect();

        filtered.clone()
    }

    /// Searches the list of active DataCategories and retrieves the DataCategory object with the specified name
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the DataCategory fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///
    ///     let category = match factory.get_data_category_by_key("system.authentication".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_data_category_by_key(&self, key: String) -> Option<DataCategory> {
        let filtered: Vec<DataCategory> = self
            .data_categories
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

    /// Searches the list of active DataCategories and retrieves the DataCategory object with the specified name
    ///
    /// # Arguments
    ///
    /// * name: String - The string that represents the DataCategory name.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///
    ///     let datause = match factory.get_data_category_by_name("Authentication Data".to_string()) {
    ///         Some(s) => s,
    ///         None => panic!("Could not find it!"),
    ///     };
    /// }
    /// ```
    pub fn get_data_category_by_name(&self, name: String) -> Option<DataCategory> {
        let filtered: Vec<DataCategory> = self
            .data_categories
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

    /// Searches the list of active DataCategories and retrieves all the children of the DataCategory object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the parent DataCategory fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///
    ///     let children = factory.get_data_category_children_by_key("user.behavior".to_string());
    ///     assert_eq!(children.len(), 4);
    /// }
    /// ```
    pub fn get_data_category_children_by_key(&self, key: String) -> Vec<DataCategory> {
        let filtered: Vec<DataCategory> = self
            .data_categories
            .iter()
            .map(|s| s.clone())
            .filter(|s| match s.parent_key.clone() {
                Some(k) => k == key,
                None => false,
            })
            .collect();
        filtered
    }

    /// Searches the list of active DataCategories and retrieves the parent of the DataCategory object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the child DataCategory fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///
    ///     let parent = factory.get_data_category_parent_by_key("user.biometric".to_string());
    ///     assert_eq!(parent.unwrap().get_key(), "user".to_string());
    /// }
    /// ```
    pub fn get_data_category_parent_by_key(&self, key: String) -> Option<DataCategory> {
        let child = self.get_data_category_by_key(key);
        match child {
            Some(c) => {
                let filtered: Vec<DataCategory> = self
                    .data_categories
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

    /// Retrieves the reversed heirarchy list (Child -> Parent) of DataCategories for the DataCategory object
    ///
    /// # Arguments
    ///
    /// * key: String - The string that represents the child DataCategory fides_key.</br>
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate pbd;
    ///
    /// use pbd::dua::data_category::DataCategoryFactory;
    ///
    /// fn main() {
    ///     let factory = DataCategoryFactory::new();
    ///
    ///     let heirarchy = factory.get_reverse_heirarchy_by_key("user.contact.address.city".to_string(), None);
    ///     assert_eq!(heirarchy.len(), 4);
    /// }
    /// ```
    pub fn get_reverse_heirarchy_by_key(
        &self,
        key: String,
        heirarchy: Option<Vec<DataCategory>>,
    ) -> Vec<DataCategory> {
        let mut list = match heirarchy {
            Some(h) => h,
            None => Vec::new(),
        };

        let child = match self.get_data_category_by_key(key.clone()) {
            Some(c) => c,
            None => panic!("Invalid DataCategory fides_key {}", key),
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

    #[test]
    fn test_data_category_get_key() {
        assert_eq!(
            get_data_category().get_key(),
            "system.authentication".to_string()
        );
    }

    #[test]
    fn test_data_category_from_serialized_ok() {
        let serialized = r#"{"name":"Authentication Data","description":"Data used to manage access to the system.","fides_key":"system.authentication","organization_fides_key":"default_organization","parent_key":"system","tags":null,"is_default":false,"active":true}"#;
        let category = DataCategory::from_serialized(serialized);
        assert_eq!(category.name, "Authentication Data".to_string());
    }

    #[test]
    fn test_data_category_serialize_ok() {
        let serialized = r#"{"name":"Authentication Data","description":"Data used to manage access to the system.","fides_key":"system.authentication","organization_fides_key":"default_organization","parent_key":"system","tags":null,"is_default":false,"active":true}"#;
        assert_eq!(get_data_category().serialize(), serialized);
    }

    #[test]
    fn test_data_category_factory_get_categories_ok() {
        let factory = DataCategoryFactory::new();
        assert_eq!(factory.get_data_categories().len(), 85);
    }

    #[test]
    fn test_data_category_factory_get_data_category_by_key() {
        let factory = DataCategoryFactory::new();

        let category = match factory.get_data_category_by_key("system.authentication".to_string()) {
            Some(s) => s,
            None => panic!("Data Use not found!"),
        };

        assert_eq!(category.fides_key, "system.authentication");
    }

    #[test]
    fn test_data_category_factory_get_data_category_by_name() {
        let factory = DataCategoryFactory::new();

        let category = match factory.get_data_category_by_name("Authentication Data".to_string()) {
            Some(s) => s,
            None => panic!("Authentication Data not found!"),
        };

        assert_eq!(category.fides_key, "system.authentication");
    }

    #[test]
    fn test_data_category_factory_get_data_category_children_by_key() {
        let factory = DataCategoryFactory::new();
        let list = factory.get_data_category_children_by_key("user.behavior".to_string());
        assert_eq!(list.len(), 4);
        assert_eq!(
            list.iter()
                .map(|s| s.fides_key.clone())
                .filter(|k| k == "user.behavior.browsing_history")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.fides_key.clone())
                .filter(|k| k == "user.behavior.media_consumption")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.fides_key.clone())
                .filter(|k| k == "user.behavior.purchase_history")
                .collect::<Vec<_>>()
                .len(),
            1
        );
        assert_eq!(
            list.iter()
                .map(|s| s.fides_key.clone())
                .filter(|k| k == "user.behavior.search_history")
                .collect::<Vec<_>>()
                .len(),
            1
        );
    }

    #[test]
    fn test_data_category_factory_get_data_category_parent_by_key() {
        let factory = DataCategoryFactory::new();
        let parent =
            factory.get_data_category_parent_by_key("user.behavior.browsing_history".to_string());
        assert_eq!(parent.unwrap().fides_key, "user.behavior".to_string());
    }

    #[test]
    fn test_data_category_factory_get_reverse_heirarchy_by_key() {
        let factory = DataCategoryFactory::new();
        let heirarchy =
            factory.get_reverse_heirarchy_by_key("user.contact.address.city".to_string(), None);
        assert_eq!(heirarchy.len(), 4);
    }
}
