# ScansConfigureRequestCredentials

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add** | Option<[**crate::models::ScansConfigureRequestCredentialsAdd**](scans_configure_request_credentials_add.md)> |  | [optional]
**edit** | Option<[**serde_json::Value**](.md)> | A scan-specific credentials object you want to modify. The parameters of the object vary based on credential category, credential type, and type-specific settings. For more information, see [Update a Scan](doc:update-scan-tio).   **Note:** This parameter is not supported for use with managed credentials. For more information about editing managed credentials, see [Edit Managed Credentials](doc:edit-managed-credentials-tio). | [optional]
**delete** | Option<**Vec<String>**> | A list of identifiers for the credentials you want to remove from the scan. For more information, see [Remove Credentials from a Scan](doc:remove-credentials-from-scan-tio). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


