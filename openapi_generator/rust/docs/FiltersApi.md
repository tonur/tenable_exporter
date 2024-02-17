# \FiltersApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_filters_agents_list**](FiltersApi.md#io_filters_agents_list) | **GET** /filters/scans/agents | List agent filters
[**io_filters_assets_list**](FiltersApi.md#io_filters_assets_list) | **GET** /filters/workbenches/assets | List asset filters
[**io_filters_assets_list_v2**](FiltersApi.md#io_filters_assets_list_v2) | **POST** /filters/workbenches/assets | List asset filters v2
[**io_filters_credentials_list**](FiltersApi.md#io_filters_credentials_list) | **GET** /filters/credentials | List credential filters
[**io_filters_scan_history_list**](FiltersApi.md#io_filters_scan_history_list) | **GET** /filters/scans/reports/history | List scan history filters
[**io_filters_scan_list**](FiltersApi.md#io_filters_scan_list) | **GET** /filters/scans/reports | List scan filters
[**io_filters_vulnerabilities_workbench_list**](FiltersApi.md#io_filters_vulnerabilities_workbench_list) | **GET** /filters/workbenches/vulnerabilities | List vulnerability filters
[**io_filters_vulnerabilities_workbench_list_v2**](FiltersApi.md#io_filters_vulnerabilities_workbench_list_v2) | **POST** /filters/workbenches/vulnerabilities | List vulnerability filters v2
[**vm_filters_reports_list**](FiltersApi.md#vm_filters_reports_list) | **GET** /filters/reports/export | List report filters



## io_filters_agents_list

> crate::models::IoFiltersAgentsList200Response io_filters_agents_list()
List agent filters

Lists the filtering, sorting, and pagination capabilities available for agent records on endpoints that support them.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_assets_list

> crate::models::IoFiltersAgentsList200Response io_filters_assets_list()
List asset filters

Lists the filtering, sorting, and pagination capabilities available for assets on endpoints that support them. For more information about these filters, see [Workbench Filters](doc:workbench-filters).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_assets_list_v2

> crate::models::IoFiltersAgentsList200Response io_filters_assets_list_v2(io_filters_assets_list_v2_request)
List asset filters v2

Lists the filtering, sorting, and pagination capabilities available for assets on endpoints that support them. For more information about these filters, see [Workbench Filters](doc:workbench-filters).  **Note:** This endpoint returns an additional filter (`tag_uuid`), displayed as `Tag (Category: Value)`, that is not returned when using the GET version of this endpoint.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_filters_assets_list_v2_request** | Option<[**IoFiltersAssetsListV2Request**](IoFiltersAssetsListV2Request.md)> |  |  |

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_credentials_list

> crate::models::IoFiltersAgentsList200Response io_filters_credentials_list()
List credential filters

Returns the filtering, sorting, and pagination capabilities available for scan credentials on endpoints that support them. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_scan_history_list

> crate::models::IoFiltersAgentsList200Response io_filters_scan_history_list()
List scan history filters

Lists the filtering, sorting, and pagination capabilities available for scan history records on endpoints that support them.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_scan_list

> crate::models::IoFiltersScanList200Response io_filters_scan_list()
List scan filters

Lists the filtering, sorting, and pagination capabilities available for scan records on endpoints that support them.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersScanList200Response**](io_filters_scan_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_vulnerabilities_workbench_list

> crate::models::IoFiltersAgentsList200Response io_filters_vulnerabilities_workbench_list()
List vulnerability filters

Returns the filters available for the Vulnerabilities Workbench. For more information about these filters, see [Workbench Filters](doc:workbench-filters).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_filters_vulnerabilities_workbench_list_v2

> crate::models::IoFiltersAgentsList200Response io_filters_vulnerabilities_workbench_list_v2(io_filters_assets_list_v2_request)
List vulnerability filters v2

Returns the filters available for the Vulnerabilities Workbench. For more information about these filters, see [Workbench Filters](doc:workbench-filters).  **Note:** This endpoint returns an additional filter (`tag_uuid`), displayed as `Tag (Category: Value)`, that is not returned when using the GET version of this endpoint.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_filters_assets_list_v2_request** | Option<[**IoFiltersAssetsListV2Request**](IoFiltersAssetsListV2Request.md)> |  |  |

### Return type

[**crate::models::IoFiltersAgentsList200Response**](io_filters_agents_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vm_filters_reports_list

> crate::models::VmFiltersReportsList200Response vm_filters_reports_list()
List report filters

Returns the filters, supported operators, data types, and allowed values for the [POST /reports/export](ref:vm-reports-create) endpoint. For more information about the supported filters, see [Report Export Filters](doc:vm-report-export-filters). <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VmFiltersReportsList200Response**](vm_filters_reports_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

