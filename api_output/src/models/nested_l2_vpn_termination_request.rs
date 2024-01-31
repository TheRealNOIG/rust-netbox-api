/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedL2VpnTerminationRequest : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedL2VpnTerminationRequest {
    #[serde(rename = "l2vpn")]
    pub l2vpn: Box<crate::models::NestedL2VpnRequest>,
}

impl NestedL2VpnTerminationRequest {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(l2vpn: crate::models::NestedL2VpnRequest) -> NestedL2VpnTerminationRequest {
        NestedL2VpnTerminationRequest {
            l2vpn: Box::new(l2vpn),
        }
    }
}


