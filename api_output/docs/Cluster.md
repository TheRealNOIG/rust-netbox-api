# Cluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**r#type** | [**crate::models::NestedClusterType**](NestedClusterType.md) |  | 
**group** | Option<[**crate::models::NestedClusterGroup**](NestedClusterGroup.md)> |  | [optional]
**status** | Option<[**crate::models::ClusterStatus**](Cluster_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i32** |  | [readonly]
**virtualmachine_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


