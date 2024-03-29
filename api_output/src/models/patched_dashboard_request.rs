/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedDashboardRequest {
    #[serde(rename = "layout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Option<serde_json::Value>>,
    #[serde(rename = "config", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config: Option<Option<serde_json::Value>>,
}

impl PatchedDashboardRequest {
    pub fn new() -> PatchedDashboardRequest {
        PatchedDashboardRequest {
            layout: None,
            config: None,
        }
    }
}


