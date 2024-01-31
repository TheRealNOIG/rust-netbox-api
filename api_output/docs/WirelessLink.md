# WirelessLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**interface_a** | [**crate::models::NestedInterface**](NestedInterface.md) |  | 
**interface_b** | [**crate::models::NestedInterface**](NestedInterface.md) |  | 
**ssid** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::CableStatus**](Cable_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**auth_type** | Option<[**crate::models::WirelessLanAuthType**](WirelessLAN_auth_type.md)> |  | [optional]
**auth_cipher** | Option<[**crate::models::WirelessLanAuthCipher**](WirelessLAN_auth_cipher.md)> |  | [optional]
**auth_psk** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


