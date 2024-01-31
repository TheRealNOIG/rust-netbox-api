# IpRangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_address** | **String** |  | 
**end_address** | **String** |  | 
**vrf** | Option<[**crate::models::NestedVrfRequest**](NestedVRFRequest.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::NestedRoleRequest**](NestedRoleRequest.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**mark_utilized** | Option<**bool**> | Treat as 100% utilized | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


