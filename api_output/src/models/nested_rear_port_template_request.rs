/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NestedRearPortTemplateRequest : Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NestedRearPortTemplateRequest {
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
}

impl NestedRearPortTemplateRequest {
    /// Represents an object related through a ForeignKey field. On write, it accepts a primary key (PK) value or a dictionary of attributes which can be used to uniquely identify the related object. This class should be subclassed to return a full representation of the related object on read.
    pub fn new(name: String) -> NestedRearPortTemplateRequest {
        NestedRearPortTemplateRequest {
            name,
        }
    }
}


