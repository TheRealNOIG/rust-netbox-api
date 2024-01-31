/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SavedFilter : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedFilter {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "content_types")]
    pub content_types: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<i32>>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "parameters", deserialize_with = "Option::deserialize")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl SavedFilter {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, content_types: Vec<String>, name: String, slug: String, parameters: Option<serde_json::Value>, created: Option<String>, last_updated: Option<String>) -> SavedFilter {
        SavedFilter {
            id,
            url,
            display,
            content_types,
            name,
            slug,
            description: None,
            user: None,
            weight: None,
            enabled: None,
            shared: None,
            parameters,
            created,
            last_updated,
        }
    }
}


