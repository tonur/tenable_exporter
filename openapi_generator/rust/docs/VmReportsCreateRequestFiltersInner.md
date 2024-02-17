# VmReportsCreateRequestFiltersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**property** | **String** | The property to filter the results by.   You can use the [GET /filters/reports/export](ref:vm-filters-reports-list) endpoint to retrieve a list of valid properties that can be used for filtering. For more information, see [Report Export Filters](doc:vm-report-export-filters). | 
**operator** | **String** | The comparison operator to apply to the filter. For example, `eq`, `neq`, `gt`, etc.   You can use the [GET /filters/reports/export](ref:vm-filters-reports-list) endpoint to retrieve a list of supported comparison operators for your chosen filter. For more information, see [Report Export Filters](doc:vm-report-export-filters). | 
**value** | [**crate::models::VmReportsCreateRequestFiltersInnerValue**](vm_reports_create_request_filters_inner_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


