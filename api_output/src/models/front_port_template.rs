/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FrontPortTemplate : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontPortTemplate {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Option<Box<crate::models::NestedDeviceType>>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleType>>>,
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::FrontPortType>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "rear_port")]
    pub rear_port: Box<crate::models::NestedRearPortTemplate>,
    #[serde(rename = "rear_port_position", skip_serializing_if = "Option::is_none")]
    pub rear_port_position: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl FrontPortTemplate {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, name: String, r#type: crate::models::FrontPortType, rear_port: crate::models::NestedRearPortTemplate, created: Option<String>, last_updated: Option<String>) -> FrontPortTemplate {
        FrontPortTemplate {
            id,
            url,
            display,
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: Box::new(r#type),
            color: None,
            rear_port: Box::new(rear_port),
            rear_port_position: None,
            description: None,
            created,
            last_updated,
        }
    }
}


