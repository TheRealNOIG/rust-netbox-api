# WritableSiteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Full name of the site | 
**slug** | **String** |  | 
**status** | Option<**String**> | * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `retired` - Retired | [optional]
**region** | Option<**i32**> |  | [optional]
**group** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**facility** | Option<**String**> | Local facility ID or description | [optional]
**time_zone** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**physical_address** | Option<**String**> | Physical location of the building | [optional]
**shipping_address** | Option<**String**> | If different from the physical address | [optional]
**latitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**longitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**comments** | Option<**String**> |  | [optional]
**asns** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


