# CircuitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **String** | Unique circuit ID | 
**provider** | [**crate::models::NestedProviderRequest**](NestedProviderRequest.md) |  | 
**provider_account** | Option<[**crate::models::NestedProviderAccountRequest**](NestedProviderAccountRequest.md)> |  | [optional]
**r#type** | [**crate::models::NestedCircuitTypeRequest**](NestedCircuitTypeRequest.md) |  | 
**status** | Option<**String**> | * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned | [optional]
**tenant** | Option<[**crate::models::NestedTenantRequest**](NestedTenantRequest.md)> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> | Committed rate | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


