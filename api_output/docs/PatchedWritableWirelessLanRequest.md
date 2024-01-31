# PatchedWritableWirelessLanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ssid** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**group** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `reserved` - Reserved * `disabled` - Disabled * `deprecated` - Deprecated | [optional]
**vlan** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**auth_type** | Option<**String**> | * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise | [optional]
**auth_cipher** | Option<**String**> | * `auto` - Auto * `tkip` - TKIP * `aes` - AES | [optional]
**auth_psk** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


