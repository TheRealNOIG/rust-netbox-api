/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConfigTemplate : Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment on create() and update().



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigTemplate {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Any <a href=\"https://jinja.palletsprojects.com/en/3.1.x/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja2 environment.
    #[serde(rename = "environment_params", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub environment_params: Option<Option<serde_json::Value>>,
    /// Jinja2 template code.
    #[serde(rename = "template_code")]
    pub template_code: String,
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Box<crate::models::NestedDataSource>>,
    /// Path to remote file (relative to data source root)
    #[serde(rename = "data_path")]
    pub data_path: String,
    #[serde(rename = "data_file", skip_serializing_if = "Option::is_none")]
    pub data_file: Option<Box<crate::models::NestedDataFile>>,
    #[serde(rename = "data_synced", deserialize_with = "Option::deserialize")]
    pub data_synced: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl ConfigTemplate {
    /// Introduces support for Tag assignment. Adds `tags` serialization, and handles tag assignment on create() and update().
    pub fn new(id: i32, url: String, display: String, name: String, template_code: String, data_path: String, data_synced: Option<String>, created: Option<String>, last_updated: Option<String>) -> ConfigTemplate {
        ConfigTemplate {
            id,
            url,
            display,
            name,
            description: None,
            environment_params: None,
            template_code,
            data_source: None,
            data_path,
            data_file: None,
            data_synced,
            tags: None,
            created,
            last_updated,
        }
    }
}


