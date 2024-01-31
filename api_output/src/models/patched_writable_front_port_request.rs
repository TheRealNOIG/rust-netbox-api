/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableFrontPortRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableFrontPortRequest {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module: Option<Option<i32>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// * `8p8c` - 8P8C * `8p6c` - 8P6C * `8p4c` - 8P4C * `8p2c` - 8P2C * `6p6c` - 6P6C * `6p4c` - 6P4C * `6p2c` - 6P2C * `4p4c` - 4P4C * `4p2c` - 4P2C * `gg45` - GG45 * `tera-4p` - TERA 4P * `tera-2p` - TERA 2P * `tera-1p` - TERA 1P * `110-punch` - 110 Punch * `bnc` - BNC * `f` - F Connector * `n` - N Connector * `mrj21` - MRJ21 * `fc` - FC * `lc` - LC * `lc-pc` - LC/PC * `lc-upc` - LC/UPC * `lc-apc` - LC/APC * `lsh` - LSH * `lsh-pc` - LSH/PC * `lsh-upc` - LSH/UPC * `lsh-apc` - LSH/APC * `lx5` - LX.5 * `lx5-pc` - LX.5/PC * `lx5-upc` - LX.5/UPC * `lx5-apc` - LX.5/APC * `mpo` - MPO * `mtrj` - MTRJ * `sc` - SC * `sc-pc` - SC/PC * `sc-upc` - SC/UPC * `sc-apc` - SC/APC * `st` - ST * `cs` - CS * `sn` - SN * `sma-905` - SMA 905 * `sma-906` - SMA 906 * `urm-p2` - URM-P2 * `urm-p4` - URM-P4 * `urm-p8` - URM-P8 * `splice` - Splice * `other` - Other
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "rear_port", skip_serializing_if = "Option::is_none")]
    pub rear_port: Option<i32>,
    /// Mapped position on corresponding rear port
    #[serde(rename = "rear_port_position", skip_serializing_if = "Option::is_none")]
    pub rear_port_position: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableFrontPortRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableFrontPortRequest {
        PatchedWritableFrontPortRequest {
            device: None,
            module: None,
            name: None,
            label: None,
            r#type: None,
            color: None,
            rear_port: None,
            rear_port_position: None,
            description: None,
            mark_connected: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `8p8c` - 8P8C * `8p6c` - 8P6C * `8p4c` - 8P4C * `8p2c` - 8P2C * `6p6c` - 6P6C * `6p4c` - 6P4C * `6p2c` - 6P2C * `4p4c` - 4P4C * `4p2c` - 4P2C * `gg45` - GG45 * `tera-4p` - TERA 4P * `tera-2p` - TERA 2P * `tera-1p` - TERA 1P * `110-punch` - 110 Punch * `bnc` - BNC * `f` - F Connector * `n` - N Connector * `mrj21` - MRJ21 * `fc` - FC * `lc` - LC * `lc-pc` - LC/PC * `lc-upc` - LC/UPC * `lc-apc` - LC/APC * `lsh` - LSH * `lsh-pc` - LSH/PC * `lsh-upc` - LSH/UPC * `lsh-apc` - LSH/APC * `lx5` - LX.5 * `lx5-pc` - LX.5/PC * `lx5-upc` - LX.5/UPC * `lx5-apc` - LX.5/APC * `mpo` - MPO * `mtrj` - MTRJ * `sc` - SC * `sc-pc` - SC/PC * `sc-upc` - SC/UPC * `sc-apc` - SC/APC * `st` - ST * `cs` - CS * `sn` - SN * `sma-905` - SMA 905 * `sma-906` - SMA 906 * `urm-p2` - URM-P2 * `urm-p4` - URM-P4 * `urm-p8` - URM-P8 * `splice` - Splice * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "8p8c")]
    Variant8p8c,
    #[serde(rename = "8p6c")]
    Variant8p6c,
    #[serde(rename = "8p4c")]
    Variant8p4c,
    #[serde(rename = "8p2c")]
    Variant8p2c,
    #[serde(rename = "6p6c")]
    Variant6p6c,
    #[serde(rename = "6p4c")]
    Variant6p4c,
    #[serde(rename = "6p2c")]
    Variant6p2c,
    #[serde(rename = "4p4c")]
    Variant4p4c,
    #[serde(rename = "4p2c")]
    Variant4p2c,
    #[serde(rename = "gg45")]
    Gg45,
    #[serde(rename = "tera-4p")]
    Tera4p,
    #[serde(rename = "tera-2p")]
    Tera2p,
    #[serde(rename = "tera-1p")]
    Tera1p,
    #[serde(rename = "110-punch")]
    Variant110Punch,
    #[serde(rename = "bnc")]
    Bnc,
    #[serde(rename = "f")]
    F,
    #[serde(rename = "n")]
    N,
    #[serde(rename = "mrj21")]
    Mrj21,
    #[serde(rename = "fc")]
    Fc,
    #[serde(rename = "lc")]
    Lc,
    #[serde(rename = "lc-pc")]
    LcPc,
    #[serde(rename = "lc-upc")]
    LcUpc,
    #[serde(rename = "lc-apc")]
    LcApc,
    #[serde(rename = "lsh")]
    Lsh,
    #[serde(rename = "lsh-pc")]
    LshPc,
    #[serde(rename = "lsh-upc")]
    LshUpc,
    #[serde(rename = "lsh-apc")]
    LshApc,
    #[serde(rename = "lx5")]
    Lx5,
    #[serde(rename = "lx5-pc")]
    Lx5Pc,
    #[serde(rename = "lx5-upc")]
    Lx5Upc,
    #[serde(rename = "lx5-apc")]
    Lx5Apc,
    #[serde(rename = "mpo")]
    Mpo,
    #[serde(rename = "mtrj")]
    Mtrj,
    #[serde(rename = "sc")]
    Sc,
    #[serde(rename = "sc-pc")]
    ScPc,
    #[serde(rename = "sc-upc")]
    ScUpc,
    #[serde(rename = "sc-apc")]
    ScApc,
    #[serde(rename = "st")]
    St,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "sma-905")]
    Sma905,
    #[serde(rename = "sma-906")]
    Sma906,
    #[serde(rename = "urm-p2")]
    UrmP2,
    #[serde(rename = "urm-p4")]
    UrmP4,
    #[serde(rename = "urm-p8")]
    UrmP8,
    #[serde(rename = "splice")]
    Splice,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant8p8c
    }
}

