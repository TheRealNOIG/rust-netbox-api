/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VirtualDeviceContext : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualDeviceContext {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    /// Numeric identifier unique to the parent device
    #[serde(rename = "identifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "primary_ip", deserialize_with = "Option::deserialize")]
    pub primary_ip: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "primary_ip6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "status")]
    pub status: Box<crate::models::VirtualDeviceContextStatus>,
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
    #[serde(rename = "interface_count")]
    pub interface_count: i32,
}

impl VirtualDeviceContext {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, device: crate::models::NestedDevice, primary_ip: Option<crate::models::NestedIpAddress>, status: crate::models::VirtualDeviceContextStatus, created: Option<String>, last_updated: Option<String>, interface_count: i32) -> VirtualDeviceContext {
        VirtualDeviceContext {
            id,
            url,
            display,
            name,
            device: Box::new(device),
            identifier: None,
            tenant: None,
            primary_ip: if let Some(x) = primary_ip {Some(Box::new(x))} else {None},
            primary_ip4: None,
            primary_ip6: None,
            status: Box::new(status),
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            interface_count,
        }
    }
}


