# IpSecProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**mode** | [**crate::models::IpSecProfileMode**](IPSecProfile_mode.md) |  | 
**ike_policy** | [**crate::models::NestedIkePolicy**](NestedIKEPolicy.md) |  | 
**ipsec_policy** | [**crate::models::NestedIpSecPolicy**](NestedIPSecPolicy.md) |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


