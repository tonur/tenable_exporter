# \ScansApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scans_configure**](ScansApi.md#scans_configure) | **PUT** /scans/{scan_id} | Update scan
[**scans_create**](ScansApi.md#scans_create) | **POST** /scans | Create scan
[**scans_delete**](ScansApi.md#scans_delete) | **DELETE** /scans/{scan_id} | Delete scan
[**scans_details**](ScansApi.md#scans_details) | **GET** /scans/{scan_id} | Get scan details
[**scans_list**](ScansApi.md#scans_list) | **GET** /scans | List scans



## scans_configure

> crate::models::ScansCreate200Response scans_configure(scan_id, scans_configure_request)
Update scan

Updates the scan configuration. For example, you can enable or disable a scan, change the scan name, description, folder, scanner, targets, and schedule parameters. For more information and request body examples, see [Update a Scan](doc:update-scan-tio).   For information on updating remediation scans and request body examples, see [Manage Remediation Scans](doc:io-manage-remediation-scans). <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to update. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_configure_request** | [**ScansConfigureRequest**](ScansConfigureRequest.md) |  | [required] |

### Return type

[**crate::models::ScansCreate200Response**](scans_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_create

> crate::models::ScansCreate200Response scans_create(scans_create_request)
Create scan

Creates a scan configuration. For more information and request body examples, see [Create a Scan](doc:create-scan-tio).   **Note:** Tenable.io limits the number of scans you can create to 10,000 scans. Tenable recommends you re-use scheduled scans instead of creating new scans. An HTTP 403 error is returned if you attempt to create a scan after you have already reached the scan limit of 10,000.  <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scans_create_request** | [**ScansCreateRequest**](ScansCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ScansCreate200Response**](scans_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_delete

> serde_json::Value scans_delete(scan_id)
Delete scan

Deletes a scan.  You cannot delete scans with a `running`, `paused`, or `stopping` status. For more information, see [Scan Status](doc:scan-status-tio). <div class=\"perms-callout\">Requires SCAN MANAGER [40] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to delete. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_details

> crate::models::ScansDetails200Response scans_details(scan_id, history_id)
Get scan details

Returns scan results for a specific scan. If you submit a request without query parameters, Tenable.io returns results from the latest run of the specified scan. If you submit a request using the query parameters to specify a historical run of the scan, Tenable.io returns the scan results for the specified run.   **Note:** Keep in mind potential [rate limits](doc:rate-limiting) and [concurrency limits](doc:concurrency-limiting) when using this endpoint. To check the status of a scan, use the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint. Tenable recommends the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint especially if you are programmatically checking the status of large numbers of scans.   **Note:** This endpoint returns limited data if scan results are older than 35 days. Scans that are older than 35 days are effectively \"archived\". The `info.is_archived` attribute in the response message for this endpoint serves as the indication of archival status. For complete data from archived scan results, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **uuid::Uuid** | The unique identifier for the scan you want to retrieve. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**history_id** | Option<**String**> | The unique identifier of the historical data that you want Tenable.io to return. This identifier corresponds to the `history.id` attribute of the response message from the [GET /scans/{scan_id}/history](ref:scans-history) endpoint. |  |

### Return type

[**crate::models::ScansDetails200Response**](scans_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_list

> crate::models::ScansList200Response scans_list(folder_id, last_modification_date)
List scans

Returns a list of scans where you have at least CAN VIEW [16] scan permissions.   **Note:** Keep in mind potential [rate limits](doc:rate-limiting) when using this endpoint. To check the status of your scans, use the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint. Tenable recommends the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint especially if you are programmatically checking the status of large numbers of scans.<div class=\"perms-callout\">Requires BASIC [16] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | Option<**i32**> | The ID of the folder where the scans you want to list are stored. |  |
**last_modification_date** | Option<**i32**> | Limit the results to those scans that have run since the specified time. This parameter does not represent the date on which the scan configuration was last modified. Must be in Unix time format. |  |

### Return type

[**crate::models::ScansList200Response**](scans_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

