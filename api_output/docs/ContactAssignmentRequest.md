# ContactAssignmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_type** | **String** |  | 
**object_id** | **i64** |  | 
**contact** | [**crate::models::NestedContactRequest**](NestedContactRequest.md) |  | 
**role** | Option<[**crate::models::NestedContactRoleRequest**](NestedContactRoleRequest.md)> |  | [optional]
**priority** | Option<**String**> | * `primary` - Primary * `secondary` - Secondary * `tertiary` - Tertiary * `inactive` - Inactive | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


