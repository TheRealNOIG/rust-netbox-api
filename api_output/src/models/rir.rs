/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Rir : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rir {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    /// IP space managed by this RIR is considered private
    #[serde(rename = "is_private", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
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
    #[serde(rename = "aggregate_count")]
    pub aggregate_count: i32,
}

impl Rir {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, slug: String, created: Option<String>, last_updated: Option<String>, aggregate_count: i32) -> Rir {
        Rir {
            id,
            url,
            display,
            name,
            slug,
            is_private: None,
            description: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            aggregate_count,
        }
    }
}


