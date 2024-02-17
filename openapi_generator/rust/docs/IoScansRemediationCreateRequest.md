# IoScansRemediationCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **String** | The UUID for the Tenable-provided remediation scan template to use. Use the [GET /editor/remediation/templates](ref:editor-list-templates) endpoint to find remediation scan template UUIDs. | 
**settings** | [**crate::models::IoScansRemediationCreateRequestSettings**](io_scans_remediation_create_request_settings.md) |  | 
**credentials** | Option<[**crate::models::ScansCreateRequestCredentials**](scans_create_request_credentials.md)> |  | [optional]
**enabled_plugins** | Option<**Vec<i32>**> | A comma-delimited list of plugins IDs to add to a remediation scan. You can use the [GET /plugins/families](ref:io-plugins-families-list) and [GET /plugins/families/{id}](ref:io-plugins-family-details) endpoints to get a list of plugin families and plugins to add to the scan.  **Note:** This parameter is only valid for remediation scans. For more information on remediation scans and examples, see [Manage Remediation Scans](doc:io-manage-remediation-scans). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


