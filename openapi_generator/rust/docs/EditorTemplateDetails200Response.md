# EditorTemplateDetails200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_was** | Option<**bool**> | If `true`, the template can be used for Web Application Scanning only. For Vulnerability Management templates, this value is always `null`. | [optional]
**title** | Option<**String**> | The long name of the template. | [optional]
**name** | Option<**String**> | The short name of the template. | [optional]
**is_agent** | Option<**bool**> | If `true`, the template is for agent scans. | [optional]
**filter_attributes** | Option<[**Vec<crate::models::EditorDetails200ResponseFilterAttributesInner>**](editor_details_200_response_filter_attributes_inner.md)> |  | [optional]
**settings** | Option<[**crate::models::EditorDetails200ResponseSettings**](editor_details_200_response_settings.md)> |  | [optional]
**credentials** | Option<[**serde_json::Value**](.md)> | Credentials that grant the scanner access to the target system without requiring an agent. Credentialed scans can perform a wider variety of checks than non-credentialed scans, which can result in more accurate scan results. This facilitates scanning of a very large network to determine local exposures or compliance violations. You can configure credentials for Cloud Services, Database, Host, Miscellaneous, Mobile Device Management, and Plaintext Authentication. | [optional]
**compliance** | Option<[**serde_json::Value**](.md)> | Plugins options enables you to select security checks by Plugin Family or individual plugins checks. | [optional]
**plugins** | Option<[**serde_json::Value**](.md)> | The settings for compliance audit checks. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


