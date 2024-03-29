/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableIpSecPolicyRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableIpSecPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "proposals", skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<i32>>,
    /// Diffie-Hellman group for Perfect Forward Secrecy  * `1` - Group 1 * `2` - Group 2 * `5` - Group 5 * `14` - Group 14 * `15` - Group 15 * `16` - Group 16 * `17` - Group 17 * `18` - Group 18 * `19` - Group 19 * `20` - Group 20 * `21` - Group 21 * `22` - Group 22 * `23` - Group 23 * `24` - Group 24 * `25` - Group 25 * `26` - Group 26 * `27` - Group 27 * `28` - Group 28 * `29` - Group 29 * `30` - Group 30 * `31` - Group 31 * `32` - Group 32 * `33` - Group 33 * `34` - Group 34
    #[serde(rename = "pfs_group", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pfs_group: Option<Option<PfsGroup>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableIpSecPolicyRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableIpSecPolicyRequest {
        PatchedWritableIpSecPolicyRequest {
            name: None,
            description: None,
            proposals: None,
            pfs_group: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// Diffie-Hellman group for Perfect Forward Secrecy  * `1` - Group 1 * `2` - Group 2 * `5` - Group 5 * `14` - Group 14 * `15` - Group 15 * `16` - Group 16 * `17` - Group 17 * `18` - Group 18 * `19` - Group 19 * `20` - Group 20 * `21` - Group 21 * `22` - Group 22 * `23` - Group 23 * `24` - Group 24 * `25` - Group 25 * `26` - Group 26 * `27` - Group 27 * `28` - Group 28 * `29` - Group 29 * `30` - Group 30 * `31` - Group 31 * `32` - Group 32 * `33` - Group 33 * `34` - Group 34
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PfsGroup {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "14")]
    Variant14,
    #[serde(rename = "15")]
    Variant15,
    #[serde(rename = "16")]
    Variant16,
    #[serde(rename = "17")]
    Variant17,
    #[serde(rename = "18")]
    Variant18,
    #[serde(rename = "19")]
    Variant19,
    #[serde(rename = "20")]
    Variant20,
    #[serde(rename = "21")]
    Variant21,
    #[serde(rename = "22")]
    Variant22,
    #[serde(rename = "23")]
    Variant23,
    #[serde(rename = "24")]
    Variant24,
    #[serde(rename = "25")]
    Variant25,
    #[serde(rename = "26")]
    Variant26,
    #[serde(rename = "27")]
    Variant27,
    #[serde(rename = "28")]
    Variant28,
    #[serde(rename = "29")]
    Variant29,
    #[serde(rename = "30")]
    Variant30,
    #[serde(rename = "31")]
    Variant31,
    #[serde(rename = "32")]
    Variant32,
    #[serde(rename = "33")]
    Variant33,
    #[serde(rename = "34")]
    Variant34,
}

impl Default for PfsGroup {
    fn default() -> PfsGroup {
        Self::Variant1
    }
}

