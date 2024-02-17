# \WorkbenchesApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workbenches_asset_info**](WorkbenchesApi.md#workbenches_asset_info) | **GET** /workbenches/assets/{asset_id}/info | Get asset information
[**workbenches_asset_vulnerabilities**](WorkbenchesApi.md#workbenches_asset_vulnerabilities) | **GET** /workbenches/assets/{asset_id}/vulnerabilities | List asset vulnerabilities
[**workbenches_asset_vulnerability_info**](WorkbenchesApi.md#workbenches_asset_vulnerability_info) | **GET** /workbenches/assets/{asset_id}/vulnerabilities/{plugin_id}/info | Get asset vulnerability details
[**workbenches_asset_vulnerability_output**](WorkbenchesApi.md#workbenches_asset_vulnerability_output) | **GET** /workbenches/assets/{asset_id}/vulnerabilities/{plugin_id}/outputs | List asset vulnerabilities for plugin
[**workbenches_assets**](WorkbenchesApi.md#workbenches_assets) | **GET** /workbenches/assets | List assets
[**workbenches_assets_activity**](WorkbenchesApi.md#workbenches_assets_activity) | **GET** /workbenches/assets/{asset_uuid}/activity | Get asset activity log
[**workbenches_assets_delete**](WorkbenchesApi.md#workbenches_assets_delete) | **DELETE** /workbenches/assets/{asset_uuid} | Delete asset
[**workbenches_assets_vulnerabilities**](WorkbenchesApi.md#workbenches_assets_vulnerabilities) | **GET** /workbenches/assets/vulnerabilities | List assets with vulnerabilities
[**workbenches_export_download**](WorkbenchesApi.md#workbenches_export_download) | **GET** /workbenches/export/{file_id}/download | Download export file
[**workbenches_export_request**](WorkbenchesApi.md#workbenches_export_request) | **GET** /workbenches/export | Export workbench
[**workbenches_export_status**](WorkbenchesApi.md#workbenches_export_status) | **GET** /workbenches/export/{file_id}/status | Check export status
[**workbenches_vulnerabilities**](WorkbenchesApi.md#workbenches_vulnerabilities) | **GET** /workbenches/vulnerabilities | List vulnerabilities
[**workbenches_vulnerability_info**](WorkbenchesApi.md#workbenches_vulnerability_info) | **GET** /workbenches/vulnerabilities/{plugin_id}/info | Get plugin details
[**workbenches_vulnerability_output**](WorkbenchesApi.md#workbenches_vulnerability_output) | **GET** /workbenches/vulnerabilities/{plugin_id}/outputs | List plugin outputs



## workbenches_asset_info

> crate::models::WorkbenchesAssetInfo200Response workbenches_asset_info(asset_id, all_fields)
Get asset information

Returns information about the specified asset.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The UUID of the asset. You can find the UUID by examining the output of the [GET /workbenches/assets](ref:workbenches-assets) endpoint. | [required] |
**all_fields** | Option<**String**> | A value specifying whether you want the returned data to include all fields (`full`) or only the default fields (`default`).  The schema for this endpoint defines the `default` fields only. For a definition of the `full` fields, see [Common Asset Attributes](doc:common-asset-attributes). |  |

### Return type

[**crate::models::WorkbenchesAssetInfo200Response**](workbenches_asset_info_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_asset_vulnerabilities

> crate::models::WorkbenchesVulnerabilities200Response workbenches_asset_vulnerabilities(asset_id, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
List asset vulnerabilities

Retrieves a list of the vulnerabilities recorded for a specified asset. By default, this list is sorted by vulnerability count in descending order. The list returned is limited to 5,000. To retrieve more than 5,000 vulnerabilities, use the [export-request API](ref:workbenches-export-request).   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The UUID of the asset. You can find the UUID by examining the output of the [GET /workbenches/assets](ref:workbenches-assets) endpoint. | [required] |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**crate::models::WorkbenchesVulnerabilities200Response**](workbenches_vulnerabilities_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_asset_vulnerability_info

> crate::models::WorkbenchesAssetVulnerabilityInfo200Response workbenches_asset_vulnerability_info(asset_id, plugin_id, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
Get asset vulnerability details

Retrieves the details for a vulnerability recorded on a specified asset.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The UUID of the asset. | [required] |
**plugin_id** | **i32** | The ID of the plugin. | [required] |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**crate::models::WorkbenchesAssetVulnerabilityInfo200Response**](workbenches_asset_vulnerability_info_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_asset_vulnerability_output

> crate::models::WorkbenchesAssetVulnerabilityOutput200Response workbenches_asset_vulnerability_output(asset_id, plugin_id, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
List asset vulnerabilities for plugin

Retrieves the vulnerability outputs for a plugin recorded on a specified asset.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The UUID of the asset. | [required] |
**plugin_id** | **i32** | The ID of the plugin. | [required] |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**crate::models::WorkbenchesAssetVulnerabilityOutput200Response**](workbenches_asset_vulnerability_output_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_assets

> crate::models::WorkbenchesAssets200Response workbenches_assets(date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type, all_fields)
List assets

Retrieves a list of assets. The list can be modified using filters. The list returned is limited to 5,000. To retrieve more than 5,000 assets, use the [export-request API](ref:exports-assets-request-export).   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. If you specify the name of the filter, you must specify the operator as the filter.0.quality parameter and the value as the filter.0.value parameter. To use multiple filters, increment the `<INDEX>` portion of `filter.<INDEX>.filter`, for example, `filter.0.filter`. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters).   **Note:** The value is case sensitive when used with the `match` (contains) or `nmatch` (does not contain) operators. |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**all_fields** | Option<**String**> | A value specifying whether you want the returned data to include all fields (`full`) or only the default fields (`default`). The schema for this endpoint defines the `default` fields only. For a definition of the `full` fields, see [Common Asset Attributes](doc:common-asset-attributes). |  |

### Return type

[**crate::models::WorkbenchesAssets200Response**](workbenches_assets_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_assets_activity

> Vec<crate::models::WorkbenchesAssetsActivity200ResponseInner> workbenches_assets_activity(asset_uuid)
Get asset activity log

Returns the activity log for the specified asset. Event types include:<ul><li>discovered—Asset created (for example, by a network scan or import).</li><li>seen—Asset observed by a network scan without any changes to its attributes.</li><li>tagging—Tag added to or removed from asset.</li><li>attribute_change—A scan identified new or changed attributes for the asset (for example, new software applications installed on the asset).</li><li>updated—Asset updated either manually by a user or automatically by a new scan.</li></ul>   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_uuid** | **String** | The UUID of the asset. You can find the UUID by examining the output of the [GET /workbenches/assets](ref:workbenches-assets) endpoint. | [required] |

### Return type

[**Vec<crate::models::WorkbenchesAssetsActivity200ResponseInner>**](workbenches_assets_activity_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_assets_delete

> serde_json::Value workbenches_assets_delete(asset_uuid)
Delete asset

Deletes the specified asset. When you delete an asset, Tenable.io deletes vulnerability data associated with the asset. Deleting an asset does not immediately subtract the asset from your licensed assets count. Deleted assets continue to be included in the count until they automatically age out as inactive.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_uuid** | **String** | The UUID of the asset. You can find the UUID by examining the output of the [GET /workbenches/assets](ref:workbenches-assets) endpoint. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_assets_vulnerabilities

> Vec<crate::models::WorkbenchesAssetsVulnerabilities200ResponseInner> workbenches_assets_vulnerabilities(date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
List assets with vulnerabilities

Returns a list of assets with vulnerabilities. The list is limited to 5,000. To retrieve more than 5,000 assets, use the [export-request API](ref:workbenches-export-request).   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint.For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**Vec<crate::models::WorkbenchesAssetsVulnerabilities200ResponseInner>**](workbenches_assets_vulnerabilities_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_export_download

> serde_json::Value workbenches_export_download(file_id)
Download export file

Downloads a file that has been prepared for export. For more information about workbench export files, see [Export File Formats](doc:export-file-formats).   **Caution:** This endpoint is deprecated. Tenable recommends that customers use the [Create report](ref:vm-reports-create) endpoint instead. Please update any existing integrations that your organization has.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **i32** | The unique identifier of the workbench report being downloaded. The value for this parameter can be obtained from the response of the initial export request. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_export_request

> crate::models::WorkbenchesExportRequest200Response workbenches_export_request(format, report, chapter, start_date, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type, minimum_vuln_info, plugin_id, asset_id)
Export workbench

Exports the specified workbench to a file. Once requested, the file can be downloaded using the [export download](ref:workbenches-export-download) method upon receiving a \"ready\" status from the [export status](ref:workbenches-export-status) method. For more information about workbench export files, see [Export File Formats](doc:export-file-formats).  **Caution:** This endpoint is deprecated. Tenable recommends that customers use the [Create report](ref:vm-reports-create) endpoint instead. Please update any existing integrations that your organization has.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | **String** | The file format to use (Nessus, HTML, PDF, or CSV).  **Note:** Tag-based filters are supported for the `CSV` file format type only. | [required] |
**report** | **String** | The type of workbench report to be exported | [required] |
**chapter** | **String** | Semicolon-separated list of chapters to include for vulnerabilities/hosts reports (vuln\\_by\\_plugin, vuln\\_by\\_asset, vuln\\_hosts\\_summary) or a single chapter for Executive Summary (exec\\_summary). Currently, only vuln\\_by\\_asset is supported for .nessus workbench exports. | [required] |
**start_date** | Option<**i32**> | The date (in unixtime) at which the exported results should begin to be included. Defaults to today. |  |
**date_range** | Option<**i32**> | The number of days of data prior to and including start\\_date that should be returned. If not specified, data for all dates is returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/assets](ref:io-filters-assets-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**minimum_vuln_info** | Option<**bool**> | When `true`, Tenable.io returns only a minimal subset of scan details for each result, excluding plugin attributes. In this case, only plugin\\_output and vulnerability\\_state fields are always returned; first\\_found, last\\_found and last\\_fixed are also returned if possible. |  |
**plugin_id** | Option<**i32**> | A plugin ID. Restricts the export data to vulnerabilities that only the specified plugin detects. |  |
**asset_id** | Option<**String**> | The UUID of an asset. Restricts the export data to findings on the specified asset only. |  |

### Return type

[**crate::models::WorkbenchesExportRequest200Response**](workbenches_export_request_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_export_status

> crate::models::WorkbenchesExportStatus200Response workbenches_export_status(file_id)
Check export status

Returns the status of a pending export. When an export has been requested, it is necessary to poll this endpoint until a \"ready\" status is returned, at which point the file is complete and can be downloaded using the [export download](ref:workbenches-export-download) endpoint.  **Caution:** This endpoint is deprecated. Tenable recommends that customers use the [Create report](ref:vm-reports-create) endpoint instead. Please update any existing integrations that your organization has.    **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **i32** | The unique identifier of the workbench report being exported. The value for this parameter can be obtained from the response of the initial export request. | [required] |

### Return type

[**crate::models::WorkbenchesExportStatus200Response**](workbenches_export_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_vulnerabilities

> crate::models::WorkbenchesVulnerabilities200Response workbenches_vulnerabilities(age, authenticated, date_range, exploitable, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type, resolvable, severity)
List vulnerabilities

Returns a list of recorded vulnerabilities. The list returned is limited to 5,000. To retrieve more than 5,000 vulnerabilities, use the [export-request API](ref:workbenches-export-request). Additionally, this endpoint only returns data less than 450 days (15 months) old.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**age** | Option<**i32**> | Lists only those vulnerabilities older than a certain number of days. |  |
**authenticated** | Option<**bool**> | Lists only authenticated vulnerabilities. |  |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**exploitable** | Option<**bool**> | Lists only exploitable vulnerabilities. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters).  **Note:** There is a limit of 10 filters. A `400 Bad Request` error is returned if you exceed this limit. |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters).  **Note:** There is a limit of 10 filter values per filter. A `400 Bad Request` error is returned if you exceed this limit. |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**resolvable** | Option<**bool**> | Lists only those vulnerabilities with a remediation path. |  |
**severity** | Option<**String**> | Lists only vulnerabilities of a specific severity (critical, high, medium, low, or info). |  |

### Return type

[**crate::models::WorkbenchesVulnerabilities200Response**](workbenches_vulnerabilities_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_vulnerability_info

> crate::models::WorkbenchesVulnerabilityInfo200Response workbenches_vulnerability_info(plugin_id, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
Get plugin details

Retrieves the details for a plugin.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **i32** | The ID of the plugin. You can find the plugin ID by examining the output of the [GET /workbenches/vulnerabilities](ref:workbenches-vulnerabilities) endpoint. | [required] |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**crate::models::WorkbenchesVulnerabilityInfo200Response**](workbenches_vulnerability_info_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workbenches_vulnerability_output

> Vec<crate::models::WorkbenchesVulnerabilityOutput200ResponseInner> workbenches_vulnerability_output(plugin_id, date_range, filter_period_0_period_filter, filter_period_0_period_quality, filter_period_0_period_value, filter_period_search_type)
List plugin outputs

Retrieves the vulnerability outputs for a plugin. The list returned is limited to 5,000. To retrieve more than 5,000 vulnerability outputs, use the [export-request API](ref:workbenches-export-request). Additionally, this endpoint only returns data less than 450 days (15 months) old.   **Note:** This endpoint is not intended for large or frequent exports of vulnerability or assets data. If you experience errors, reduce the volume, [rate](doc:rate-limiting), or [concurrency](doc:concurrency-limiting) of your requests or narrow your filters. Contact support if you continue to experience errors. Additionally, Tenable recommends the [POST /vulns/export](ref:exports-vulns-request-export) endpoint for large or frequent exports of vulnerability data, and the [POST /assets/export](ref:exports-assets-request-export) endpoint for large or frequent exports of assets data.  For information and best practices for retrieving vulnerability and assets data from Tenable.io, see [Retrieve Vulnerability Data from Tenable.io](doc:retrieve-vulnerability-data-from-tenableio) and [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **i32** | The ID of the plugin. You can find the plugin ID by examining the output of the [GET /workbenches/vulnerabilities](ref:workbenches-vulnerabilities) endpoint. | [required] |
**date_range** | Option<**i32**> | The number of days of data prior to and including today that should be returned. |  |
**filter_period_0_period_filter** | Option<**String**> | The name of the filter to apply to the exported scan report. You can find available filters by using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_quality** | Option<**String**> | The operator of the filter to apply to the exported scan report. You can find the operators for the filter using the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_0_period_value** | Option<**String**> | The value of the filter to apply to the exported scan report. You can find valid values for the filter in the 'control' attribute of the objects returned by the [GET /filters/workbenches/vulnerabilities](ref:io-filters-vulnerabilities-workbench-list) endpoint. For more information about the format of this parameter, see [Workbench Filters](doc:workbench-filters). |  |
**filter_period_search_type** | Option<**String**> | For multiple filters, specifies whether to use the AND or the OR logical operator. The default is AND. For more information about this parameter, see [Workbench Filters](doc:workbench-filters). |  |

### Return type

[**Vec<crate::models::WorkbenchesVulnerabilityOutput200ResponseInner>**](workbenches_vulnerability_output_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

