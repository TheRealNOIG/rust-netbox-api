# PatchedWritableCircuitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | Option<**String**> | Unique circuit ID | [optional]
**provider** | Option<**i32**> |  | [optional]
**provider_account** | Option<**i32**> |  | [optional]
**r#type** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned | [optional]
**tenant** | Option<**i32**> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> | Committed rate | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


