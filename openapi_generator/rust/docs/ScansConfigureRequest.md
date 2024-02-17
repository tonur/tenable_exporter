# ScansConfigureRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **String** | The UUID for the Tenable-provided scan template to use. Use the [GET /editor/scan/templates](ref:editor-list-templates) endpoint to find the template UUID. | 
**settings** | [**crate::models::ScansConfigureRequestSettings**](scans_configure_request_settings.md) |  | 
**credentials** | Option<[**crate::models::ScansConfigureRequestCredentials**](scans_configure_request_credentials.md)> |  | [optional]
**plugins** | Option<[**crate::models::ScansCreateRequestPlugins**](scans_create_request_plugins.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


