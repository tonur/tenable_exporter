# \EditorApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**editor_audits**](EditorApi.md#editor_audits) | **GET** /editor/{type}/{object_id}/audits/{file_id} | Download audit file
[**editor_details**](EditorApi.md#editor_details) | **GET** /editor/{type}/{id} | Get configuration details
[**editor_list_templates**](EditorApi.md#editor_list_templates) | **GET** /editor/{type}/templates | List templates
[**editor_plugin_description**](EditorApi.md#editor_plugin_description) | **GET** /editor/policy/{policy_id}/families/{family_id}/plugins/{plugin_id} | Get plugin details
[**editor_template_details**](EditorApi.md#editor_template_details) | **GET** /editor/{type}/templates/{wizard_uuid} | Get template details



## editor_audits

> std::path::PathBuf editor_audits(r#type, object_id, file_id)
Download audit file

Downloads the specified custom audit file associated with the scan or policy. The file ID can be found in the scan or policy details using the /editor/{type}/{object_id} endpoint.<div class=\"perms-callout\">Requires CAN EXECUTE [32] scan template (policy) permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of template to retrieve (scan or policy). | [required] |[default to scan]
**object_id** | **i32** | The unique ID of the object. | [required] |
**file_id** | **i32** | The ID of the file to export. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## editor_details

> crate::models::EditorDetails200Response editor_details(r#type, id)
Get configuration details

Gets the configuration details for the scan or user-defined template (policy).<div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of object (`scan` or `policy`). | [required] |[default to scan]
**id** | **i32** | The unique ID of the scan or user-defined template (policy). | [required] |

### Return type

[**crate::models::EditorDetails200Response**](editor_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## editor_list_templates

> Vec<crate::models::EditorListTemplates200ResponseInner> editor_list_templates(r#type)
List templates

Lists Tenable-provided scan templates. Tenable provides a number of scan templates to facilitate the creation of scans and scan policies. For a full description of these scan templates, see the [*Tenable.io Vulnerability Management Guide*](https://docs.tenable.com/vulnerability-management/Content/Scans/Templates.htm). <div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of templates to retrieve (scan, policy, or remediation). | [required] |[default to scan]

### Return type

[**Vec<crate::models::EditorListTemplates200ResponseInner>**](editor_list_templates_200_response_inner.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## editor_plugin_description

> crate::models::EditorPluginDescription200Response editor_plugin_description(policy_id, family_id, plugin_id)
Get plugin details

Gets the details of the plugin associated with the scan or policy.<div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID of the policy to look up. | [required] |
**family_id** | **i32** | The ID of the family to lookup within the policy. | [required] |
**plugin_id** | **i32** | The ID of the plugin to lookup within the family. | [required] |

### Return type

[**crate::models::EditorPluginDescription200Response**](editor_plugin_description_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## editor_template_details

> crate::models::EditorTemplateDetails200Response editor_template_details(r#type, wizard_uuid)
Get template details

Gets details for the specified template.<div class=\"perms-callout\">Requires STANDARD [32] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of template to retrieve (scan, policy, or remediation). | [required] |[default to scan]
**wizard_uuid** | **String** | The UUID for the template. | [required] |

### Return type

[**crate::models::EditorTemplateDetails200Response**](editor_template_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

