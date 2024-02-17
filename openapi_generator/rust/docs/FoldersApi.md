# \FoldersApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**folders_create**](FoldersApi.md#folders_create) | **POST** /folders | Create folder
[**folders_delete**](FoldersApi.md#folders_delete) | **DELETE** /folders/{folder_id} | Delete folder
[**folders_edit**](FoldersApi.md#folders_edit) | **PUT** /folders/{folder_id} | Rename folder
[**folders_list**](FoldersApi.md#folders_list) | **GET** /folders | List folders



## folders_create

> crate::models::FoldersCreate200Response folders_create(folders_create_request)
Create folder

Creates a new custom folder for the current user. There is a rate limit of 10 folder creation requests per minute. <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folders_create_request** | [**FoldersCreateRequest**](FoldersCreateRequest.md) |  | [required] |

### Return type

[**crate::models::FoldersCreate200Response**](folders_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_delete

> serde_json::Value folders_delete(folder_id)
Delete folder

Deletes a folder. If you delete a folder that contains scans, Tenable.io automatically moves those scans to the Trash folder. You cannot delete Tenable-provided folders or custom folders that belong to other users (even if you use an administrator account). <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i32** | The ID of the custom folder to delete. Use the [GET /folders](/reference#folders-list) endpoint to determine the ID of the custom folder you want to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_edit

> serde_json::Value folders_edit(folder_id, folders_edit_request)
Rename folder

Renames a folder for the current user. You cannot rename Tenable-provided scan folders or custom folder that belong to other users (even if your account has administrator privileges). <div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **i32** | The ID of the folder to edit. | [required] |
**folders_edit_request** | [**FoldersEditRequest**](FoldersEditRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_list

> Vec<crate::models::FoldersList200ResponseInner> folders_list()
List folders

Lists both Tenable-provided folders and the current user's custom folders.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::FoldersList200ResponseInner>**](folders_list_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

