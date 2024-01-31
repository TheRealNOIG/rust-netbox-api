/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedInterface : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedInterface {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "cable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Option<i32>>,
    #[serde(rename = "_occupied")]
    pub _occupied: bool,
}

impl NestedInterface {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(id: i32, url: String, display: String, device: crate::models::NestedDevice, name: String, _occupied: bool) -> NestedInterface {
        NestedInterface {
            id,
            url,
            display,
            device: Box::new(device),
            name,
            cable: None,
            _occupied,
        }
    }
}


