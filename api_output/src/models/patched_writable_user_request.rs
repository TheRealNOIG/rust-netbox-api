/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableUserRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableUserRequest {
    /// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Designates whether the user can log into this admin site.
    #[serde(rename = "is_staff", skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    /// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "date_joined", skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// The groups this user belongs to. A user will get all permissions granted to each of their groups.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
}

impl PatchedWritableUserRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new() -> PatchedWritableUserRequest {
        PatchedWritableUserRequest {
            username: None,
            password: None,
            first_name: None,
            last_name: None,
            email: None,
            is_staff: None,
            is_active: None,
            date_joined: None,
            groups: None,
        }
    }
}


