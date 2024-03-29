/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableCableRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableCableRequest {
    /// * `cat3` - CAT3 * `cat5` - CAT5 * `cat5e` - CAT5e * `cat6` - CAT6 * `cat6a` - CAT6a * `cat7` - CAT7 * `cat7a` - CAT7a * `cat8` - CAT8 * `dac-active` - Direct Attach Copper (Active) * `dac-passive` - Direct Attach Copper (Passive) * `mrj21-trunk` - MRJ21 Trunk * `coaxial` - Coaxial * `mmf` - Multimode Fiber * `mmf-om1` - Multimode Fiber (OM1) * `mmf-om2` - Multimode Fiber (OM2) * `mmf-om3` - Multimode Fiber (OM3) * `mmf-om4` - Multimode Fiber (OM4) * `mmf-om5` - Multimode Fiber (OM5) * `smf` - Singlemode Fiber * `smf-os1` - Singlemode Fiber (OS1) * `smf-os2` - Singlemode Fiber (OS2) * `aoc` - Active Optical Cabling (AOC) * `power` - Power
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "a_terminations", skip_serializing_if = "Option::is_none")]
    pub a_terminations: Option<Vec<crate::models::GenericObjectRequest>>,
    #[serde(rename = "b_terminations", skip_serializing_if = "Option::is_none")]
    pub b_terminations: Option<Vec<crate::models::GenericObjectRequest>>,
    /// * `connected` - Connected * `planned` - Planned * `decommissioning` - Decommissioning
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "length", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub length: Option<Option<f64>>,
    /// * `km` - Kilometers * `m` - Meters * `cm` - Centimeters * `mi` - Miles * `ft` - Feet * `in` - Inches
    #[serde(rename = "length_unit", skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<LengthUnit>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableCableRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableCableRequest {
        PatchedWritableCableRequest {
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
/// * `connected` - Connected * `planned` - Planned * `decommissioning` - Decommissioning
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Connected
    }
}
/// * `km` - Kilometers * `m` - Meters * `cm` - Centimeters * `mi` - Miles * `ft` - Feet * `in` - Inches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LengthUnit {
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "ft")]
    Ft,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "")]
    Empty,
}

impl Default for LengthUnit {
    fn default() -> LengthUnit {
        Self::Km
    }
}

