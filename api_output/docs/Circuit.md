# Circuit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**cid** | **String** | Unique circuit ID | 
**provider** | [**crate::models::NestedProvider**](NestedProvider.md) |  | 
**provider_account** | Option<[**crate::models::NestedProviderAccount**](NestedProviderAccount.md)> |  | [optional]
**r#type** | [**crate::models::NestedCircuitType**](NestedCircuitType.md) |  | 
**status** | Option<[**crate::models::CircuitStatus**](Circuit_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> | Committed rate | [optional]
**description** | Option<**String**> |  | [optional]
**termination_a** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [readonly]
**termination_z** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [readonly]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


