/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Service : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device: Option<Option<Box<crate::models::NestedDevice>>>,
    #[serde(rename = "virtual_machine", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<Option<Box<crate::models::NestedVirtualMachine>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ports")]
    pub ports: Vec<i32>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Box<crate::models::ServiceProtocol>>,
    #[serde(rename = "ipaddresses", skip_serializing_if = "Option::is_none")]
    pub ipaddresses: Option<Vec<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl Service {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, ports: Vec<i32>, created: Option<String>, last_updated: Option<String>) -> Service {
        Service {
            id,
            url,
            display,
            device: None,
            virtual_machine: None,
            name,
            ports,
            protocol: None,
            ipaddresses: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}


