# \FileApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**file_upload**](FileApi.md#file_upload) | **POST** /file/upload | Upload file



## file_upload

> crate::models::FileUpload200Response file_upload(no_enc, filedata)
Upload file

Uploads a file.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**no_enc** | Option<**i32**> | Send value of `1` when uploading an encrypted file. |  |
**filedata** | Option<**std::path::PathBuf**> | The file to upload. |  |

### Return type

[**crate::models::FileUpload200Response**](file_upload_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

