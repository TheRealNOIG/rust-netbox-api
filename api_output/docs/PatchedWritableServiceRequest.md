# PatchedWritableServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | Option<**i32**> |  | [optional]
**virtual_machine** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**ports** | Option<**Vec<i32>**> |  | [optional]
**protocol** | Option<**String**> | * `tcp` - TCP * `udp` - UDP * `sctp` - SCTP | [optional]
**ipaddresses** | Option<**Vec<i32>**> | The specific IP addresses (if any) to which this service is bound | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


