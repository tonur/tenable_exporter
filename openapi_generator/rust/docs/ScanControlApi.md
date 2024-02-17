# \ScanControlApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scans_launch**](ScanControlApi.md#scans_launch) | **POST** /scans/{scan_id}/launch | Launch scan
[**scans_pause**](ScanControlApi.md#scans_pause) | **POST** /scans/{scan_id}/pause | Pause scan
[**scans_resume**](ScanControlApi.md#scans_resume) | **POST** /scans/{scan_id}/resume | Resume scan
[**scans_stop**](ScanControlApi.md#scans_stop) | **POST** /scans/{scan_id}/stop | Stop scan
[**vm_scans_stop_force**](ScanControlApi.md#vm_scans_stop_force) | **POST** /scans/{schedule_uuid}/force-stop | Force stop scan



## scans_launch

> crate::models::ScansLaunch200Response scans_launch(scan_id, scans_launch_request)
Launch scan

Launches a scan. For more information, see [Launch a Scan](doc:launch-scan-tio).  **Note:** There is a limit of 25 active scans per container. You can use use the [Get scan count](ref:io-scans-count) endpoint to retrieve the total number of active scans in your container. For more information, see [Concurrency Limiting](doc:concurrency-limiting). <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to launch. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_launch_request** | [**ScansLaunchRequest**](ScansLaunchRequest.md) |  | [required] |

### Return type

[**crate::models::ScansLaunch200Response**](scans_launch_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_pause

> serde_json::Value scans_pause(scan_id)
Pause scan

Pauses a scan. You can only pause scans that have a `running` status.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to pause. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_resume

> serde_json::Value scans_resume(scan_id)
Resume scan

Resumes a scan. You can only resume a scan that has a status of `paused`.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to resume. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_stop

> serde_json::Value scans_stop(scan_id)
Stop scan

Stops a scan.   You can only stop a scan that has a status of `pending`, `running`, or `resuming`. To stop a scan with a status of `stopping` or `publishing`, use the [Force stop scan](ref:vm-scans-stop-force) endpoint. For more information about scan statuses, see [Scan Status](doc:scan-status-tio).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The identifier for the scan you want to stop. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the [GET /scans](ref:scans-list) endpoint's response message. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vm_scans_stop_force

> serde_json::Value vm_scans_stop_force(schedule_uuid)
Force stop scan

Force stops a scan. A force stop cancels all the scan's incomplete scan tasks and updates the scan status to `aborted`. Tenable Vulnerability Management processes and indexes the completed scan tasks. After you force stop a scan, Tenable recommends re-running the scan in its entirety to ensure total scan coverage.   You can use the force stop endpoint to abort a stalled scan in the `stopping` or `publishing` status. This can be helpful when you need to abort a scan before a freeze window or before a subsequent scheduled scan begins.    You can only force stop a scan that has a status of `stopping` or `publishing`. For more information about scan statuses, see [Scan Status](doc:scan-status-tio).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_uuid** | **String** | The identifier for the scan you want to force stop. For the identifier, use the `scans.schedule_uuid` in the [GET /scans](ref:scans-list) endpoint's response message. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

