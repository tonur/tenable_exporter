# \ScanTasksApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_scans_check_auto_targets**](ScanTasksApi.md#io_scans_check_auto_targets) | **POST** /scans/check-auto-targets | Test scan routes
[**io_scans_count**](ScanTasksApi.md#io_scans_count) | **GET** /scans/count | Get scan count
[**io_scans_credentials_convert**](ScanTasksApi.md#io_scans_credentials_convert) | **POST** /scans/{scan_id}/credentials/{credentials_id}/upgrade | Convert credentials
[**scans_copy**](ScanTasksApi.md#scans_copy) | **POST** /scans/{scan_id}/copy | Copy scan
[**scans_import**](ScanTasksApi.md#scans_import) | **POST** /scans/import | Import uploaded scan
[**scans_schedule**](ScanTasksApi.md#scans_schedule) | **PUT** /scans/{scan_id}/schedule | Enable schedule
[**scans_timezones**](ScanTasksApi.md#scans_timezones) | **GET** /scans/timezones | Get timezones



## io_scans_check_auto_targets

> crate::models::IoScansCheckAutoTargets200Response io_scans_check_auto_targets(io_scans_check_auto_targets_request, limit, matched_resource_limit)
Test scan routes

Evaluates a list of targets and/or tags against the scan route configuration of scanner groups. Returns the list of missed targets (if any) and the list of matched scanner groups (if any).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_scans_check_auto_targets_request** | [**IoScansCheckAutoTargetsRequest**](IoScansCheckAutoTargetsRequest.md) |  | [required] |
**limit** | Option<**i32**> | Limit the number of missed targets returned in the response. |  |
**matched_resource_limit** | Option<**i32**> | Limit the number of matched resource UUIDs returned in the response. |  |

### Return type

[**crate::models::IoScansCheckAutoTargets200Response**](io_scans_check_auto_targets_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_scans_count

> crate::models::IoScansCount200Response io_scans_count(active)
Get scan count

Returns the total number of scans in your container. You can use the `active` query parameter to return only the number of active scans. <div class=\"perms-callout\">Requires BASIC [16] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active** | Option<**bool**> | If `true`, only active scans are counted. If `false`, all active and inactive scans are counted. If this parameter is omitted, Tenable.io defaults to `false`. |  |

### Return type

[**crate::models::IoScansCount200Response**](io_scans_count_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_scans_credentials_convert

> crate::models::IoScansCredentialsConvert200Response io_scans_credentials_convert(scan_id, credentials_id, io_scans_credentials_convert_request)
Convert credentials

Converts scan-specific credentials to managed credentials. For more information, see [Convert Scan-specific Credentials to Managed Credentials](doc:convert-scan-specific-credentials-to-managed-credentials). <div class=\"perms-callout\">Requires SCAN MANAGER [40] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **i32** | The ID of the scan where the scan-specific credentials are currently stored. | [required] |
**credentials_id** | **i32** | The ID of the scan-specific credentials. For more information about determining this ID, see [Convert Scan-specific Credentials to Managed Credentials](doc:convert-scan-specific-credentials-to-managed-credentials). | [required] |
**io_scans_credentials_convert_request** | [**IoScansCredentialsConvertRequest**](IoScansCredentialsConvertRequest.md) |  | [required] |

### Return type

[**crate::models::IoScansCredentialsConvert200Response**](io_scans_credentials_convert_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_copy

> crate::models::ScansCopy200Response scans_copy(scan_id, scans_copy_request)
Copy scan

Copies the specified scan.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The identifier for the scan you want to copy. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_copy_request** | [**ScansCopyRequest**](ScansCopyRequest.md) |  | [required] |

### Return type

[**crate::models::ScansCopy200Response**](scans_copy_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_import

> crate::models::ScansCopy200Response scans_import(scans_import_request, include_aggregate)
Import uploaded scan

Import an existing scan that was previously uploaded using the [POST /file/upload](ref:file-upload) endpoint. Tenable.io supports scan imports up to 4 GB in size.   **Note:** You cannot import results from scans run more than 15 months ago.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scans_import_request** | [**ScansImportRequest**](ScansImportRequest.md) |  | [required] |
**include_aggregate** | Option<**i32**> | Specifies whether to include the imported scan data in the vulnerabilities dashboard views. To include, use `1`. To exclude, use `0`. If you don't specify the include_aggregate parameter, the data does not appear in the dashboard. |  |

### Return type

[**crate::models::ScansCopy200Response**](scans_copy_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_schedule

> crate::models::ScansSchedule200Response scans_schedule(scan_id, scans_schedule_request)
Enable schedule

Enables or disables a scan schedule.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions and CAN EXECUTE [32] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan_id** | **String** | The unique identifier for the scan you want to enable or disable. This identifier can be either the `scans.schedule_uuid` or the `scans.id` attribute in the response message from the [GET /scans](ref:scans-list) endpoint. Tenable recommends that you use `scans.schedule_uuid`. | [required] |
**scans_schedule_request** | [**ScansScheduleRequest**](ScansScheduleRequest.md) |  | [required] |

### Return type

[**crate::models::ScansSchedule200Response**](scans_schedule_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scans_timezones

> Vec<crate::models::ScansTimezones200ResponseInner> scans_timezones()
Get timezones

Returns the timezones list for creating a recurring scan. For more information, see [Example Assessment Scan: Recurring](doc:example-assessment-scan-recurring).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ScansTimezones200ResponseInner>**](scans_timezones_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

