# VirtualDeviceContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**identifier** | Option<**i32**> | Numeric identifier unique to the parent device | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [readonly]
**primary_ip4** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**status** | [**crate::models::VirtualDeviceContextStatus**](VirtualDeviceContext_status.md) |  | 
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**interface_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


