# RackReservationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rack** | [**crate::models::NestedRackRequest**](NestedRackRequest.md) |  | 
**units** | **Vec<i32>** |  | 
**user** | [**crate::models::NestedUserRequest**](NestedUserRequest.md) |  | 
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**description** | **String** |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


