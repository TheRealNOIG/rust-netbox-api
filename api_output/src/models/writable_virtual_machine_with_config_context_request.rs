/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableVirtualMachineWithConfigContextRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableVirtualMachineWithConfigContextRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<i32>>,
    #[serde(rename = "cluster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<i32>>,
    #[serde(rename = "device", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device: Option<Option<i32>>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Option<i32>>,
    #[serde(rename = "primary_ip4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Option<i32>>,
    #[serde(rename = "primary_ip6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Option<i32>>,
    #[serde(rename = "vcpus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<Option<f64>>,
    #[serde(rename = "memory", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Option<i32>>,
    #[serde(rename = "disk", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Local config context data takes precedence over source contexts in the final rendered config context
    #[serde(rename = "local_context_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Option<serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableVirtualMachineWithConfigContextRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String) -> WritableVirtualMachineWithConfigContextRequest {
        WritableVirtualMachineWithConfigContextRequest {
            name,
            status: None,
            site: None,
            cluster: None,
            device: None,
            role: None,
            tenant: None,
            platform: None,
            primary_ip4: None,
            primary_ip6: None,
            vcpus: None,
            memory: None,
            disk: None,
            description: None,
            comments: None,
            local_context_data: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staged")]
    Staged,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Offline
    }
}

