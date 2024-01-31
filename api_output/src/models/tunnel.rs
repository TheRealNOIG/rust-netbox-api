/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Tunnel : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tunnel {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: Box<crate::models::TunnelStatus>,
    #[serde(rename = "group")]
    pub group: Box<crate::models::NestedTunnelGroup>,
    #[serde(rename = "encapsulation")]
    pub encapsulation: Box<crate::models::TunnelEncapsulation>,
    #[serde(rename = "ipsec_profile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipsec_profile: Option<Option<Box<crate::models::NestedIpSecProfile>>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "tunnel_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<Option<i64>>,
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

impl Tunnel {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, status: crate::models::TunnelStatus, group: crate::models::NestedTunnelGroup, encapsulation: crate::models::TunnelEncapsulation, created: Option<String>, last_updated: Option<String>) -> Tunnel {
        Tunnel {
            id,
            url,
            display,
            name,
            status: Box::new(status),
            group: Box::new(group),
            encapsulation: Box::new(encapsulation),
            ipsec_profile: None,
            tenant: None,
            tunnel_id: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}

