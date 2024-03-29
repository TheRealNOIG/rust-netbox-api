/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableFhrpGroupAssignmentRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableFhrpGroupAssignmentRequest {
    #[serde(rename = "group")]
    pub group: i32,
    #[serde(rename = "interface_type")]
    pub interface_type: String,
    #[serde(rename = "interface_id")]
    pub interface_id: i64,
    #[serde(rename = "priority")]
    pub priority: i32,
}

impl WritableFhrpGroupAssignmentRequest {
    /// Adds support for custom fields and tags.
    pub fn new(group: i32, interface_type: String, interface_id: i64, priority: i32) -> WritableFhrpGroupAssignmentRequest {
        WritableFhrpGroupAssignmentRequest {
            group,
            interface_type,
            interface_id,
            priority,
        }
    }
}


