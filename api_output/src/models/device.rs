/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Device : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "device_type")]
    pub device_type: Box<crate::models::NestedDeviceType>,
    #[serde(rename = "role")]
    pub role: Box<crate::models::NestedDeviceRole>,
    #[serde(rename = "device_role")]
    pub device_role: Box<crate::models::DeviceDeviceRole>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Option<Box<crate::models::NestedPlatform>>>,
    /// Chassis serial number, assigned by the manufacturer
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this device
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    #[serde(rename = "site")]
    pub site: Box<crate::models::NestedSite>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<Box<crate::models::NestedLocation>>>,
    #[serde(rename = "rack", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rack: Option<Option<Box<crate::models::NestedRack>>>,
    #[serde(rename = "position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<f64>>,
    #[serde(rename = "face", skip_serializing_if = "Option::is_none")]
    pub face: Option<Box<crate::models::DeviceFace>>,
    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[serde(rename = "latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f64>>,
    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[serde(rename = "longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f64>>,
    #[serde(rename = "parent_device", deserialize_with = "Option::deserialize")]
    pub parent_device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::DeviceStatus>>,
    #[serde(rename = "airflow", skip_serializing_if = "Option::is_none")]
    pub airflow: Option<Box<crate::models::DeviceAirflow>>,
    #[serde(rename = "primary_ip", deserialize_with = "Option::deserialize")]
    pub primary_ip: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "primary_ip6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "oob_ip", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub oob_ip: Option<Option<Box<crate::models::NestedIpAddress>>>,
    #[serde(rename = "cluster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<Box<crate::models::NestedCluster>>>,
    #[serde(rename = "virtual_chassis", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_chassis: Option<Option<Box<crate::models::NestedVirtualChassis>>>,
    #[serde(rename = "vc_position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vc_position: Option<Option<i32>>,
    /// Virtual chassis master election priority
    #[serde(rename = "vc_priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vc_priority: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "config_template", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_template: Option<Option<Box<crate::models::NestedConfigTemplate>>>,
    /// Local config context data takes precedence over source contexts in the final rendered config context
    #[serde(rename = "local_context_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Option<serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "console_port_count")]
    pub console_port_count: i32,
    #[serde(rename = "console_server_port_count")]
    pub console_server_port_count: i32,
    #[serde(rename = "power_port_count")]
    pub power_port_count: i32,
    #[serde(rename = "power_outlet_count")]
    pub power_outlet_count: i32,
    #[serde(rename = "interface_count")]
    pub interface_count: i32,
    #[serde(rename = "front_port_count")]
    pub front_port_count: i32,
    #[serde(rename = "rear_port_count")]
    pub rear_port_count: i32,
    #[serde(rename = "device_bay_count")]
    pub device_bay_count: i32,
    #[serde(rename = "module_bay_count")]
    pub module_bay_count: i32,
    #[serde(rename = "inventory_item_count")]
    pub inventory_item_count: i32,
}

impl Device {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, device_type: crate::models::NestedDeviceType, role: crate::models::NestedDeviceRole, device_role: crate::models::DeviceDeviceRole, site: crate::models::NestedSite, parent_device: Option<crate::models::NestedDevice>, primary_ip: Option<crate::models::NestedIpAddress>, created: Option<String>, last_updated: Option<String>, console_port_count: i32, console_server_port_count: i32, power_port_count: i32, power_outlet_count: i32, interface_count: i32, front_port_count: i32, rear_port_count: i32, device_bay_count: i32, module_bay_count: i32, inventory_item_count: i32) -> Device {
        Device {
            id,
            url,
            display,
            name: None,
            device_type: Box::new(device_type),
            role: Box::new(role),
            device_role: Box::new(device_role),
            tenant: None,
            platform: None,
            serial: None,
            asset_tag: None,
            site: Box::new(site),
            location: None,
            rack: None,
            position: None,
            face: None,
            latitude: None,
            longitude: None,
            parent_device: if let Some(x) = parent_device {Some(Box::new(x))} else {None},
            status: None,
            airflow: None,
            primary_ip: if let Some(x) = primary_ip {Some(Box::new(x))} else {None},
            primary_ip4: None,
            primary_ip6: None,
            oob_ip: None,
            cluster: None,
            virtual_chassis: None,
            vc_position: None,
            vc_priority: None,
            description: None,
            comments: None,
            config_template: None,
            local_context_data: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            console_port_count,
            console_server_port_count,
            power_port_count,
            power_outlet_count,
            interface_count,
            front_port_count,
            rear_port_count,
            device_bay_count,
            module_bay_count,
            inventory_item_count,
        }
    }
}


