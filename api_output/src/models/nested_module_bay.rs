/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedModuleBay : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedModuleBay {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "module", deserialize_with = "Option::deserialize")]
    pub module: Option<Box<crate::models::NestedModule>>,
    #[serde(rename = "name")]
    pub name: String,
}

impl NestedModuleBay {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(id: i32, url: String, display: String, module: Option<crate::models::NestedModule>, name: String) -> NestedModuleBay {
        NestedModuleBay {
            id,
            url,
            display,
            module: if let Some(x) = module {Some(Box::new(x))} else {None},
            name,
        }
    }
}


