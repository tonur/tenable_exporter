# \ScanStatusApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_vm_scans_progress_get**](ScanStatusApi.md#io_vm_scans_progress_get) | **GET** /scans/{scan_id}/progress | Get scan progress
[**scans_get_latest_status**](ScanStatusApi.md#scans_get_latest_status) | **GET** /scans/{scan_id}/latest-status | Get latest scan status
[**scans_read_status**](ScanStatusApi.md#scans_read_status) | **PUT** /scans/{scan_id}/status | Update scan status



## io_vm_scans_progress_get

> crate::models::IoVmScansProgressGet200Response io_vm_scans_progress_get(scan_id, history_id, history_uuid)
Get scan progress

Returns the progress for the specified scan.  **Note:** If you submit a request without query parameters, Tenable.io returns the progress from the latest run of the specified scan. If you submit a request using the `history_id` or `history_uuid` query parameters to specify a historical run of the scan, Tenable.io returns the progress for the specified historical run.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier of the scan you want to retrieve the progress for.  For the identifier, you can use either the `scans.schedule_uuid` or the `scans.id` attribute from the [GET /scans](ref:scans-list) endpoint. However, Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**history_id** | Option<**i32**> | The unique identifier of the historical run of the scan that you want to retrieve the progress for.  This identifier corresponds to the `history.id` attribute from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. You can use either this parameter or the `history_uuid` query parameter to specify a historical run of the scan. |  |
**history_uuid** | Option<**uuid::Uuid**> | The UUID of the historical run of the scan that you want to retrieve the progress for.  This identifier corresponds to the `history.scan_uuid` attribute from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. You can use either this parameter or the `history_id` query parameter to specify a historical run of the scan. |  |

### Return type

[**crate::models::IoVmScansProgressGet200Response**](io_vm_scans_progress_get_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_get_latest_status

> crate::models::ScansGetLatestStatus200Response scans_get_latest_status(scan_id)
Get latest scan status

Returns the latest status for a scan. For a list of possible status values, see [Scan Status](doc:scan-status-tio).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan.  This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |

### Return type

[**crate::models::ScansGetLatestStatus200Response**](scans_get_latest_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_read_status

> serde_json::Value scans_read_status(scan_id, scans_read_status_request)
Update scan status

Changes the status of a scan. For a list of possible status values, see [Scan Status](doc:scan-status-tio).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_read_status_request** | [**ScansReadStatusRequest**](ScansReadStatusRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

