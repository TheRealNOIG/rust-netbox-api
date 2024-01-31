/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IpAddress : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "family")]
    pub family: Box<crate::models::AggregateFamily>,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<Box<crate::models::NestedVrf>>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::IpAddressStatus>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::IpAddressRole>>,
    #[serde(rename = "assigned_object_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<Option<String>>,
    #[serde(rename = "assigned_object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<Option<i64>>,
    #[serde(rename = "assigned_object", deserialize_with = "Option::deserialize")]
    pub assigned_object: Option<serde_json::Value>,
    #[serde(rename = "nat_inside", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nat_inside: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "nat_outside")]
    pub nat_outside: Vec<crate::models::NestedIpAddress>,
    /// Hostname or FQDN (not case-sensitive)
    #[serde(rename = "dns_name", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
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

impl IpAddress {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, family: crate::models::AggregateFamily, address: String, assigned_object: Option<serde_json::Value>, nat_outside: Vec<crate::models::NestedIpAddress>, created: Option<String>, last_updated: Option<String>) -> IpAddress {
        IpAddress {
            id,
            url,
            display,
            family: Box::new(family),
            address,
            vrf: None,
            tenant: None,
            status: None,
            role: None,
            assigned_object_type: None,
            assigned_object_id: None,
            assigned_object,
            nat_inside: None,
            nat_outside,
            dns_name: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}


