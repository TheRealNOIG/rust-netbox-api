# DeviceBayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | [**crate::models::NestedDeviceRequest**](NestedDeviceRequest.md) |  | 
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**installed_device** | Option<[**crate::models::NestedDeviceRequest**](NestedDeviceRequest.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


