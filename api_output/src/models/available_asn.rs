/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AvailableAsn : Representation of an ASN which does not exist in the database.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableAsn {
    #[serde(rename = "asn")]
    pub asn: i32,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AvailableAsn {
    /// Representation of an ASN which does not exist in the database.
    pub fn new(asn: i32) -> AvailableAsn {
        AvailableAsn {
            asn,
            description: None,
        }
    }
}


