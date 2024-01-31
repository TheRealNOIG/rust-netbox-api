/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequest {
    #[serde(rename = "device", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device: Option<Option<Box<crate::models::NestedDeviceRequest>>>,
    #[serde(rename = "virtual_machine", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<Option<Box<crate::models::NestedVirtualMachineRequest>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ports")]
    pub ports: Vec<i32>,
    /// * `tcp` - TCP * `udp` - UDP * `sctp` - SCTP
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "ipaddresses", skip_serializing_if = "Option::is_none")]
    pub ipaddresses: Option<Vec<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ServiceRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String, ports: Vec<i32>) -> ServiceRequest {
        ServiceRequest {
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
        }
    }
}

/// * `tcp` - TCP * `udp` - UDP * `sctp` - SCTP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}

