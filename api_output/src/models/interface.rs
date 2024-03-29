/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Interface : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interface {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "vdcs", skip_serializing_if = "Option::is_none")]
    pub vdcs: Option<Vec<i32>>,
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module: Option<Option<Box<crate::models::ComponentNestedModule>>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::InterfaceType>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<Box<crate::models::NestedInterface>>>,
    #[serde(rename = "bridge", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Option<Box<crate::models::NestedInterface>>>,
    #[serde(rename = "lag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lag: Option<Option<Box<crate::models::NestedInterface>>>,
    #[serde(rename = "mtu", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<Option<i32>>,
    #[serde(rename = "mac_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Option<String>>,
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<i32>>,
    #[serde(rename = "duplex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duplex: Option<Option<Box<crate::models::InterfaceDuplex>>>,
    #[serde(rename = "wwn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wwn: Option<Option<String>>,
    /// This interface is used only for out-of-band management
    #[serde(rename = "mgmt_only", skip_serializing_if = "Option::is_none")]
    pub mgmt_only: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<crate::models::InterfaceMode>>,
    #[serde(rename = "rf_role", skip_serializing_if = "Option::is_none")]
    pub rf_role: Option<Box<crate::models::InterfaceRfRole>>,
    #[serde(rename = "rf_channel", skip_serializing_if = "Option::is_none")]
    pub rf_channel: Option<Box<crate::models::InterfaceRfChannel>>,
    #[serde(rename = "poe_mode", skip_serializing_if = "Option::is_none")]
    pub poe_mode: Option<Box<crate::models::InterfacePoeMode>>,
    #[serde(rename = "poe_type", skip_serializing_if = "Option::is_none")]
    pub poe_type: Option<Box<crate::models::InterfacePoeType>>,
    /// Populated by selected channel (if set)
    #[serde(rename = "rf_channel_frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rf_channel_frequency: Option<Option<f64>>,
    /// Populated by selected channel (if set)
    #[serde(rename = "rf_channel_width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rf_channel_width: Option<Option<f64>>,
    #[serde(rename = "tx_power", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tx_power: Option<Option<i32>>,
    #[serde(rename = "untagged_vlan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<Option<Box<crate::models::NestedVlan>>>,
    #[serde(rename = "tagged_vlans", skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", deserialize_with = "Option::deserialize")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    #[serde(rename = "cable_end")]
    pub cable_end: String,
    #[serde(rename = "wireless_link", deserialize_with = "Option::deserialize")]
    pub wireless_link: Option<Box<crate::models::NestedWirelessLink>>,
    #[serde(rename = "link_peers")]
    pub link_peers: Vec<serde_json::Value>,
    /// Return the type of the peer link terminations, or None.
    #[serde(rename = "link_peers_type")]
    pub link_peers_type: String,
    #[serde(rename = "wireless_lans", skip_serializing_if = "Option::is_none")]
    pub wireless_lans: Option<Vec<i32>>,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<Box<crate::models::NestedVrf>>>,
    #[serde(rename = "l2vpn_termination", deserialize_with = "Option::deserialize")]
    pub l2vpn_termination: Option<Box<crate::models::NestedL2VpnTermination>>,
    #[serde(rename = "connected_endpoints")]
    pub connected_endpoints: Vec<serde_json::Value>,
    #[serde(rename = "connected_endpoints_type")]
    pub connected_endpoints_type: String,
    #[serde(rename = "connected_endpoints_reachable")]
    pub connected_endpoints_reachable: bool,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "count_ipaddresses")]
    pub count_ipaddresses: i32,
    #[serde(rename = "count_fhrp_groups")]
    pub count_fhrp_groups: i32,
    #[serde(rename = "_occupied")]
    pub _occupied: bool,
}

impl Interface {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, device: crate::models::NestedDevice, name: String, r#type: crate::models::InterfaceType, cable: Option<crate::models::NestedCable>, cable_end: String, wireless_link: Option<crate::models::NestedWirelessLink>, link_peers: Vec<serde_json::Value>, link_peers_type: String, l2vpn_termination: Option<crate::models::NestedL2VpnTermination>, connected_endpoints: Vec<serde_json::Value>, connected_endpoints_type: String, connected_endpoints_reachable: bool, created: Option<String>, last_updated: Option<String>, count_ipaddresses: i32, count_fhrp_groups: i32, _occupied: bool) -> Interface {
        Interface {
            id,
            url,
            display,
            device: Box::new(device),
            vdcs: None,
            module: None,
            name,
            label: None,
            r#type: Box::new(r#type),
            enabled: None,
            parent: None,
            bridge: None,
            lag: None,
            mtu: None,
            mac_address: None,
            speed: None,
            duplex: None,
            wwn: None,
            mgmt_only: None,
            description: None,
            mode: None,
            rf_role: None,
            rf_channel: None,
            poe_mode: None,
            poe_type: None,
            rf_channel_frequency: None,
            rf_channel_width: None,
            tx_power: None,
            untagged_vlan: None,
            tagged_vlans: None,
            mark_connected: None,
            cable: if let Some(x) = cable {Some(Box::new(x))} else {None},
            cable_end,
            wireless_link: if let Some(x) = wireless_link {Some(Box::new(x))} else {None},
            link_peers,
            link_peers_type,
            wireless_lans: None,
            vrf: None,
            l2vpn_termination: if let Some(x) = l2vpn_termination {Some(Box::new(x))} else {None},
            connected_endpoints,
            connected_endpoints_type,
            connected_endpoints_reachable,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            count_ipaddresses,
            count_fhrp_groups,
            _occupied,
        }
    }
}


