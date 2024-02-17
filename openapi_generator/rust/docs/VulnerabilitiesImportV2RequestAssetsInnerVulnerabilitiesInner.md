# VulnerabilitiesImportV2RequestAssetsInnerVulnerabilitiesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenable_plugin_id** | Option<**String**> | The ID of the Nessus plugin that identified the vulnerability. This parameter is required if the vulnerability object does not specify a cve value. | [optional]
**cve** | Option<**String**> | The Common Vulnerability and Exposure (CVE) ID for the vulnerability. This parameter is required if the vulnerability object does not specify a `tenable_plugin_id` value. | [optional]
**port** | Option<**i32**> | The port the scanner used to communicate with the asset. | [optional]
**protocol** | Option<**String**> | The protocol the scanner used to communicate with the asset. | [optional]
**authenticated** | Option<**bool**> | A value specifying whether the scan that identified the vulnerability was an authenticated (credentialed) scan. | [optional]
**last_found** | Option<**i32**> | The date (in Unix time) when a scan last identified the vulnerability on the asset. | [optional]
**output** | Option<**String**> | (Required) The text output of the scanner, up to 2,000 maximum characters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


