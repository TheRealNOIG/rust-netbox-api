# PatchedWritableDeviceWithConfigContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**device_type** | Option<**i32**> |  | [optional]
**role** | Option<**i32**> | The function this device serves | [optional]
**tenant** | Option<**i32**> |  | [optional]
**platform** | Option<**i32**> |  | [optional]
**serial** | Option<**String**> | Chassis serial number, assigned by the manufacturer | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**site** | Option<**i32**> |  | [optional]
**location** | Option<**i32**> |  | [optional]
**rack** | Option<**i32**> |  | [optional]
**position** | Option<**f64**> |  | [optional]
**face** | Option<**String**> | * `front` - Front * `rear` - Rear | [optional]
**latitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**longitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `inventory` - Inventory * `decommissioning` - Decommissioning | [optional]
**airflow** | Option<**String**> | * `front-to-rear` - Front to rear * `rear-to-front` - Rear to front * `left-to-right` - Left to right * `right-to-left` - Right to left * `side-to-rear` - Side to rear * `passive` - Passive * `mixed` - Mixed | [optional]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**oob_ip** | Option<**i32**> |  | [optional]
**cluster** | Option<**i32**> |  | [optional]
**virtual_chassis** | Option<**i32**> |  | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> | Virtual chassis master election priority | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**config_template** | Option<**i32**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


