# EditorDetails200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_was** | Option<**bool**> | Indicates whether the scan or user-defined template (policy) can be used in Tenable.io Web Application Scanning. | [optional]
**user_permissions** | Option<**i32**> | The scan or policy permissions that the requesting user has for the specified scan or user-defined template (policy). For more information, see [Permissions](doc:permissions). | [optional]
**owner** | Option<**String**> | The username of the owner of the scan or user-defined template (policy). | [optional]
**title** | Option<**String**> | For scans, the standard text, `Custom Scan`. For user-defined templates (policies), the name of the Tenable-provided template used to create the user-defined template.  | [optional]
**is_agent** | Option<**bool**> | Indicates whether the scan or user-defined template (policy) can be used for agent scans. | [optional]
**uuid** | Option<**String**> | The UUID of the scan or user-defined template (policy). | [optional]
**filter_attributes** | Option<[**Vec<crate::models::EditorDetails200ResponseFilterAttributesInner>**](editor_details_200_response_filter_attributes_inner.md)> |  | [optional]
**settings** | Option<[**crate::models::EditorDetails200ResponseSettings**](editor_details_200_response_settings.md)> |  | [optional]
**credentials** | Option<[**serde_json::Value**](.md)> | Credentials that grant the scanner access to the target system without requiring an agent. Credentialed scans can perform a wider variety of checks than non-credentialed scans, which can result in more accurate scan results. This facilitates scanning of a very large network to determine local exposures or compliance violations. You can configure credentials for Cloud Services, Database, Host, Miscellaneous, Mobile Device Management, and Plaintext Authentication. | [optional]
**compliance** | Option<[**serde_json::Value**](.md)> | Plugins options enables you to select security checks by Plugin Family or individual plugins checks. | [optional]
**plugins** | Option<[**serde_json::Value**](.md)> | The settings for compliance audit checks. | [optional]
**name** | Option<**String**> | For scans, the standard text, `custom`. For user-defined templates (policies), the system name for the Tenable-provided template used to create the scan or user-defined template. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


