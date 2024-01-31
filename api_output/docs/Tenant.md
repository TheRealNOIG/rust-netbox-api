# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**group** | Option<[**crate::models::NestedTenantGroup**](NestedTenantGroup.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**circuit_count** | **i32** |  | [readonly]
**device_count** | **i32** |  | [readonly]
**ipaddress_count** | **i32** |  | [readonly]
**prefix_count** | **i32** |  | [readonly]
**rack_count** | **i32** |  | [readonly]
**site_count** | **i32** |  | [readonly]
**virtualmachine_count** | **i32** |  | [readonly]
**vlan_count** | **i32** |  | [readonly]
**vrf_count** | **i32** |  | [readonly]
**cluster_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


