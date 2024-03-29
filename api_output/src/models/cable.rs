/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Cable : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    /// * `cat3` - CAT3 * `cat5` - CAT5 * `cat5e` - CAT5e * `cat6` - CAT6 * `cat6a` - CAT6a * `cat7` - CAT7 * `cat7a` - CAT7a * `cat8` - CAT8 * `dac-active` - Direct Attach Copper (Active) * `dac-passive` - Direct Attach Copper (Passive) * `mrj21-trunk` - MRJ21 Trunk * `coaxial` - Coaxial * `mmf` - Multimode Fiber * `mmf-om1` - Multimode Fiber (OM1) * `mmf-om2` - Multimode Fiber (OM2) * `mmf-om3` - Multimode Fiber (OM3) * `mmf-om4` - Multimode Fiber (OM4) * `mmf-om5` - Multimode Fiber (OM5) * `smf` - Singlemode Fiber * `smf-os1` - Singlemode Fiber (OS1) * `smf-os2` - Singlemode Fiber (OS2) * `aoc` - Active Optical Cabling (AOC) * `power` - Power
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "a_terminations", skip_serializing_if = "Option::is_none")]
    pub a_terminations: Option<Vec<crate::models::GenericObject>>,
    #[serde(rename = "b_terminations", skip_serializing_if = "Option::is_none")]
    pub b_terminations: Option<Vec<crate::models::GenericObject>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::CableStatus>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "length", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub length: Option<Option<f64>>,
    #[serde(rename = "length_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<Option<Box<crate::models::CableLengthUnit>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl Cable {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, created: Option<String>, last_updated: Option<String>) -> Cable {
        Cable {
            id,
            url,
            display,
            r#type: None,
            a_terminations: None,
            b_terminations: None,
            status: None,
            tenant: None,
            label: None,
            color: None,
            length: None,
            length_unit: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}

/// * `cat3` - CAT3 * `cat5` - CAT5 * `cat5e` - CAT5e * `cat6` - CAT6 * `cat6a` - CAT6a * `cat7` - CAT7 * `cat7a` - CAT7a * `cat8` - CAT8 * `dac-active` - Direct Attach Copper (Active) * `dac-passive` - Direct Attach Copper (Passive) * `mrj21-trunk` - MRJ21 Trunk * `coaxial` - Coaxial * `mmf` - Multimode Fiber * `mmf-om1` - Multimode Fiber (OM1) * `mmf-om2` - Multimode Fiber (OM2) * `mmf-om3` - Multimode Fiber (OM3) * `mmf-om4` - Multimode Fiber (OM4) * `mmf-om5` - Multimode Fiber (OM5) * `smf` - Singlemode Fiber * `smf-os1` - Singlemode Fiber (OS1) * `smf-os2` - Singlemode Fiber (OS2) * `aoc` - Active Optical Cabling (AOC) * `power` - Power
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "cat3")]
    Cat3,
    #[serde(rename = "cat5")]
    Cat5,
    #[serde(rename = "cat5e")]
    Cat5e,
    #[serde(rename = "cat6")]
    Cat6,
    #[serde(rename = "cat6a")]
    Cat6a,
    #[serde(rename = "cat7")]
    Cat7,
    #[serde(rename = "cat7a")]
    Cat7a,
    #[serde(rename = "cat8")]
    Cat8,
    #[serde(rename = "dac-active")]
    DacActive,
    #[serde(rename = "dac-passive")]
    DacPassive,
    #[serde(rename = "mrj21-trunk")]
    Mrj21Trunk,
    #[serde(rename = "coaxial")]
    Coaxial,
    #[serde(rename = "mmf")]
    Mmf,
    #[serde(rename = "mmf-om1")]
    MmfOm1,
    #[serde(rename = "mmf-om2")]
    MmfOm2,
    #[serde(rename = "mmf-om3")]
    MmfOm3,
    #[serde(rename = "mmf-om4")]
    MmfOm4,
    #[serde(rename = "mmf-om5")]
    MmfOm5,
    #[serde(rename = "smf")]
    Smf,
    #[serde(rename = "smf-os1")]
    SmfOs1,
    #[serde(rename = "smf-os2")]
    SmfOs2,
    #[serde(rename = "aoc")]
    Aoc,
    #[serde(rename = "power")]
    Power,
    #[serde(rename = "")]
    Empty,
}

impl Default for Type {
    fn default() -> Type {
        Self::Cat3
    }
}

