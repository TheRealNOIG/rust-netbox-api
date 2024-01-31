# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**virtual_machine** | Option<[**crate::models::NestedVirtualMachine**](NestedVirtualMachine.md)> |  | [optional]
**name** | **String** |  | 
**ports** | **Vec<i32>** |  | 
**protocol** | Option<[**crate::models::ServiceProtocol**](Service_protocol.md)> |  | [optional]
**ipaddresses** | Option<**Vec<i32>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


