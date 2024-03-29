/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TokenRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenRequest {
    #[serde(rename = "user")]
    pub user: Box<crate::models::NestedUserRequest>,
    #[serde(rename = "expires", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    #[serde(rename = "last_used", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<Option<String>>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Permit create/update/delete operations using this key
    #[serde(rename = "write_enabled", skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TokenRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(user: crate::models::NestedUserRequest) -> TokenRequest {
        TokenRequest {
            user: Box::new(user),
            expires: None,
            last_used: None,
            key: None,
            write_enabled: None,
            description: None,
        }
    }
}


