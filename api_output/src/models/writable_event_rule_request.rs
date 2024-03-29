/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableEventRuleRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableEventRuleRequest {
    #[serde(rename = "content_types")]
    pub content_types: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// Triggers when a matching object is created.
    #[serde(rename = "type_create", skip_serializing_if = "Option::is_none")]
    pub type_create: Option<bool>,
    /// Triggers when a matching object is updated.
    #[serde(rename = "type_update", skip_serializing_if = "Option::is_none")]
    pub type_update: Option<bool>,
    /// Triggers when a matching object is deleted.
    #[serde(rename = "type_delete", skip_serializing_if = "Option::is_none")]
    pub type_delete: Option<bool>,
    /// Triggers when a job for a matching object is started.
    #[serde(rename = "type_job_start", skip_serializing_if = "Option::is_none")]
    pub type_job_start: Option<bool>,
    /// Triggers when a job for a matching object terminates.
    #[serde(rename = "type_job_end", skip_serializing_if = "Option::is_none")]
    pub type_job_end: Option<bool>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A set of conditions which determine whether the event will be generated.
    #[serde(rename = "conditions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Option<serde_json::Value>>,
    /// * `webhook` - Webhook * `script` - Script
    #[serde(rename = "action_type", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionType>,
    #[serde(rename = "action_object_type")]
    pub action_object_type: String,
    #[serde(rename = "action_object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub action_object_id: Option<Option<i64>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
}

impl WritableEventRuleRequest {
    /// Adds support for custom fields and tags.
    pub fn new(content_types: Vec<String>, name: String, action_object_type: String) -> WritableEventRuleRequest {
        WritableEventRuleRequest {
            content_types,
            name,
            type_create: None,
            type_update: None,
            type_delete: None,
            type_job_start: None,
            type_job_end: None,
            enabled: None,
            conditions: None,
            action_type: None,
            action_object_type,
            action_object_id: None,
            description: None,
            custom_fields: None,
            tags: None,
        }
    }
}

/// * `webhook` - Webhook * `script` - Script
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "script")]
    Script,
}

impl Default for ActionType {
    fn default() -> ActionType {
        Self::Webhook
    }
}

