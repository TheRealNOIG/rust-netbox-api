# ConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**weight** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**regions** | Option<**Vec<i32>**> |  | [optional]
**site_groups** | Option<**Vec<i32>**> |  | [optional]
**sites** | Option<**Vec<i32>**> |  | [optional]
**locations** | Option<**Vec<i32>**> |  | [optional]
**device_types** | Option<**Vec<i32>**> |  | [optional]
**roles** | Option<**Vec<i32>**> |  | [optional]
**platforms** | Option<**Vec<i32>**> |  | [optional]
**cluster_types** | Option<**Vec<i32>**> |  | [optional]
**cluster_groups** | Option<**Vec<i32>**> |  | [optional]
**clusters** | Option<**Vec<i32>**> |  | [optional]
**tenant_groups** | Option<**Vec<i32>**> |  | [optional]
**tenants** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**data_source** | Option<[**crate::models::NestedDataSource**](NestedDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | [**crate::models::NestedDataFile**](NestedDataFile.md) |  | [readonly]
**data_synced** | Option<**String**> |  | [readonly]
**data** | Option<[**serde_json::Value**](.md)> |  | 
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


