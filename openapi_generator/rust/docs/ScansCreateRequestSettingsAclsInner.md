# ScansCreateRequestSettingsAclsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**permissions** | Option<**i32**> | The scan permission. For more information, see [Permissions](doc:permissions). | [optional]
**owner** | Option<**i32**> | A value that indicates whether the user or user group specified in the object owns the scan. Possible values include: `null` (system-owned permissions), `0` (the user is not the owner of the scan), `1` (the user is the owner of the scan). | [optional]
**display_name** | Option<**String**> | The name of the user or group granted the specified permissions, as it appears in the Tenable.io user interface. | [optional]
**name** | Option<**String**> | The name of the user or group granted the specified permissions. | [optional]
**id** | Option<**i32**> | A number representing the order in which the user or user groups display in the Permissions tab in the Tenable.io user interface. | [optional]
**r#type** | Option<**String**> | The type of scan permissions: `default` (default permissions for the scan), `user` (permissions for an individual user), or `group` (permissions for a user group). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


