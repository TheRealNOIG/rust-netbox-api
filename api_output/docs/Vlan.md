# Vlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**group** | Option<[**crate::models::NestedVlanGroup**](NestedVLANGroup.md)> |  | [optional]
**vid** | **i32** | Numeric VLAN ID (1-4094) | 
**name** | **String** |  | 
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::IpRangeStatus**](IPRange_status.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRole**](NestedRole.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::NestedL2VpnTermination**](NestedL2VPNTermination.md)> |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**prefix_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


