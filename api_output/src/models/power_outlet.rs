/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PowerOutlet : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerOutlet {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module: Option<Option<Box<crate::models::ComponentNestedModule>>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Box<crate::models::PowerOutletType>>>,
    #[serde(rename = "power_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub power_port: Option<Option<Box<crate::models::NestedPowerPort>>>,
    #[serde(rename = "feed_leg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feed_leg: Option<Option<Box<crate::models::PowerOutletFeedLeg>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", deserialize_with = "Option::deserialize")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    #[serde(rename = "cable_end")]
    pub cable_end: String,
    #[serde(rename = "link_peers")]
    pub link_peers: Vec<serde_json::Value>,
    /// Return the type of the peer link terminations, or None.
    #[serde(rename = "link_peers_type")]
    pub link_peers_type: String,
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
    #[serde(rename = "_occupied")]
    pub _occupied: bool,
}

impl PowerOutlet {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, device: crate::models::NestedDevice, name: String, cable: Option<crate::models::NestedCable>, cable_end: String, link_peers: Vec<serde_json::Value>, link_peers_type: String, connected_endpoints: Vec<serde_json::Value>, connected_endpoints_type: String, connected_endpoints_reachable: bool, created: Option<String>, last_updated: Option<String>, _occupied: bool) -> PowerOutlet {
        PowerOutlet {
            id,
            url,
            display,
            device: Box::new(device),
            module: None,
            name,
            label: None,
            r#type: None,
            power_port: None,
            feed_leg: None,
            description: None,
            mark_connected: None,
            cable: if let Some(x) = cable {Some(Box::new(x))} else {None},
            cable_end,
            link_peers,
            link_peers_type,
            connected_endpoints,
            connected_endpoints_type,
            connected_endpoints_reachable,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            _occupied,
        }
    }
}


