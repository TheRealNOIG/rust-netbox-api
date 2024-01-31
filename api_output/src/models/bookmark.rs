/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Bookmark : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "object_type")]
    pub object_type: String,
    #[serde(rename = "object_id")]
    pub object_id: i64,
    #[serde(rename = "object", deserialize_with = "Option::deserialize")]
    pub object: Option<serde_json::Value>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::NestedUser>,
    #[serde(rename = "created")]
    pub created: String,
}

impl Bookmark {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, object_type: String, object_id: i64, object: Option<serde_json::Value>, user: crate::models::NestedUser, created: String) -> Bookmark {
        Bookmark {
            id,
            url,
            display,
            object_type,
            object_id,
            object,
            user: Box::new(user),
            created,
        }
    }
}


