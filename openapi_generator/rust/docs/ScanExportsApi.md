# \ScanExportsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scans_export_download**](ScanExportsApi.md#scans_export_download) | **GET** /scans/{scan_id}/export/{file_id}/download | Download exported scan
[**scans_export_request**](ScanExportsApi.md#scans_export_request) | **POST** /scans/{scan_id}/export | Export scan
[**scans_export_status**](ScanExportsApi.md#scans_export_status) | **GET** /scans/{scan_id}/export/{file_id}/status | Check scan export status



## scans_export_download

> serde_json::Value scans_export_download(scan_id, file_id)
Download exported scan

Download an exported scan.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the exported scan you want to download. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**file_id** | **String** | The ID of the file to download (Included in response from /scans/{scan\\_id}/export). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_export_request

> crate::models::ScansExportRequest200Response scans_export_request(scan_id, scans_export_request_request, history_id, history_uuid)
Export scan

Export the specified scan. To see the status of the requested export, submit an [export status](ref:scans-export-status) request. On receiving a \"ready\" status from the export-status request, download the export file using the [export download](ref:scans-export-download) method.  **Note:** If you request a scan export in the `nessus` file format, but do not specify filters for the export, Tenable.io truncates the plugins output data in the export file at 5 MB or 5,000,000 characters, and appends `TRUNCATED` (bracketed by three asterisks) at the end of the output in the export file. You can obtain the full plugins output by exporting the scan in any other file format than `nessus`.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The identifier for the scan you want to export. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_export_request_request** | [**ScansExportRequestRequest**](ScansExportRequestRequest.md) |  | [required] |
**history_id** | Option<**i32**> | The unique identifier of the historical data that you want Tenable.io to export. This identifier corresponds to the `history.id` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. |  |
**history_uuid** | Option<**uuid::Uuid**> | The UUID of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.scan_uuid` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. |  |

### Return type

[**crate::models::ScansExportRequest200Response**](scans_export_request_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_export_status

> crate::models::ScansExportStatus200Response scans_export_status(scan_id, file_id)
Check scan export status

Check the file status of an exported scan. When an export has been requested, it is necessary to poll this endpoint until a \"ready\" status is returned, at which point the file is complete and can be downloaded using the [export download](ref:scans-export-download) endpoint.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**file_id** | **String** | The ID of the file to poll (included in response from /scans/{scan\\_id}/export). | [required] |

### Return type

[**crate::models::ScansExportStatus200Response**](scans_export_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

