# \ScanHistoryApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scans_delete_history**](ScanHistoryApi.md#scans_delete_history) | **DELETE** /scans/{scan_id}/history/{history_id} | Delete scan history
[**scans_history**](ScanHistoryApi.md#scans_history) | **GET** /scans/{scan_id}/history | Get scan history
[**scans_history_details**](ScanHistoryApi.md#scans_history_details) | **GET** /scans/{scan_id}/history/{history_uuid} | Get scan history details



## scans_delete_history

> serde_json::Value scans_delete_history(scan_id, history_id, delete_rollovers)
Delete scan history

Deletes historical results from a scan. Note that rollover scan data is also deleted.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**history_id** | **i32** | The unique identifier of the historical data that you want Tenable.io to delete. This identifier corresponds to the `history.id` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. | [required] |
**delete_rollovers** | Option<**bool**> | Indicates whether or not to delete scan rollover history. If `true`, the scan history and its associated rollover scan data are deleted. If `false` or null and the scan has rollover data, you receive a 409 error: \"This scan contains rollover scan data. If you want to delete it and all the associated rollover scan data, use query parameter `delete_rollovers=true`.\" |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_history

> crate::models::ScansHistory200Response scans_history(scan_id, exclude_rollover, limit, offset, sort)
Get scan history

Returns a list of objects, each of which represent an individual run of the specified scan.  **Note:** Scans with `agent_scan_launch_type` set to `triggered` will not have scan history. <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**exclude_rollover** | **bool** | Indicates whether or not to exclude rollover scans from the scan history. If no value is provided for this parameter, Tenable.io uses the default value `false`. | [required] |
**limit** | Option<**i32**> | The number of records to retrieve. If this parameter is omitted, Tenable.io uses the default value of `50`. |  |
**offset** | Option<**i32**> | The starting record to retrieve. If this parameter is omitted, Tenable.io uses the default value of `0`. |  |
**sort** | Option<**String**> | The field you want to use to sort the results by along with the sort order. The field is specified first, followed by a colon, and the order is specified second (`asc` or `desc`). For example, `start_date:desc` would sort results by the `start_date` field in descending order.  If you specify multiple fields, the fields must be separated by commas. For example, `start_date:desc,status:asc` would first sort results by the `start_date` field in descending order and then by the `status` field in ascending order. |  |

### Return type

[**crate::models::ScansHistory200Response**](scans_history_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_history_details

> crate::models::ScansHistoryDetails200Response scans_history_details(scan_id, history_uuid)
Get scan history details

Returns the details of a previous run of the specified scan. Scan details include information about when and where the scan ran, as well as the scan results for the target hosts.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan in your Tenable.io instance. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**history_uuid** | **uuid::Uuid** | The UUID of the historical scan result to return details about. This identifier corresponds to the `history.scan_uuid` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. | [required] |

### Return type

[**crate::models::ScansHistoryDetails200Response**](scans_history_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

