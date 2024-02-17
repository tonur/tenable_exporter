# WorkbenchesAssetsVulnerabilities200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID of the asset. | [optional]
**severities** | Option<[**Vec<crate::models::WorkbenchesAssetsVulnerabilities200ResponseInnerSeveritiesInner>**](workbenches_assets_vulnerabilities_200_response_inner_severities_inner.md)> | A count of vulnerabilities by severity. | [optional]
**total** | Option<**i32**> | The total number of vulnerabilities detected on the asset. | [optional]
**fqdn** | Option<**Vec<String>**> | A list of fully-qualified domain names (FQDNs) for the asset. | [optional]
**ipv4** | Option<**Vec<String>**> | A list of ipv4 addresses for the asset. | [optional]
**ipv6** | Option<**Vec<String>**> | A list of ipv6 addresses for the asset. | [optional]
**last_seen** | Option<**String**> | The ISO timestamp of the scan that most recently detected the asset. | [optional]
**netbios_name** | Option<**Vec<String>**> | The NetBIOS name for the asset. | [optional]
**agent_name** | Option<**Vec<String>**> | The names of any Nessus agents that scanned and identified the asset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


