/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConfigContext : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigContext {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    #[serde(rename = "site_groups", skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    #[serde(rename = "sites", skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    #[serde(rename = "device_types", skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    #[serde(rename = "cluster_types", skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    #[serde(rename = "cluster_groups", skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    #[serde(rename = "clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    #[serde(rename = "tenant_groups", skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    #[serde(rename = "tenants", skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Box<crate::models::NestedDataSource>>,
    /// Path to remote file (relative to data source root)
    #[serde(rename = "data_path")]
    pub data_path: String,
    #[serde(rename = "data_file")]
    pub data_file: Box<crate::models::NestedDataFile>,
    #[serde(rename = "data_synced", deserialize_with = "Option::deserialize")]
    pub data_synced: Option<String>,
    #[serde(rename = "data", deserialize_with = "Option::deserialize")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl ConfigContext {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, name: String, data_path: String, data_file: crate::models::NestedDataFile, data_synced: Option<String>, data: Option<serde_json::Value>, created: Option<String>, last_updated: Option<String>) -> ConfigContext {
        ConfigContext {
            id,
            url,
            display,
            name,
            weight: None,
            description: None,
            is_active: None,
            regions: None,
            site_groups: None,
            sites: None,
            locations: None,
            device_types: None,
            roles: None,
            platforms: None,
            cluster_types: None,
            cluster_groups: None,
            clusters: None,
            tenant_groups: None,
            tenants: None,
            tags: None,
            data_source: None,
            data_path,
            data_file: Box::new(data_file),
            data_synced,
            data,
            created,
            last_updated,
        }
    }
}


