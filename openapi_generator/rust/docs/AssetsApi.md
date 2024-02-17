# \AssetsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assets_asset_info**](AssetsApi.md#assets_asset_info) | **GET** /assets/{asset_uuid} | Get asset details
[**assets_bulk_delete**](AssetsApi.md#assets_bulk_delete) | **POST** /api/v2/assets/bulk-jobs/delete | Bulk delete assets
[**assets_bulk_move**](AssetsApi.md#assets_bulk_move) | **POST** /api/v2/assets/bulk-jobs/move-to-network | Move assets
[**assets_bulk_update_acr**](AssetsApi.md#assets_bulk_update_acr) | **POST** /api/v2/assets/bulk-jobs/acr | Update ACR
[**assets_import**](AssetsApi.md#assets_import) | **POST** /import/assets | Import assets
[**assets_import_job_info**](AssetsApi.md#assets_import_job_info) | **GET** /import/asset-jobs/{asset_import_job_uuid} | Get import job information
[**assets_list_assets**](AssetsApi.md#assets_list_assets) | **GET** /assets | List assets
[**assets_list_import_jobs**](AssetsApi.md#assets_list_import_jobs) | **GET** /import/asset-jobs | List asset import jobs



## assets_asset_info

> crate::models::AssetsAssetInfo200Response assets_asset_info(asset_uuid)
Get asset details

Returns details of the specified asset.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_uuid** | **String** | The UUID of the asset. | [required] |

### Return type

[**crate::models::AssetsAssetInfo200Response**](assets_asset_info_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_bulk_delete

> crate::models::AssetsBulkMove202Response assets_bulk_delete(assets_bulk_delete_request)
Bulk delete assets

Deletes the specified assets. This request creates an asynchronous delete job in Tenable.io.  For information about the assets bulk delete workflow and payload examples, see [Bulk Asset Operations](doc:bulk-asset-operations).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assets_bulk_delete_request** | [**AssetsBulkDeleteRequest**](AssetsBulkDeleteRequest.md) |  | [required] |

### Return type

[**crate::models::AssetsBulkMove202Response**](assets_bulk_move_202_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_bulk_move

> crate::models::AssetsBulkMove202Response assets_bulk_move(assets_bulk_move_request)
Move assets

Moves assets from the specified network to another network. You can use this endpoint to move assets from the default network to a user-defined network, from a user-defined network to the default network, and from one user-defined network to another user-defined network. This request creates an asynchronous job in Tenable.io.  For information about the assets move workflow and payload examples, see [Bulk Asset Operations](doc:bulk-asset-operations).<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assets_bulk_move_request** | [**AssetsBulkMoveRequest**](AssetsBulkMoveRequest.md) |  | [required] |

### Return type

[**crate::models::AssetsBulkMove202Response**](assets_bulk_move_202_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_bulk_update_acr

> assets_bulk_update_acr(assets_bulk_update_acr_request_inner)
Update ACR

Overwrites the Tenable-provided Asset Criticality Rating (ACR) for the specified assets. Tenable assigns an ACR to each asset on your network to represent the asset's relative risk as an integer from 1 to 10. For more information about ACR, see [Lumin metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  You must have a Lumin license to update the ACR for assets in your organization.<div class=\"perms-callout\">Requires SCAN OPERATOR [24] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assets_bulk_update_acr_request_inner** | [**Vec<crate::models::AssetsBulkUpdateAcrRequestInner>**](assets_bulk_update_acr_request_inner.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_import

> crate::models::AssetsImport200Response assets_import(assets_import_request)
Import assets

Imports asset data in JSON format.  The request size cannot exceed 5 MB. For example, if the average asset record you want to import is about 2 KB, you can import approximately 2,500 assets in a single request.  **Note:** This endpoint does not support the network_id attribute in asset objects for import. Tenable.io automatically assigns imported assets to the default network object. For more information about network objects, see [Manage Networks](doc:manage-networks-tio).<div class=\"perms-callout\">Requires SCAN MANAGER [40] user permissions and CAN EDIT [64] scan permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assets_import_request** | [**AssetsImportRequest**](AssetsImportRequest.md) |  | [required] |

### Return type

[**crate::models::AssetsImport200Response**](assets_import_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_import_job_info

> crate::models::AssetsListImportJobs200ResponseInner assets_import_job_info(asset_import_job_uuid)
Get import job information

Gets information about the specified import job.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_import_job_uuid** | **String** | The UUID of the asset import job. | [required] |

### Return type

[**crate::models::AssetsListImportJobs200ResponseInner**](assets_list_import_jobs_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_list_assets

> crate::models::AssetsListAssets200Response assets_list_assets()
List assets

Lists up to 5,000 assets.  **Note:** You can use the [POST /assets/export](ref:exports-assets-request-export) endpoint to export data for all assets. For more information about exporting assets, see [Retrieve Asset Data from Tenable.io](doc:retrieve-asset-data-from-tenableio).<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AssetsListAssets200Response**](assets_list_assets_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets_list_import_jobs

> Vec<crate::models::AssetsListImportJobs200ResponseInner> assets_list_import_jobs()
List asset import jobs

Lists asset import jobs.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AssetsListImportJobs200ResponseInner>**](assets_list_import_jobs_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

