/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedL2VpnTermination : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedL2VpnTermination {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "l2vpn")]
    pub l2vpn: Box<crate::models::NestedL2Vpn>,
}

impl NestedL2VpnTermination {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(id: i32, url: String, display: String, l2vpn: crate::models::NestedL2Vpn) -> NestedL2VpnTermination {
        NestedL2VpnTermination {
            id,
            url,
            display,
            l2vpn: Box::new(l2vpn),
        }
    }
}


