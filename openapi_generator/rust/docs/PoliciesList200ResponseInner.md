# PoliciesList200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of the policy. | [optional]
**template_uuid** | Option<**String**> | The UUID for the Tenable-provided template used to create the policy. | [optional]
**name** | Option<**String**> | The name of the policy. | [optional]
**description** | Option<**String**> | The description of the policy. | [optional]
**owner_id** | Option<**String**> | The unique ID of the owner of the policy. | [optional]
**owner** | Option<**String**> | The username for the owner of the policy. | [optional]
**shared** | Option<**i32**> | The shared status of the policy (`1` if shared with users other than owner, `0` if not shared). | [optional]
**user_permissions** | Option<**i32**> | The sharing permissions for the policy. | [optional]
**creation_date** | Option<**i32**> | The creation date of the policy in Unix time format. | [optional]
**last_modification_date** | Option<**i32**> | The last modification date for the policy in Unix time format. | [optional]
**visibility** | Option<**i32**> | The visibility of the target (`private` or `shared`). | [optional]
**no_target** | Option<**bool**> | If `true`, the policy configuration does not include targets. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


