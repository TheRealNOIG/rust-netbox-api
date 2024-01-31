/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableObjectPermissionRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableObjectPermissionRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "object_types", skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i32>>,
    /// The list of actions granted by this permission
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Queryset filter matching the applicable objects of the selected type(s)
    #[serde(rename = "constraints", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Option<serde_json::Value>>,
}

impl PatchedWritableObjectPermissionRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new() -> PatchedWritableObjectPermissionRequest {
        PatchedWritableObjectPermissionRequest {
            name: None,
            description: None,
            enabled: None,
            object_types: None,
            groups: None,
            users: None,
            actions: None,
            constraints: None,
        }
    }
}


