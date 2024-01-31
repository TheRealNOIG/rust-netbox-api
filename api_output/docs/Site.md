# Site

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** | Full name of the site | 
**slug** | **String** |  | 
**status** | Option<[**crate::models::LocationStatus**](Location_status.md)> |  | [optional]
**region** | Option<[**crate::models::NestedRegion**](NestedRegion.md)> |  | [optional]
**group** | Option<[**crate::models::NestedSiteGroup**](NestedSiteGroup.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**facility** | Option<**String**> | Local facility ID or description | [optional]
**time_zone** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**physical_address** | Option<**String**> | Physical location of the building | [optional]
**shipping_address** | Option<**String**> | If different from the physical address | [optional]
**latitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**longitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**comments** | Option<**String**> |  | [optional]
**asns** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**circuit_count** | **i32** |  | [readonly]
**device_count** | **i32** |  | [readonly]
**prefix_count** | **i32** |  | [readonly]
**rack_count** | **i32** |  | [readonly]
**virtualmachine_count** | **i32** |  | [readonly]
**vlan_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


