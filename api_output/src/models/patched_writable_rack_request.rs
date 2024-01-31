/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableRackRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableRackRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "facility_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub facility_id: Option<Option<String>>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    /// * `reserved` - Reserved * `available` - Available * `planned` - Planned * `active` - Active * `deprecated` - Deprecated
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Functional role
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this rack
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    /// * `2-post-frame` - 2-post frame * `4-post-frame` - 4-post frame * `4-post-cabinet` - 4-post cabinet * `wall-frame` - Wall-mounted frame * `wall-frame-vertical` - Wall-mounted frame (vertical) * `wall-cabinet` - Wall-mounted cabinet * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Rail-to-rail width  * `10` - 10 inches * `19` - 19 inches * `21` - 21 inches * `23` - 23 inches
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Width>,
    /// Height in rack units
    #[serde(rename = "u_height", skip_serializing_if = "Option::is_none")]
    pub u_height: Option<i32>,
    /// Starting unit for rack
    #[serde(rename = "starting_unit", skip_serializing_if = "Option::is_none")]
    pub starting_unit: Option<i32>,
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f64>>,
    /// Maximum load capacity for the rack
    #[serde(rename = "max_weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_weight: Option<Option<i32>>,
    /// * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces
    #[serde(rename = "weight_unit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<WeightUnit>,
    /// Units are numbered top-to-bottom
    #[serde(rename = "desc_units", skip_serializing_if = "Option::is_none")]
    pub desc_units: Option<bool>,
    /// Outer dimension of rack (width)
    #[serde(rename = "outer_width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outer_width: Option<Option<i32>>,
    /// Outer dimension of rack (depth)
    #[serde(rename = "outer_depth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outer_depth: Option<Option<i32>>,
    /// * `mm` - Millimeters * `in` - Inches
    #[serde(rename = "outer_unit", skip_serializing_if = "Option::is_none")]
    pub outer_unit: Option<OuterUnit>,
    /// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
    #[serde(rename = "mounting_depth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mounting_depth: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableRackRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableRackRequest {
        PatchedWritableRackRequest {
            name: None,
            facility_id: None,
            site: None,
            location: None,
            tenant: None,
            status: None,
            role: None,
            serial: None,
            asset_tag: None,
            r#type: None,
            width: None,
            u_height: None,
            starting_unit: None,
            weight: None,
            max_weight: None,
            weight_unit: None,
            desc_units: None,
            outer_width: None,
            outer_depth: None,
            outer_unit: None,
            mounting_depth: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `reserved` - Reserved * `available` - Available * `planned` - Planned * `active` - Active * `deprecated` - Deprecated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Reserved
    }
}
/// * `2-post-frame` - 2-post frame * `4-post-frame` - 4-post frame * `4-post-cabinet` - 4-post cabinet * `wall-frame` - Wall-mounted frame * `wall-frame-vertical` - Wall-mounted frame (vertical) * `wall-cabinet` - Wall-mounted cabinet * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "2-post-frame")]
    Variant2PostFrame,
    #[serde(rename = "4-post-frame")]
    Variant4PostFrame,
    #[serde(rename = "4-post-cabinet")]
    Variant4PostCabinet,
    #[serde(rename = "wall-frame")]
    WallFrame,
    #[serde(rename = "wall-frame-vertical")]
    WallFrameVertical,
    #[serde(rename = "wall-cabinet")]
    WallCabinet,
    #[serde(rename = "wall-cabinet-vertical")]
    WallCabinetVertical,
    #[serde(rename = "")]
    Empty,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant2PostFrame
    }
}
/// Rail-to-rail width  * `10` - 10 inches * `19` - 19 inches * `21` - 21 inches * `23` - 23 inches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Width {
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "19")]
    Variant19,
    #[serde(rename = "21")]
    Variant21,
    #[serde(rename = "23")]
    Variant23,
}

impl Default for Width {
    fn default() -> Width {
        Self::Variant10
    }
}
/// * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeightUnit {
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "oz")]
    Oz,
    #[serde(rename = "")]
    Empty,
}

impl Default for WeightUnit {
    fn default() -> WeightUnit {
        Self::Kg
    }
}
/// * `mm` - Millimeters * `in` - Inches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OuterUnit {
    #[serde(rename = "mm")]
    Mm,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "")]
    Empty,
}

impl Default for OuterUnit {
    fn default() -> OuterUnit {
        Self::Mm
    }
}

