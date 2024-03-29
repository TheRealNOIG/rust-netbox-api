/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AvailableIp : Representation of an IP address which does not exist in the database.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableIp {
    #[serde(rename = "family")]
    pub family: i32,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "vrf")]
    pub vrf: Box<crate::models::NestedVrf>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AvailableIp {
    /// Representation of an IP address which does not exist in the database.
    pub fn new(family: i32, address: String, vrf: crate::models::NestedVrf) -> AvailableIp {
        AvailableIp {
            family,
            address,
            vrf: Box::new(vrf),
            description: None,
        }
    }
}


