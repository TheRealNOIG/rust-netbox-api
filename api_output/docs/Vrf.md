# Vrf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**rd** | Option<**String**> | Unique route distinguisher (as defined in RFC 4364) | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**enforce_unique** | Option<**bool**> | Prevent duplicate prefixes/IP addresses within this VRF | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**import_targets** | Option<**Vec<i32>**> |  | [optional]
**export_targets** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**ipaddress_count** | **i32** |  | [readonly]
**prefix_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


