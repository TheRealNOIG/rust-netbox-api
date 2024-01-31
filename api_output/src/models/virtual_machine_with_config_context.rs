/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VirtualMachineWithConfigContext : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineWithConfigContext {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::ModuleStatus>>,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<Box<crate::models::NestedSite>>>,
    #[serde(rename = "cluster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<Box<crate::models::NestedCluster>>>,
    #[serde(rename = "device", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device: Option<Option<Box<crate::models::NestedDevice>>>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<Box<crate::models::NestedDeviceRole>>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Option<Box<crate::models::NestedPlatform>>>,
    #[serde(rename = "primary_ip", deserialize_with = "Option::deserialize")]
    pub primary_ip: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "primary_ip6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Option<Box<crate::models::NestedIpAddress>>>,
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
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "config_context", deserialize_with = "Option::deserialize")]
    pub config_context: Option<serde_json::Value>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "interface_count")]
    pub interface_count: i32,
    #[serde(rename = "virtual_disk_count")]
    pub virtual_disk_count: i32,
}

impl VirtualMachineWithConfigContext {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, primary_ip: Option<crate::models::NestedIpAddress>, config_context: Option<serde_json::Value>, created: Option<String>, last_updated: Option<String>, interface_count: i32, virtual_disk_count: i32) -> VirtualMachineWithConfigContext {
        VirtualMachineWithConfigContext {
            id,
            url,
            display,
            name,
            status: None,
            site: None,
            cluster: None,
            device: None,
            role: None,
            tenant: None,
            platform: None,
            primary_ip: if let Some(x) = primary_ip {Some(Box::new(x))} else {None},
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
            config_context,
            created,
            last_updated,
            interface_count,
            virtual_disk_count,
        }
    }
}


