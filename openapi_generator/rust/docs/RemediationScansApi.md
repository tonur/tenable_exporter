# \RemediationScansApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_scans_remediation_create**](RemediationScansApi.md#io_scans_remediation_create) | **POST** /scans/remediation | Create remediation scan
[**io_scans_remediation_list**](RemediationScansApi.md#io_scans_remediation_list) | **GET** /scans/remediation | List remediation scans



## io_scans_remediation_create

> crate::models::ScansCreate200Response io_scans_remediation_create(io_scans_remediation_create_request)
Create remediation scan

Creates a remediation scan configuration. For more information and request body examples, see [Manage Remediation Scans](doc:io-manage-remediation-scans).   **Note:** Tenable.io limits the number of scans you can create to 10,000 scans. Tenable recommends you re-use scheduled scans instead of creating new scans. An HTTP 403 error is returned if you attempt to create a scan after you have already reached the scan limit of 10,000.  <div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_scans_remediation_create_request** | [**IoScansRemediationCreateRequest**](IoScansRemediationCreateRequest.md) |  | [required] |

### Return type

[**crate::models::ScansCreate200Response**](scans_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_scans_remediation_list

> crate::models::IoScansRemediationList200Response io_scans_remediation_list(limit, offset, sort)
List remediation scans

Returns a list of remediation scans where you have at least CAN VIEW [16] scan permissions.   **Note:** Keep in mind potential [rate limits](doc:rate-limiting) when using this endpoint. To check the status of your scans, use the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint. Tenable recommends the [GET /scans/{scan_id}/latest-status](ref:scans-get-latest-status) endpoint especially if you are programmatically checking the status of large numbers of scans.<div class=\"perms-callout\">Requires BASIC [16] user permissions and CAN VIEW [16] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of records to retrieve. Minimum value is `1`. Maximum value is `200`. If this parameter is omitted, Tenable.io uses the default value of `50`. |  |
**offset** | Option<**i32**> | The starting record to retrieve. Minimum value is `0`. If this parameter is omitted, Tenable.io uses the default value of `0`. |  |
**sort** | Option<**String**> | The field you want to sort the results by along with the sort order. Valid sort fields: `scan_creation_date`. Valid sort orders: `desc`, `asc`. `scan_creation_date` is the Unix timestamp when the remediation scan run was created. If this parameter is omitted, Tenable.io uses the default value of `scan_creation_date:desc`. |  |

### Return type

[**crate::models::IoScansRemediationList200Response**](io_scans_remediation_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

