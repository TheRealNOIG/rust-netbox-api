/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ModuleBay : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleBay {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "installed_module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub installed_module: Option<Option<Box<crate::models::ModuleBayNestedModule>>>,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Identifier to reference when renaming installed components
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl ModuleBay {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, device: crate::models::NestedDevice, name: String, created: Option<String>, last_updated: Option<String>) -> ModuleBay {
        ModuleBay {
            id,
            url,
            display,
            device: Box::new(device),
            name,
            installed_module: None,
            label: None,
            position: None,
            description: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}


