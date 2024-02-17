# VmReportsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A name for the report.  If this parameter is omitted, Tenable Vulnerability Management uses the default name `Vulnerabilities_Export_Report` with a timestamp in ISO 8601 format appended to the end to create a unique name. For example, `Vulnerabilities_Export_Report_2023-11-30T00:23:13.199227748Z`. | [optional]
**template_name** | **String** | The type of template to use for the report. The following templates are available:  - `host_vulns_summary`—An executive summary report that provides operations teams a snapshot of risk based on vulnerable assets.  - `host_vulns_by_plugins`—A report that provides a summary of the plugins that detected vulnerabilities on affected assets. Plugins are sorted by severity and the assets are sorted by the Asset Criticality Rating (ACR).  - `host_vulns_by_assets`—A summary of the most vulnerable assets. | 
**filters** | Option<[**Vec<crate::models::VmReportsCreateRequestFiltersInner>**](vm_reports_create_request_filters_inner.md)> | A set of filters to apply to the report. Filters can be used to narrow the vulnerabilities or assets included in the report.   You can use the [GET /filters/reports/export](ref:vm-filters-reports-list) endpoint to retrieve the list of valid filters, their supported comparison operators, data types, and allowed values. For more information, see [Report Export Filters](doc:vm-report-export-filters). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


