/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeviceType : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceType {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "manufacturer")]
    pub manufacturer: Box<crate::models::NestedManufacturer>,
    #[serde(rename = "default_platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_platform: Option<Option<Box<crate::models::NestedPlatform>>>,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "slug")]
    pub slug: String,
    /// Discrete part number (optional)
    #[serde(rename = "part_number", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "u_height", skip_serializing_if = "Option::is_none")]
    pub u_height: Option<f64>,
    /// Devices of this type are excluded when calculating rack utilization.
    #[serde(rename = "exclude_from_utilization", skip_serializing_if = "Option::is_none")]
    pub exclude_from_utilization: Option<bool>,
    /// Device consumes both front and rear rack faces.
    #[serde(rename = "is_full_depth", skip_serializing_if = "Option::is_none")]
    pub is_full_depth: Option<bool>,
    #[serde(rename = "subdevice_role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subdevice_role: Option<Option<Box<crate::models::DeviceTypeSubdeviceRole>>>,
    #[serde(rename = "airflow", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub airflow: Option<Option<Box<crate::models::DeviceTypeAirflow>>>,
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f64>>,
    #[serde(rename = "weight_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<Option<Box<crate::models::DeviceTypeWeightUnit>>>,
    #[serde(rename = "front_image", skip_serializing_if = "Option::is_none")]
    pub front_image: Option<String>,
    #[serde(rename = "rear_image", skip_serializing_if = "Option::is_none")]
    pub rear_image: Option<String>,
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
    #[serde(rename = "device_count")]
    pub device_count: i32,
    #[serde(rename = "console_port_template_count")]
    pub console_port_template_count: i32,
    #[serde(rename = "console_server_port_template_count")]
    pub console_server_port_template_count: i32,
    #[serde(rename = "power_port_template_count")]
    pub power_port_template_count: i32,
    #[serde(rename = "power_outlet_template_count")]
    pub power_outlet_template_count: i32,
    #[serde(rename = "interface_template_count")]
    pub interface_template_count: i32,
    #[serde(rename = "front_port_template_count")]
    pub front_port_template_count: i32,
    #[serde(rename = "rear_port_template_count")]
    pub rear_port_template_count: i32,
    #[serde(rename = "device_bay_template_count")]
    pub device_bay_template_count: i32,
    #[serde(rename = "module_bay_template_count")]
    pub module_bay_template_count: i32,
    #[serde(rename = "inventory_item_template_count")]
    pub inventory_item_template_count: i32,
}

impl DeviceType {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, manufacturer: crate::models::NestedManufacturer, model: String, slug: String, created: Option<String>, last_updated: Option<String>, device_count: i32, console_port_template_count: i32, console_server_port_template_count: i32, power_port_template_count: i32, power_outlet_template_count: i32, interface_template_count: i32, front_port_template_count: i32, rear_port_template_count: i32, device_bay_template_count: i32, module_bay_template_count: i32, inventory_item_template_count: i32) -> DeviceType {
        DeviceType {
            id,
            url,
            display,
            manufacturer: Box::new(manufacturer),
            default_platform: None,
            model,
            slug,
            part_number: None,
            u_height: None,
            exclude_from_utilization: None,
            is_full_depth: None,
            subdevice_role: None,
            airflow: None,
            weight: None,
            weight_unit: None,
            front_image: None,
            rear_image: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            device_count,
            console_port_template_count,
            console_server_port_template_count,
            power_port_template_count,
            power_outlet_template_count,
            interface_template_count,
            front_port_template_count,
            rear_port_template_count,
            device_bay_template_count,
            module_bay_template_count,
            inventory_item_template_count,
        }
    }
}


