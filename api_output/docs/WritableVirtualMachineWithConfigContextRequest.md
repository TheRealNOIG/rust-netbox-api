# WritableVirtualMachineWithConfigContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning | [optional]
**site** | Option<**i32**> |  | [optional]
**cluster** | Option<**i32**> |  | [optional]
**device** | Option<**i32**> |  | [optional]
**role** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**platform** | Option<**i32**> |  | [optional]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**vcpus** | Option<**f64**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


