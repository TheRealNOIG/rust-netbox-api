# ClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | [**crate::models::NestedClusterTypeRequest**](NestedClusterTypeRequest.md) |  | 
**group** | Option<[**crate::models::NestedClusterGroupRequest**](NestedClusterGroupRequest.md)> |  | [optional]
**status** | Option<**String**> | * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `offline` - Offline | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSiteRequest**](NestedSiteRequest.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


