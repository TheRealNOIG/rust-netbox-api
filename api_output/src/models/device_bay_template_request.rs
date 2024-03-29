/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceBayTemplateRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceBayTemplateRequest {
    #[serde(rename = "device_type")]
    pub device_type: Box<crate::models::NestedDeviceTypeRequest>,
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DeviceBayTemplateRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(device_type: crate::models::NestedDeviceTypeRequest, name: String) -> DeviceBayTemplateRequest {
        DeviceBayTemplateRequest {
            device_type: Box::new(device_type),
            name,
            label: None,
            description: None,
        }
    }
}


