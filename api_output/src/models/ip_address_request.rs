/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IpAddressRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddressRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<Box<crate::models::NestedVrfRequest>>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenantRequest>>>,
    /// * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated * `dhcp` - DHCP * `slaac` - SLAAC
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// * `loopback` - Loopback * `secondary` - Secondary * `anycast` - Anycast * `vip` - VIP * `vrrp` - VRRP * `hsrp` - HSRP * `glbp` - GLBP * `carp` - CARP
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "assigned_object_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<Option<String>>,
    #[serde(rename = "assigned_object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<Option<i64>>,
    #[serde(rename = "nat_inside", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nat_inside: Option<Option<Box<crate::models::NestedIpAddressRequest>>>,
    /// Hostname or FQDN (not case-sensitive)
    #[serde(rename = "dns_name", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl IpAddressRequest {
    /// Adds support for custom fields and tags.
    pub fn new(address: String) -> IpAddressRequest {
        IpAddressRequest {
            address,
            vrf: None,
            tenant: None,
            status: None,
            role: None,
            assigned_object_type: None,
            assigned_object_id: None,
            nat_inside: None,
            dns_name: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated * `dhcp` - DHCP * `slaac` - SLAAC
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "dhcp")]
    Dhcp,
    #[serde(rename = "slaac")]
    Slaac,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// * `loopback` - Loopback * `secondary` - Secondary * `anycast` - Anycast * `vip` - VIP * `vrrp` - VRRP * `hsrp` - HSRP * `glbp` - GLBP * `carp` - CARP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "loopback")]
    Loopback,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "anycast")]
    Anycast,
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "vrrp")]
    Vrrp,
    #[serde(rename = "hsrp")]
    Hsrp,
    #[serde(rename = "glbp")]
    Glbp,
    #[serde(rename = "carp")]
    Carp,
    #[serde(rename = "")]
    Empty,
}

impl Default for Role {
    fn default() -> Role {
        Self::Loopback
    }
}

