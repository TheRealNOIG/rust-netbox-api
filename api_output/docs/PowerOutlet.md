# PowerOutlet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**module** | Option<[**crate::models::ComponentNestedModule**](ComponentNestedModule.md)> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<[**crate::models::PowerOutletType**](PowerOutlet_type.md)> |  | [optional]
**power_port** | Option<[**crate::models::NestedPowerPort**](NestedPowerPort.md)> |  | [optional]
**feed_leg** | Option<[**crate::models::PowerOutletFeedLeg**](PowerOutlet_feed_leg.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [readonly]
**cable_end** | **String** |  | [readonly]
**link_peers** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**link_peers_type** | **String** | Return the type of the peer link terminations, or None. | [readonly]
**connected_endpoints** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**connected_endpoints_type** | **String** |  | [readonly]
**connected_endpoints_reachable** | **bool** |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_occupied** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


