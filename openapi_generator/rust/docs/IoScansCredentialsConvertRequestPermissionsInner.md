# IoScansCredentialsConvertRequestPermissionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grantee_uuid** | Option<**String**> | The UUID of the user or user group granted permissions for managed credentials.   This parameter is required when assigning CAN USE [32] or CAN EDIT [64] permissions for managed credentials. | [optional]
**r#type** | Option<**String**> | A value specifying whether the grantee is a user (`user`) or a user group (`group`).    This parameter is required when assigning CAN USE [32] or CAN EDIT [64] permissions for  managed credentials. | [optional]
**permissions** | Option<**i32**> | A value specifying the permissions granted to the user or user group for the managed credentials. Possible values are:  - 32—The user can view credentials information and use the credentials in scans. Corresponds to the **Can Use** permission in the user interface.  - 64—The user can view and edit credential settings, delete the credentials, and use the credentials in scans. Corresponds to the **Can Edit** permission in the user interface.    This parameter is required when assigning CAN USE [32] or CAN EDIT [64] permissions for managed credentials. | [optional]
**name** | Option<**String**> | The name of the user or user group that you want to grant permissions for the managed credentials.    This parameter is optional when assigning CAN USE [32] or CAN EDIT [64] permissions for the managed credentials. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


