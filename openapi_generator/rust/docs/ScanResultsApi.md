# \ScanResultsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scans_attachments**](ScanResultsApi.md#scans_attachments) | **GET** /scans/{scan_id}/attachments/{attachment_id} | Get scan attachment file
[**scans_host_details**](ScanResultsApi.md#scans_host_details) | **GET** /scans/{scan_uuid}/hosts/{host_id} | Get host details
[**scans_plugin_output**](ScanResultsApi.md#scans_plugin_output) | **GET** /scans/{scan_uuid}/hosts/{host_id}/plugins/{plugin_id} | Get plugin output



## scans_attachments

> serde_json::Value scans_attachments(scan_id, attachment_id, key)
Get scan attachment file

Gets the requested scan attachment file.  **Note:** Scan attachments are not available if the scan results are older than 60 days. To retrieve scan data in `.nessus` and `.csv` format, use the [POST /scans/{scan_id}/export](ref:scans-attachments) endpoint.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan containing the attachment. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**attachment_id** | **String** | The ID of the scan attachment. | [required] |
**key** | **String** | The attachment access token. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_host_details

> crate::models::ScansHostDetails200Response scans_host_details(scan_uuid, host_id, history_id, history_uuid)
Get host details

Returns details for the specified host.   **Note:** This endpoint can only return scan results that are younger than 35 days. For scan results that are older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_uuid** | **String** | The identifier for the scan. This identifier can be the either the `schedule_uuid` or the numeric `id` attribute for the scan. We recommend that you use `schedule_uuid`. | [required] |
**host_id** | **i32** | The ID of the host to retrieve. | [required] |
**history_id** | Option<**i32**> | The unique identifier of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.id` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint.  |  |
**history_uuid** | Option<**uuid::Uuid**> | The UUID of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.scan_uuid` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. |  |

### Return type

[**crate::models::ScansHostDetails200Response**](scans_host_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_plugin_output

> crate::models::ScansPluginOutput200Response scans_plugin_output(scan_uuid, host_id, plugin_id, history_id, history_uuid)
Get plugin output

Returns the output for a specified plugin.  **Caution:** This endpoint is deprecated. Tenable recommends that you use the [Exports API](ref:exports) endpoints instead. Please update any existing integrations that your organization has since this endpoint will be removed.  **Note:** This endpoint can only return scan results that are younger than 35 days. For scan results that are older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. Output for an individual plugin is limited to 1,024 KB (1 MB). Additionally, this endpoint has a rate limit of 8 requests per minute. For more information, see [Rate Limiting](doc:rate-limiting).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_uuid** | **String** | The identifier for the scan. This identifier can be the either the `schedule_uuid` or the numeric `id` attribute for the scan. We recommend that you use `schedule_uuid`. | [required] |
**host_id** | **i32** | The ID of the host to retrieve. | [required] |
**plugin_id** | **i32** | The ID of the plugin to retrieve. | [required] |
**history_id** | Option<**i32**> | The unique identifier of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.id` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint.  |  |
**history_uuid** | Option<**uuid::Uuid**> | The UUID of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.scan_uuid` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. |  |

### Return type

[**crate::models::ScansPluginOutput200Response**](scans_plugin_output_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

