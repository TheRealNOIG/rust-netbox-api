# LocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**slug** | **String** |  | 
**site** | [**crate::models::NestedSiteRequest**](NestedSiteRequest.md) |  | 
**parent** | Option<[**crate::models::NestedLocationRequest**](NestedLocationRequest.md)> |  | [optional]
**status** | Option<**String**> | * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `retired` - Retired | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


