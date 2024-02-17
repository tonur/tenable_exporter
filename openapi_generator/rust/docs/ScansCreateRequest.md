# ScansCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **String** | The UUID for the Tenable-provided scan template to use. Use the [GET /editor/scan/templates](ref:editor-list-templates) endpoint to find the template UUID.  **Caution:** The defaults listed for a template via the [GET /editor/{type}/templates/{template_uuid}](ref:editor-template-details) endpoint apply to the user interface only. When you create a scan via the API you must include the settings in the request even if the setting is listed as a default in the editor. For example, even if `host_tagging` is set to `yes` by default in the editor, you still need to include `\"host_tagging\": \"yes\"` in the `settings` object for the scan. | 
**settings** | [**crate::models::ScansCreateRequestSettings**](scans_create_request_settings.md) |  | 
**credentials** | Option<[**crate::models::ScansCreateRequestCredentials**](scans_create_request_credentials.md)> |  | [optional]
**plugins** | Option<[**crate::models::ScansCreateRequestPlugins**](scans_create_request_plugins.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


