# Rack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**facility_id** | Option<**String**> |  | [optional]
**site** | [**crate::models::NestedSite**](NestedSite.md) |  | 
**location** | Option<[**crate::models::NestedLocation**](NestedLocation.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**status** | Option<[**crate::models::RackStatus**](Rack_status.md)> |  | [optional]
**role** | Option<[**crate::models::NestedRackRole**](NestedRackRole.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**r#type** | Option<[**crate::models::RackType**](Rack_type.md)> |  | [optional]
**width** | Option<[**crate::models::RackWidth**](Rack_width.md)> |  | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**starting_unit** | Option<**i32**> | Starting unit for rack | [optional]
**weight** | Option<**f64**> |  | [optional]
**max_weight** | Option<**i32**> | Maximum load capacity for the rack | [optional]
**weight_unit** | Option<[**crate::models::DeviceTypeWeightUnit**](DeviceType_weight_unit.md)> |  | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**outer_unit** | Option<[**crate::models::RackOuterUnit**](Rack_outer_unit.md)> |  | [optional]
**mounting_depth** | Option<**i32**> | Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails. | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i32** |  | [readonly]
**powerfeed_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


