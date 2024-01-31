# VirtualMachineWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**status** | Option<[**crate::models::ModuleStatus**](Module_status.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**cluster** | Option<[**crate::models::NestedCluster**](NestedCluster.md)> |  | [optional]
**device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**role** | Option<[**crate::models::NestedDeviceRole**](NestedDeviceRole.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::NestedPlatform**](NestedPlatform.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [readonly]
**primary_ip4** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**vcpus** | Option<**f64**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**config_context** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**interface_count** | **i32** |  | [readonly]
**virtual_disk_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


