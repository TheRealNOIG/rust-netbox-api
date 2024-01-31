# IpAddressRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** |  | 
**vrf** | Option<[**crate::models::NestedVrfRequest**](NestedVRFRequest.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated * `dhcp` - DHCP * `slaac` - SLAAC | [optional]
**role** | Option<**String**> | * `loopback` - Loopback * `secondary` - Secondary * `anycast` - Anycast * `vip` - VIP * `vrrp` - VRRP * `hsrp` - HSRP * `glbp` - GLBP * `carp` - CARP | [optional]
**assigned_object_type** | Option<**String**> |  | [optional]
**assigned_object_id** | Option<**i64**> |  | [optional]
**nat_inside** | Option<[**crate::models::NestedIpAddressRequest**](NestedIPAddressRequest.md)> |  | [optional]
**dns_name** | Option<**String**> | Hostname or FQDN (not case-sensitive) | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


