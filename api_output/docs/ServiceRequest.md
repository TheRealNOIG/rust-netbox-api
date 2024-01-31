# ServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | Option<[**crate::models::NestedDeviceRequest**](NestedDeviceRequest.md)> |  | [optional]
**virtual_machine** | Option<[**crate::models::NestedVirtualMachineRequest**](NestedVirtualMachineRequest.md)> |  | [optional]
**name** | **String** |  | 
**ports** | **Vec<i32>** |  | 
**protocol** | Option<**String**> | * `tcp` - TCP * `udp` - UDP * `sctp` - SCTP | [optional]
**ipaddresses** | Option<**Vec<i32>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


