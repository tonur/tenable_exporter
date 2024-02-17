# ScansCreateRequestCredentialsAddHost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**windows** | Option<[**Vec<crate::models::ScansCreateRequestCredentialsAddHostWindowsInner>**](scans_create_request_credentials_add_Host_Windows_inner.md)> | The name of this parameter corresponds to the display name that uniquely identifies the credentials type (in this case, `Windows` for a Windows credential). This value corresponds to the following response message attributes:  - `credentials[].data[].types[].name` in the [GET /editor/type/templates/{template_uuid}](/reference#editor-template-details) response message  - `credentials[].types[].id` in the [GET /credentials/types](/reference#credentials-list-credential-types) response message` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


