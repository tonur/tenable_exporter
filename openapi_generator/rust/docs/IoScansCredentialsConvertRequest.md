# IoScansCredentialsConvertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the managed credentials. This name must be unique within your Tenable.io instance. | 
**settings** | [**crate::models::IoScansCredentialsConvertRequestSettings**](io_scans_credentials_convert_request_settings.md) |  | 
**r#type** | **String** | The system name that uniquely identifies the credentials type, for example, `Windows`. | 
**category** | **String** | The system name that uniquely identifies the credentials category, for example `Host`. | 
**ad_hoc** | **bool** | A value specifying whether the credentials are scan-specific (`true`) or managed (`false`). You must use `false` for this request body attribute to convert scan-specific to managed credentials. | 
**permissions** | Option<[**Vec<crate::models::IoScansCredentialsConvertRequestPermissionsInner>**](io_scans_credentials_convert_request_permissions_inner.md)> | A list of user permissions for the managed credentials. If a request message omits this parameter, Tenable.io automatically creates a `permissions` object for the user account that submits the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


