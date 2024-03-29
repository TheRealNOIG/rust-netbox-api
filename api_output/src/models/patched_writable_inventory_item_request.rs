/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableInventoryItemRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableInventoryItemRequest {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<i32>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    #[serde(rename = "manufacturer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Option<i32>>,
    /// Manufacturer-assigned part identifier
    #[serde(rename = "part_id", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this item
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    /// This item was automatically discovered
    #[serde(rename = "discovered", skip_serializing_if = "Option::is_none")]
    pub discovered: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "component_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<Option<String>>,
    #[serde(rename = "component_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub component_id: Option<Option<i64>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableInventoryItemRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableInventoryItemRequest {
        PatchedWritableInventoryItemRequest {
            device: None,
            parent: None,
            name: None,
            label: None,
            role: None,
            manufacturer: None,
            part_id: None,
            serial: None,
            asset_tag: None,
            discovered: None,
            description: None,
            component_type: None,
            component_id: None,
            tags: None,
            custom_fields: None,
        }
    }
}


