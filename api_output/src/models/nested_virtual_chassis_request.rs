/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedVirtualChassisRequest : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedVirtualChassisRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "master")]
    pub master: Box<crate::models::NestedDeviceRequest>,
}

impl NestedVirtualChassisRequest {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(name: String, master: crate::models::NestedDeviceRequest) -> NestedVirtualChassisRequest {
        NestedVirtualChassisRequest {
            name,
            master: Box::new(master),
        }
    }
}


