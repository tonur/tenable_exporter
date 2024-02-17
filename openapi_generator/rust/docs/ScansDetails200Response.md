# ScansDetails200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**info** | Option<[**crate::models::ScansDetails200ResponseInfo**](scans_details_200_response_info.md)> |  | [optional]
**comphosts** | Option<[**Vec<crate::models::ScansDetails200ResponseComphostsInner>**](scans_details_200_response_comphosts_inner.md)> | A list of the hosts targeted by the scan for the specified run. If the scan results are older than 35 days (that is, if the `info.archived` attribute for the scan is `true`), this array does not appear in the response message. | [optional]
**hosts** | Option<[**Vec<crate::models::ScansDetails200ResponseComphostsInner>**](scans_details_200_response_comphosts_inner.md)> | A list of the hosts targeted by the scan for the specified run. If the scan results are older than 35 days (that is, if the `info.archived` attribute for the scan is `true`), this array does not appear in the response message. | [optional]
**notes** | Option<[**Vec<crate::models::ScansDetails200ResponseNotesInner>**](scans_details_200_response_notes_inner.md)> |  | [optional]
**remediations** | Option<[**serde_json::Value**](.md)> |  | [optional]
**vulnerabilities** | Option<[**Vec<crate::models::ScansDetails200ResponseVulnerabilitiesInner>**](scans_details_200_response_vulnerabilities_inner.md)> | A list of vulnerabilities that the scan identified on the target hosts. If the scan results are older than 35 days (that is, if the `info.archived` attribute for the scan is `true`), this array does not appear in the response message. | [optional]
**filters** | Option<[**Vec<crate::models::ScansDetails200ResponseFiltersInner>**](scans_details_200_response_filters_inner.md)> | A list of filters. If the scan results are older than 35 days (that is, if the `info.archived` attribute for the scan is `true`), this array does not appear in the response message. | [optional]
**history** | Option<[**Vec<crate::models::ScansDetails200ResponseHistoryInner>**](scans_details_200_response_history_inner.md)> | A list of details about each time the scan has run. | [optional]
**compliance** | Option<[**Vec<crate::models::ScansDetails200ResponseVulnerabilitiesInner>**](scans_details_200_response_vulnerabilities_inner.md)> | A list of compliance checks performed during the run of the scan. If the scan results are older than 35 days (that is, if the `info.archived` attribute for the scan is `true`), this array does not appear in the response message. | [optional]
**progress** | Option<**i32**> | The progress of the scan ranging from 0 to 100. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


