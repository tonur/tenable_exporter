# ScansExportRequestRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | **String** | The file format to use (`Nessus`, `HTML`, `PDF`, or `CSV`).  You can export scans in `HTML` and `PDF` format for up to 60 days. For scans that are older than 60 days, only the `Nessus` and `CSV` formats are supported. | 
**chapters** | Option<**String**> | The chapters to include in the export. This parameter accepts a semi-colon delimited string comprised of some combination of the following options: vuln_hosts_summary, vuln_by_host, compliance_exec, remediations, vuln_by_plugin, compliance).  **Note:** This parameter is required if the file format is `PDF` or `HTML`. | [optional]
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. If you specify the name of the filter, you must specify the operator as the `filter.0.quality` parameter and the value as the `filter.0.value` parameter. To use multiple filters, increment the `<INDEX>` portion of `filter.<INDEX>.filter`, for example, `filter.1.filter`. For more information about using this parameter, see [Scan Export Filters](doc:scan-export-filters-tio).  **Note:** Filters are not supported when exporting scan results that are older than 60 days. | [optional]
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. To use multiple filters, increment the `<INDEX>` portion of `filter.<INDEX>.quality`, for example, `filter.1.quality`. For more information about using this parameter, see [Scan Export Filters](doc:scan-export-filters-tio). | [optional]
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the `control` attribute of the objects returned by the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. To use multiple filters, increment the `<INDEX>` portion of `filter.<INDEX>.value`, for example, `filter.1.value`. For more information about using this parameter, see [Scan Export Filters](doc:scan-export-filters-tio). | [optional]
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about using this parameter, see [Scan Export Filters](doc:scan-export-filters-tio). | [optional]
**asset_id** | Option<**String**> | The ID of the asset scanned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


