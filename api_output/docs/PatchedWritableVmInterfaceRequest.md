# PatchedWritableVmInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**virtual_machine** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**bridge** | Option<**i32**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> | IEEE 802.1Q tagging strategy  * `access` - Access * `tagged` - Tagged * `tagged-all` - Tagged (All) | [optional]
**untagged_vlan** | Option<**i32**> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**vrf** | Option<**i32**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


