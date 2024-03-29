/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableL2VpnRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableL2VpnRequest {
    #[serde(rename = "identifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Option<i64>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// * `vpws` - VPWS * `vpls` - VPLS * `vxlan` - VXLAN * `vxlan-evpn` - VXLAN-EVPN * `mpls-evpn` - MPLS EVPN * `pbb-evpn` - PBB EVPN * `epl` - EPL * `evpl` - EVPL * `ep-lan` - Ethernet Private LAN * `evp-lan` - Ethernet Virtual Private LAN * `ep-tree` - Ethernet Private Tree * `evp-tree` - Ethernet Virtual Private Tree
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "import_targets", skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<i32>>,
    #[serde(rename = "export_targets", skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableL2VpnRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableL2VpnRequest {
        PatchedWritableL2VpnRequest {
            identifier: None,
            name: None,
            slug: None,
            r#type: None,
            import_targets: None,
            export_targets: None,
            description: None,
            comments: None,
            tenant: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `vpws` - VPWS * `vpls` - VPLS * `vxlan` - VXLAN * `vxlan-evpn` - VXLAN-EVPN * `mpls-evpn` - MPLS EVPN * `pbb-evpn` - PBB EVPN * `epl` - EPL * `evpl` - EVPL * `ep-lan` - Ethernet Private LAN * `evp-lan` - Ethernet Virtual Private LAN * `ep-tree` - Ethernet Private Tree * `evp-tree` - Ethernet Virtual Private Tree
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "vpws")]
    Vpws,
    #[serde(rename = "vpls")]
    Vpls,
    #[serde(rename = "vxlan")]
    Vxlan,
    #[serde(rename = "vxlan-evpn")]
    VxlanEvpn,
    #[serde(rename = "mpls-evpn")]
    MplsEvpn,
    #[serde(rename = "pbb-evpn")]
    PbbEvpn,
    #[serde(rename = "epl")]
    Epl,
    #[serde(rename = "evpl")]
    Evpl,
    #[serde(rename = "ep-lan")]
    EpLan,
    #[serde(rename = "evp-lan")]
    EvpLan,
    #[serde(rename = "ep-tree")]
    EpTree,
    #[serde(rename = "evp-tree")]
    EvpTree,
}

impl Default for Type {
    fn default() -> Type {
        Self::Vpws
    }
}

