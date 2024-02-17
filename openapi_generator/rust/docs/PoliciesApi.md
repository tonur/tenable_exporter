# \PoliciesApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**policies_configure**](PoliciesApi.md#policies_configure) | **PUT** /policies/{policy_id} | Update policy
[**policies_copy**](PoliciesApi.md#policies_copy) | **POST** /policies/{policy_id}/copy | Copy policy
[**policies_create**](PoliciesApi.md#policies_create) | **POST** /policies | Create policy
[**policies_delete**](PoliciesApi.md#policies_delete) | **DELETE** /policies/{policy_id} | Delete policy
[**policies_details**](PoliciesApi.md#policies_details) | **GET** /policies/{policy_id} | List policy details
[**policies_export**](PoliciesApi.md#policies_export) | **GET** /policies/{policy_id}/export | Export policy
[**policies_import**](PoliciesApi.md#policies_import) | **POST** /policies/import | Import policy
[**policies_list**](PoliciesApi.md#policies_list) | **GET** /policies | List policies



## policies_configure

> serde_json::Value policies_configure(policy_id)
Update policy

Updates an existing policy (scan template).   **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface.<div class=\"perms-callout\">Requires STANDARD [32] user permissions and CAN EXECUTE [32] scan template (policy) permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID of the policy to change. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_copy

> serde_json::Value policies_copy(policy_id)
Copy policy

Copies a policy (scan template).  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface. <div class=\"perms-callout\">Requires STANDARD [32] user permissions and CAN EXECUTE [32] scan template (policy) permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The id of the policy to copy. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_create

> crate::models::PoliciesCreate200Response policies_create(policies_create_request)
Create policy

Creates a policy (scan template).  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface. <div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policies_create_request** | [**PoliciesCreateRequest**](PoliciesCreateRequest.md) |  | [required] |

### Return type

[**crate::models::PoliciesCreate200Response**](policies_create_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_delete

> serde_json::Value policies_delete(policy_id)
Delete policy

Deletes a policy (scan template).   **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface.<div class=\"perms-callout\">Requires STANDARD [32] user permissions and CAN EXECUTE [32] scan template (policy) permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID of the policy to delete. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_details

> crate::models::PoliciesDetails200Response policies_details(policy_id)
List policy details

Returns the details for the specified policy (scan template).  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface. <div class=\"perms-callout\">Requires STANDARD [32] user permissions and CAN USE [32] policy permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID of the policy to retrieve. | [required] |

### Return type

[**crate::models::PoliciesDetails200Response**](policies_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_export

> policies_export(policy_id)
Export policy

Exports the specified policy (scan template).  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface.<div class=\"perms-callout\">Requires STANDARD [32] user permissions and CAN EXECUTE [32] scan template (policy) permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID of the policy to export. | [required] |

### Return type

 (empty response body)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/xml, application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_import

> crate::models::PoliciesImport200Response policies_import(policies_import_request)
Import policy

Imports an existing policy (scan template) that was uploaded using the [POST /file/upload](ref:file-upload) endpoint. Only policies (scan templates) in .nessus format can be imported.  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface. <div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policies_import_request** | [**PoliciesImportRequest**](PoliciesImportRequest.md) |  | [required] |

### Return type

[**crate::models::PoliciesImport200Response**](policies_import_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policies_list

> Vec<crate::models::PoliciesList200ResponseInner> policies_list()
List policies

Returns a list of policies (scan templates).  **Note:** Policies are referred to as scan templates in the new interface. The term policy is only used in the classic interface. <div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PoliciesList200ResponseInner>**](policies_list_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

