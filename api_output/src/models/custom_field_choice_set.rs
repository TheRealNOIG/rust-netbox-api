/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldChoiceSet : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldChoiceSet {
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
    #[serde(rename = "base_choices", skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<Box<crate::models::CustomFieldChoiceSetBaseChoices>>,
    #[serde(rename = "extra_choices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extra_choices: Option<Option<Vec<Vec<String>>>>,
    /// Choices are automatically ordered alphabetically
    #[serde(rename = "order_alphabetically", skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
    #[serde(rename = "choices_count")]
    pub choices_count: String,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl CustomFieldChoiceSet {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, name: String, choices_count: String, created: Option<String>, last_updated: Option<String>) -> CustomFieldChoiceSet {
        CustomFieldChoiceSet {
            id,
            url,
            display,
            name,
            description: None,
            base_choices: None,
            extra_choices: None,
            order_alphabetically: None,
            choices_count,
            created,
            last_updated,
        }
    }
}

