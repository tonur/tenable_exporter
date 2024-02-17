# \VulnerabilitiesApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vulnerabilities_import**](VulnerabilitiesApi.md#vulnerabilities_import) | **POST** /import/vulnerabilities | Import vulnerabilities v1
[**vulnerabilities_import_v2**](VulnerabilitiesApi.md#vulnerabilities_import_v2) | **POST** /api/v2/vulnerabilities | Import vulnerabilities v2



## vulnerabilities_import

> serde_json::Value vulnerabilities_import(vulnerabilities_import_request)
Import vulnerabilities v1

Imports a list of vulnerabilities in JSON format. The request cannot exceed 15 MB in total size. In addition, the request can contain a maximum of 50 asset objects. For request body examples, see [Add Vulnerability Data to Tenable.io](doc:add-vulnerability-data-to-tenableio).  **Caution** This endpoint is deprecated. Tenable recommends that you use the [POST /api/v2/vulnerabilities](#vulnerabilities-import-v2) endpoint instead.  **Note:** This endpoint can only import Tenable scan data. It cannot import vulnerability information from third-party vendors. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vulnerabilities_import_request** | [**VulnerabilitiesImportRequest**](VulnerabilitiesImportRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vulnerabilities_import_v2

> crate::models::VulnerabilitiesImportV2200Response vulnerabilities_import_v2(vulnerabilities_import_v2_request)
Import vulnerabilities v2

Imports a list of vulnerabilities in JSON format. The request cannot exceed 15 MB in total size. In addition, the request can contain a maximum of 50 asset objects. For request body examples, see [Add Vulnerability Data to Tenable.io](doc:add-vulnerability-data-to-tenableio).  **Note:** This endpoint can only import Tenable scan data. It cannot import vulnerability information from third-party vendors. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vulnerabilities_import_v2_request** | [**VulnerabilitiesImportV2Request**](VulnerabilitiesImportV2Request.md) |  | [required] |

### Return type

[**crate::models::VulnerabilitiesImportV2200Response**](vulnerabilities_import_v2_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

