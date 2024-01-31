# VlanGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**scope** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**min_vid** | Option<**i32**> | Lowest permissible ID of a child VLAN | [optional]
**max_vid** | Option<**i32**> | Highest permissible ID of a child VLAN | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**vlan_count** | **i32** |  | [readonly]
**utilization** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


