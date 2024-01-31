# VirtualMachineWithConfigContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning | [optional]
**site** | Option<[**crate::models::NestedSiteRequest**](NestedSiteRequest.md)> |  | [optional]
**cluster** | Option<[**crate::models::NestedClusterRequest**](NestedClusterRequest.md)> |  | [optional]
**device** | Option<[**crate::models::NestedDeviceRequest**](NestedDeviceRequest.md)> |  | [optional]
**role** | Option<[**crate::models::NestedDeviceRoleRequest**](NestedDeviceRoleRequest.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**platform** | Option<[**crate::models::NestedPlatformRequest**](NestedPlatformRequest.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::NestedIpAddressRequest**](NestedIPAddressRequest.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddressRequest**](NestedIPAddressRequest.md)> |  | [optional]
**vcpus** | Option<**f64**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


