# IoScansCheckAutoTargets200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**missed_targets** | Option<**Vec<String>**> | The list of targets that did not match a route in any scanner group. | [optional]
**total_missed_targets** | Option<**i32**> | The total count of missed targets, before being truncated by the optional `limit` parameter. | [optional]
**matched_resource_uuids** | Option<**Vec<String>**> | A list of UUIDs for scanner groups where configured scan routes matched at least one of the specified targets. | [optional]
**total_matched_resource_uuids** | Option<**i32**> | The count of matched resource UUIDs, before being truncated by the optional `matched_resource_limit` parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


