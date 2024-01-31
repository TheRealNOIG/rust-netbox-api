# VmInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**virtual_machine** | [**crate::models::NestedVirtualMachineRequest**](NestedVirtualMachineRequest.md) |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedVmInterfaceRequest**](NestedVMInterfaceRequest.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedVmInterfaceRequest**](NestedVMInterfaceRequest.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> | * `access` - Access * `tagged` - Tagged * `tagged-all` - Tagged (All) | [optional]
**untagged_vlan** | Option<[**crate::models::NestedVlanRequest**](NestedVLANRequest.md)> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrfRequest**](NestedVRFRequest.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


