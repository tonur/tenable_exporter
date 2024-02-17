# \ReportsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vm_reports_create**](ReportsApi.md#vm_reports_create) | **POST** /reports/export | Create report
[**vm_reports_download**](ReportsApi.md#vm_reports_download) | **GET** /reports/export/{report_uuid}/download | Download report
[**vm_reports_status**](ReportsApi.md#vm_reports_status) | **GET** /reports/export/{report_uuid}/status | Get report status



## vm_reports_create

> crate::models::VmReportsCreate200Response vm_reports_create(vm_reports_create_request)
Create report

Creates a report in PDF format based on the specified template and filters.  **Note:** Tenable Vulnerability Management limits the number of findings that can be included in a single report to 10,000. If you have more than 10,000 findings, Tenable recommends that you narrow the findings included in the report with a filter or generate multiple reports. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vm_reports_create_request** | [**VmReportsCreateRequest**](VmReportsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::VmReportsCreate200Response**](vm_reports_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vm_reports_download

> vm_reports_download(report_uuid)
Download report

Downloads the specified PDF report. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_uuid** | **uuid::Uuid** | The UUID of the report to download. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vm_reports_status

> crate::models::VmReportsStatus200Response vm_reports_status(report_uuid)
Get report status

Returns the status of the specified report export request. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_uuid** | **uuid::Uuid** | The UUID of the report to check the status for. | [required] |

### Return type

[**crate::models::VmReportsStatus200Response**](vm_reports_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

