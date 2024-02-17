# \AssetAttributesApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_v3_asset_attributes_assign**](AssetAttributesApi.md#io_v3_asset_attributes_assign) | **PUT** /api/v3/assets/{asset_id}/attributes | Assign attributes to asset
[**io_v3_asset_attributes_assigned_delete**](AssetAttributesApi.md#io_v3_asset_attributes_assigned_delete) | **DELETE** /api/v3/assets/{asset_id}/attributes | Delete attributes from asset
[**io_v3_asset_attributes_assigned_list**](AssetAttributesApi.md#io_v3_asset_attributes_assigned_list) | **GET** /api/v3/assets/{asset_id}/attributes | List attributes assigned to asset
[**io_v3_asset_attributes_create**](AssetAttributesApi.md#io_v3_asset_attributes_create) | **POST** /api/v3/assets/attributes | Create attribute
[**io_v3_asset_attributes_delete**](AssetAttributesApi.md#io_v3_asset_attributes_delete) | **DELETE** /api/v3/assets/attributes/{attribute_id} | Delete attribute
[**io_v3_asset_attributes_list**](AssetAttributesApi.md#io_v3_asset_attributes_list) | **GET** /api/v3/assets/attributes | List attributes
[**io_v3_asset_attributes_single_delete**](AssetAttributesApi.md#io_v3_asset_attributes_single_delete) | **DELETE** /api/v3/assets/{asset_id}/attributes/{attribute_id} | Delete attribute from asset
[**io_v3_asset_attributes_single_update**](AssetAttributesApi.md#io_v3_asset_attributes_single_update) | **PUT** /api/v3/assets/{asset_id}/attributes/{attribute_id} | Assign single attribute to asset
[**io_v3_asset_attributes_update**](AssetAttributesApi.md#io_v3_asset_attributes_update) | **PUT** /api/v3/assets/attributes/{attribute_id} | Update attribute



## io_v3_asset_attributes_assign

> io_v3_asset_attributes_assign(asset_id, io_v3_asset_attributes_assign_request)
Assign attributes to asset

Assigns custom asset attributes to the specified asset. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset for which you want to assign custom asset attributes. | [required] |
**io_v3_asset_attributes_assign_request** | [**IoV3AssetAttributesAssignRequest**](IoV3AssetAttributesAssignRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_assigned_delete

> io_v3_asset_attributes_assigned_delete(asset_id)
Delete attributes from asset

Deletes all custom asset attributes assigned to the specified asset. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset for which you want to remove all custom attributes. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_assigned_list

> crate::models::IoV3AssetAttributesAssignedList200Response io_v3_asset_attributes_assigned_list(asset_id)
List attributes assigned to asset

Returns a list of custom asset attributes assigned to the specified asset.  **Note:** A container can have a maximum of 10 custom attributes. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset you want to list custom asset attributes for. | [required] |

### Return type

[**crate::models::IoV3AssetAttributesAssignedList200Response**](io_v3_asset_attributes_assigned_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_create

> io_v3_asset_attributes_create(io_v3_asset_attributes_create_request)
Create attribute

Creates a new custom asset attribute. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_v3_asset_attributes_create_request** | [**IoV3AssetAttributesCreateRequest**](IoV3AssetAttributesCreateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_delete

> io_v3_asset_attributes_delete(attribute_id)
Delete attribute

Deletes the specified custom asset attribute and removes it from all assets that it's assigned to. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | The ID of the custom asset attribute that you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_list

> crate::models::IoV3AssetAttributesList200Response io_v3_asset_attributes_list()
List attributes

Returns a list of custom asset attributes.  **Note:** A container can have a maximum of 10 custom asset attributes. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IoV3AssetAttributesList200Response**](io_v3_asset_attributes_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_single_delete

> io_v3_asset_attributes_single_delete(asset_id, attribute_id)
Delete attribute from asset

Deletes a single custom asset attribute from the specified asset. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset for which you want to remove a single custom asset attribute. | [required] |
**attribute_id** | **String** | The ID of the custom asset attribute that you want to remove from the asset. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_single_update

> io_v3_asset_attributes_single_update(asset_id, attribute_id, io_v3_asset_attributes_single_update_request)
Assign single attribute to asset

Assigns a single custom asset attribute to the specified asset. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The ID of the asset for which you want to assign a single custom asset attribute. | [required] |
**attribute_id** | **String** | The ID of the custom asset attribute you want to assign to the asset. | [required] |
**io_v3_asset_attributes_single_update_request** | [**IoV3AssetAttributesSingleUpdateRequest**](IoV3AssetAttributesSingleUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_v3_asset_attributes_update

> io_v3_asset_attributes_update(attribute_id, io_v3_asset_attributes_update_request)
Update attribute

Updates the specified custom asset attribute.  **Note:** You can only update non-key attributes like `description`. <div class=\"perms-callout\">Requires ADMINISTRATOR [64] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | The ID of the custom attribute that you want to update. | [required] |
**io_v3_asset_attributes_update_request** | [**IoV3AssetAttributesUpdateRequest**](IoV3AssetAttributesUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

