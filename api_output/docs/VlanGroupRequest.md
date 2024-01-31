# VlanGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**slug** | **String** |  | 
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**min_vid** | Option<**i32**> | Lowest permissible ID of a child VLAN | [optional]
**max_vid** | Option<**i32**> | Highest permissible ID of a child VLAN | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


