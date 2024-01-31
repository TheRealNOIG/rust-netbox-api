/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FrontPortRearPort : NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontPortRearPort {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FrontPortRearPort {
    /// NestedRearPortSerializer but with parent device omitted (since front and rear ports must belong to same device)
    pub fn new(id: i32, url: String, display: String, name: String) -> FrontPortRearPort {
        FrontPortRearPort {
            id,
            url,
            display,
            name,
            label: None,
            description: None,
        }
    }
}


