# WritableTunnelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**status** | Option<**String**> | * `planned` - Planned * `active` - Active * `disabled` - Disabled | [optional]
**group** | Option<**i32**> |  | [optional]
**encapsulation** | **String** | * `ipsec-transport` - IPsec - Transport * `ipsec-tunnel` - IPsec - Tunnel * `ip-ip` - IP-in-IP * `gre` - GRE | 
**ipsec_profile** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**tunnel_id** | Option<**i64**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


