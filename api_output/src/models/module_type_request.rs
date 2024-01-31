/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ModuleTypeRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleTypeRequest {
    #[serde(rename = "manufacturer")]
    pub manufacturer: Box<crate::models::NestedManufacturerRequest>,
    #[serde(rename = "model")]
    pub model: String,
    /// Discrete part number (optional)
    #[serde(rename = "part_number", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f64>>,
    /// * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces
    #[serde(rename = "weight_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<Option<WeightUnit>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ModuleTypeRequest {
    /// Adds support for custom fields and tags.
    pub fn new(manufacturer: crate::models::NestedManufacturerRequest, model: String) -> ModuleTypeRequest {
        ModuleTypeRequest {
            manufacturer: Box::new(manufacturer),
            model,
            part_number: None,
            weight: None,
            weight_unit: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeightUnit {
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "oz")]
    Oz,
    #[serde(rename = "")]
    Empty,
}

impl Default for WeightUnit {
    fn default() -> WeightUnit {
        Self::Kg
    }
}
