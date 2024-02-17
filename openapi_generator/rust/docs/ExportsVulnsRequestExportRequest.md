# ExportsVulnsRequestExportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_assets** | **i32** | Specifies the number of assets used to chunk the vulnerabilities. The vulnerabilities export is split up by number of asset IDs in a chunk. The exported data of a chunk is the sum of all the vulnerabilities for each asset in that chunk. The range for number of assets in a chunk is a minimum of 50 (the default size) to a maximum of 5,000. If you specify a value outside this range, the system uses the upper or lower-bound value. | 
**include_unlicensed** | Option<**bool**> | Specifies whether or not to include unlicensed assets. The default is `false` when no parameter is specified. | [optional]
**filters** | Option<[**crate::models::ExportsVulnsRequestExportRequestFilters**](exports_vulns_request_export_request_filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


