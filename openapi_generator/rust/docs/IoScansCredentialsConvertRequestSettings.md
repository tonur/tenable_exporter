# IoScansCredentialsConvertRequestSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | Option<**String**> | The Windows domain to which the username belongs. | [optional]
**username** | Option<**String**> | The username on the target system. | [optional]
**auth_method** | Option<**String**> | The name for the authentication method. This value corresponds to the credentials[].types[].configuration[].options[].id attribute in the response message for the [GET /credentials/types](ref:credentials-list-credential-types) endpoint. | [optional]
**password** | Option<**String**> | The user password on the target system. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


