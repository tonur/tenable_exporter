# \ExportsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exports_assets_download_chunk**](ExportsApi.md#exports_assets_download_chunk) | **GET** /assets/export/{export_uuid}/chunks/{chunk_id} | Download assets chunk
[**exports_assets_export_cancel**](ExportsApi.md#exports_assets_export_cancel) | **POST** /assets/export/{export_uuid}/cancel | Cancel asset export
[**exports_assets_export_status**](ExportsApi.md#exports_assets_export_status) | **GET** /assets/export/{export_uuid}/status | Get assets export status
[**exports_assets_export_status_recent**](ExportsApi.md#exports_assets_export_status_recent) | **GET** /assets/export/status | Get asset export jobs
[**exports_assets_request_export**](ExportsApi.md#exports_assets_request_export) | **POST** /assets/export | Export assets
[**exports_vulns_download_chunk**](ExportsApi.md#exports_vulns_download_chunk) | **GET** /vulns/export/{export_uuid}/chunks/{chunk_id} | Download vulnerabilities chunk
[**exports_vulns_export_cancel**](ExportsApi.md#exports_vulns_export_cancel) | **POST** /vulns/export/{export_uuid}/cancel | Cancel vuln export
[**exports_vulns_export_status**](ExportsApi.md#exports_vulns_export_status) | **GET** /vulns/export/{export_uuid}/status | Get vulnerabilities export status
[**exports_vulns_export_status_recent**](ExportsApi.md#exports_vulns_export_status_recent) | **GET** /vulns/export/status | Get vuln export jobs
[**exports_vulns_request_export**](ExportsApi.md#exports_vulns_request_export) | **POST** /vulns/export | Export vulnerabilities
[**io_exports_compliance_cancel**](ExportsApi.md#io_exports_compliance_cancel) | **POST** /compliance/export/{export_uuid}/cancel | Cancel compliance export
[**io_exports_compliance_create**](ExportsApi.md#io_exports_compliance_create) | **POST** /compliance/export | Export compliance data
[**io_exports_compliance_download**](ExportsApi.md#io_exports_compliance_download) | **GET** /compliance/export/{export_uuid}/chunks/{chunk_id} | Download compliance chunk
[**io_exports_compliance_status**](ExportsApi.md#io_exports_compliance_status) | **GET** /compliance/export/{export_uuid}/status | Get compliance export status



## exports_assets_download_chunk

> crate::models::ExportsAssetsDownloadChunk200Response exports_assets_download_chunk(export_uuid, chunk_id)
Download assets chunk

Downloads exported assets chunk by ID. Chunks are available for download for up to 24 hours after they have been created. Tenable Vulnerability Management returns a 404 message for expired chunks. <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID of the export request. | [required] |
**chunk_id** | **i32** | The ID of the asset chunk you want to export. | [required] |

### Return type

[**crate::models::ExportsAssetsDownloadChunk200Response**](exports_assets_download_chunk_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_assets_export_cancel

> crate::models::ExportsVulnsExportCancel200Response exports_assets_export_cancel(export_uuid)
Cancel asset export

Cancels the specified assets export job. If you cancel an export job, Tenable Vulnerability Management finishes any chunk that is currently processing, terminates the processing of any unprocessed chunks, and updates the job status to `CANCELLED`. If a canceled job includes completed chunks, you can download those chunks for three days after cancellation.<div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID for the export request. | [required] |

### Return type

[**crate::models::ExportsVulnsExportCancel200Response**](exports_vulns_export_cancel_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_assets_export_status

> crate::models::ExportsAssetsExportStatus200Response exports_assets_export_status(export_uuid)
Get assets export status

Returns the status of an assets export request. Tenable Vulnerability Management processes the chunks in parallel so the chunks may not complete in order, and the chunk IDs may not be arranged sequentially in the completed output. <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID for the export request. | [required] |

### Return type

[**crate::models::ExportsAssetsExportStatus200Response**](exports_assets_export_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_assets_export_status_recent

> crate::models::ExportsAssetsExportStatusRecent200Response exports_assets_export_status_recent()
Get asset export jobs

Retrieves a list of asset export jobs. This list includes the 1,000 most recent export jobs regardless of status. <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExportsAssetsExportStatusRecent200Response**](exports_assets_export_status_recent_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_assets_request_export

> crate::models::ExportsAssetsRequestExport200Response exports_assets_request_export(exports_assets_request_export_request)
Export assets

Exports all assets that match the request criteria.   Users need at least BASIC [16] user permissions and Can View access control permissions for the asset objects they want to export. You can set Can View permissions for the All Assets object for all assets. For more information about this endpoint, see guidelines and limitations described in [Retrieve Asset Data from Vulnerability Management](doc:retrieve-asset-data-from-tenableio).    **Note:** There are limits for concurrent export requests. For more information, see [Concurrency Limiting](doc:concurrency-limiting) and [Rate Limiting](doc:rate-limiting). <div class=\"perms-callout\">Requires BASIC [16] user permissions and Can View access control permissions for the asset objects to be exported. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exports_assets_request_export_request** | [**ExportsAssetsRequestExportRequest**](ExportsAssetsRequestExportRequest.md) |  | [required] |

### Return type

[**crate::models::ExportsAssetsRequestExport200Response**](exports_assets_request_export_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_vulns_download_chunk

> crate::models::ExportsVulnsDownloadChunk200Response exports_vulns_download_chunk(export_uuid, chunk_id)
Download vulnerabilities chunk

Downloads exported vulnerabilities as a JSON file. The response content type is `application/octet-stream`.  Chunks are available for download for up to 24 hours after they have been created. Tenable Vulnerability Management returns a 404 message for expired chunks. Export chunks do not include an attribute if that attribute is empty in the vulnerability record.  A successful response message contains attributes that correspond to CVSS metrics; these metrics are described fully in the following documents:<ul><li>[A Complete Guide to the Common Vulnerability Scoring System, Version 2.0](https://www.first.org/cvss/v2/guide)</li><li>[Common Vulnerability Scoring System v3.0: Specification Document](https://www.first.org/cvss/specification-document)</li></ul><div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID of the vulnerability export request. | [required] |
**chunk_id** | **i32** | The ID of the chunk you want to export. | [required] |

### Return type

[**crate::models::ExportsVulnsDownloadChunk200Response**](exports_vulns_download_chunk_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_vulns_export_cancel

> crate::models::ExportsVulnsExportCancel200Response exports_vulns_export_cancel(export_uuid)
Cancel vuln export

Cancels the specified vulnerability export job. If you cancel an export job, Tenable Vulnerability Management finishes any chunk that is currently processing, terminates the processing of any unprocessed chunks, and updates the job status to `CANCELLED`. If a canceled job includes completed chunks, you can download those chunks for three days after cancellation. <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID for the export request. | [required] |

### Return type

[**crate::models::ExportsVulnsExportCancel200Response**](exports_vulns_export_cancel_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_vulns_export_status

> crate::models::ExportsVulnsExportStatus200Response exports_vulns_export_status(export_uuid)
Get vulnerabilities export status

Returns the status of a vulnerability export request. Tenable Vulnerability Management processes the chunks in parallel so the chunks may not complete in order, and the chunk IDs may not be arranged sequentially in the completed output.   **Note:** Output for an individual plugin is limited to 1,024 KB (1 MB). <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID for the vulnerability export request. | [required] |

### Return type

[**crate::models::ExportsVulnsExportStatus200Response**](exports_vulns_export_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_vulns_export_status_recent

> crate::models::ExportsVulnsExportStatusRecent200Response exports_vulns_export_status_recent()
Get vuln export jobs

Retrieves a list of vulnerability export jobs. This list includes the 1,000 most recent export jobs regardless of status. However, this list includes completed jobs only if the job completed in the previous three days. <div class=\"perms-callout\">Requires BASIC [16] user permissions. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExportsVulnsExportStatusRecent200Response**](exports_vulns_export_status_recent_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exports_vulns_request_export

> crate::models::ExportsVulnsRequestExport200Response exports_vulns_request_export(exports_vulns_request_export_request)
Export vulnerabilities

Exports vulnerabilities that match the request criteria.   Users need at least BASIC [16] user permissions and Can View access control permissions for the asset objects they want to export. You can set Can View permissions for the All Assets object for all assets. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. For more information about this endpoint, see guidelines and limitations described in [Retrieve Vulnerability Data from Vulnerability Management](doc:retrieve-vulnerability-data-from-tenableio).   **Note:** There are limits for concurrent export requests. For more information, see [Concurrency Limiting](doc:concurrency-limiting) and [Rate Limiting](doc:rate-limiting). <div class=\"perms-callout\">Requires BASIC [16] user permissions and Can View access control permissions for the asset objects to be exported. FedRAMP customers who use the GovCloud region must have ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exports_vulns_request_export_request** | [**ExportsVulnsRequestExportRequest**](ExportsVulnsRequestExportRequest.md) |  | [required] |

### Return type

[**crate::models::ExportsVulnsRequestExport200Response**](exports_vulns_request_export_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_exports_compliance_cancel

> io_exports_compliance_cancel(export_uuid)
Cancel compliance export

Cancels the specified compliance export job. If you cancel an export job, Tenable Vulnerability Management finishes any chunk that is currently processing, terminates the processing of any unprocessed chunks, and updates the job status to `CANCELLED`. If a canceled job includes completed chunks, you can still download those chunks after cancellation. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID of the export request you want to cancel. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_exports_compliance_create

> crate::models::IoExportsComplianceCreate200Response io_exports_compliance_create(io_exports_compliance_create_request)
Export compliance data

Exports compliance data that matches the request criteria. The compliance export API does not return results for individual scans. Instead, the exported data is an aggregation / summary of all compliance scan data. As new compliance scans are run, the summary is updated with the latest audit results.  **Note:** If no request body is submitted with the POST request a `500 Internal Server Error` will be returned. If you do not wish to use any body parameters, you must still supply an empty JSON body with the request in order to avoid this error. For example: `{}`   **Note:** There are limits for concurrent export requests. For more information, see [Concurrency Limiting](doc:concurrency-limiting) and [Rate Limiting](doc:rate-limiting). <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_exports_compliance_create_request** | [**IoExportsComplianceCreateRequest**](IoExportsComplianceCreateRequest.md) |  | [required] |

### Return type

[**crate::models::IoExportsComplianceCreate200Response**](io_exports_compliance_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_exports_compliance_download

> Vec<crate::models::IoExportsComplianceDownload200ResponseInner> io_exports_compliance_download(export_uuid, chunk_id)
Download compliance chunk

Downloads exported compliance data chunks by ID. Tenable Vulnerability Management processes the chunks in parallel. Chunks are available for download for up to 24 hours after they have been created. Tenable Vulnerability Management returns a 404 message for expired chunks. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID of the compliance export request. | [required] |
**chunk_id** | **i32** | The ID of the compliance chunk you want to export. | [required] |

### Return type

[**Vec<crate::models::IoExportsComplianceDownload200ResponseInner>**](io_exports_compliance_download_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_exports_compliance_status

> crate::models::IoExportsComplianceStatus200Response io_exports_compliance_status(export_uuid)
Get compliance export status

Returns the status of a compliance export request. Tenable Vulnerability Management processes the chunks in parallel so the chunks may not complete in order. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_uuid** | **String** | The UUID for the compliance export request. | [required] |

### Return type

[**crate::models::IoExportsComplianceStatus200Response**](io_exports_compliance_status_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

