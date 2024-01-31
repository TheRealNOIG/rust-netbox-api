# WritableCustomFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_types** | **Vec<String>** |  | 
**r#type** | Option<**String**> | The type of data this custom field holds  * `text` - Text * `longtext` - Text (long) * `integer` - Integer * `decimal` - Decimal * `boolean` - Boolean (true/false) * `date` - Date * `datetime` - Date & time * `url` - URL * `json` - JSON * `select` - Selection * `multiselect` - Multiple selection * `object` - Object * `multiobject` - Multiple objects | [optional]
**object_type** | Option<**String**> |  | [optional]
**name** | **String** | Internal field name | 
**label** | Option<**String**> | Name of the field as displayed to users (if not provided, 'the field's name will be used) | [optional]
**group_name** | Option<**String**> | Custom fields within the same group will be displayed together | [optional]
**description** | Option<**String**> |  | [optional]
**required** | Option<**bool**> | If true, this field is required when creating new objects or editing an existing object. | [optional]
**search_weight** | Option<**i32**> | Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored. | [optional]
**filter_logic** | Option<**String**> | Loose matches any instance of a given string; exact matches the entire field.  * `disabled` - Disabled * `loose` - Loose * `exact` - Exact | [optional]
**ui_visible** | Option<**String**> | Specifies whether the custom field is displayed in the UI  * `always` - Always * `if-set` - If set * `hidden` - Hidden | [optional]
**ui_editable** | Option<**String**> | Specifies whether the custom field value can be edited in the UI  * `yes` - Yes * `no` - No * `hidden` - Hidden | [optional]
**is_cloneable** | Option<**bool**> | Replicate this value when cloning objects | [optional]
**default** | Option<[**serde_json::Value**](.md)> | Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. \"Foo\"). | [optional]
**weight** | Option<**i32**> | Fields with higher weights appear lower in a form. | [optional]
**validation_minimum** | Option<**i64**> | Minimum allowed value (for numeric fields) | [optional]
**validation_maximum** | Option<**i64**> | Maximum allowed value (for numeric fields) | [optional]
**validation_regex** | Option<**String**> | Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters. | [optional]
**choice_set** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


