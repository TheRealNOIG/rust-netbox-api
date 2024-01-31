/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Vrf : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vrf {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Unique route distinguisher (as defined in RFC 4364)
    #[serde(rename = "rd", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rd: Option<Option<String>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    /// Prevent duplicate prefixes/IP addresses within this VRF
    #[serde(rename = "enforce_unique", skip_serializing_if = "Option::is_none")]
    pub enforce_unique: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "import_targets", skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<i32>>,
    #[serde(rename = "export_targets", skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "ipaddress_count")]
    pub ipaddress_count: i32,
    #[serde(rename = "prefix_count")]
    pub prefix_count: i32,
}

impl Vrf {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, created: Option<String>, last_updated: Option<String>, ipaddress_count: i32, prefix_count: i32) -> Vrf {
        Vrf {
            id,
            url,
            display,
            name,
            rd: None,
            tenant: None,
            enforce_unique: None,
            description: None,
            comments: None,
            import_targets: None,
            export_targets: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            ipaddress_count,
            prefix_count,
        }
    }
}


