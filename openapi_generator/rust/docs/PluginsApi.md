# \PluginsApi

All URIs are relative to *https://cloud.tenable.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**io_plugins_details**](PluginsApi.md#io_plugins_details) | **GET** /plugins/plugin/{id} | Get plugin details
[**io_plugins_families_list**](PluginsApi.md#io_plugins_families_list) | **GET** /plugins/families | List plugin families
[**io_plugins_family_details_id**](PluginsApi.md#io_plugins_family_details_id) | **GET** /plugins/families/{id} | List plugins in family (ID)
[**io_plugins_family_details_name**](PluginsApi.md#io_plugins_family_details_name) | **POST** /plugins/families/_byName | List plugins in family (name)
[**io_plugins_list**](PluginsApi.md#io_plugins_list) | **GET** /plugins/plugin | List plugins



## io_plugins_details

> crate::models::IoPluginsDetails200Response io_plugins_details(id)
Get plugin details

Returns details for a specified plugin.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the plugin. | [required] |

### Return type

[**crate::models::IoPluginsDetails200Response**](io_plugins_details_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_plugins_families_list

> crate::models::IoPluginsFamiliesList200Response io_plugins_families_list(all)
List plugin families

Returns the list of plugin families.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Specifies whether to return all plugin families. If `true`, the plugin families hidden in Tenable Vulnerability Management UI, for example, Port Scanners, are included in the list. |  |

### Return type

[**crate::models::IoPluginsFamiliesList200Response**](io_plugins_families_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_plugins_family_details_id

> crate::models::IoPluginsFamilyDetailsId200Response io_plugins_family_details_id(id)
List plugins in family (ID)

Returns the list of plugins for the specified family ID.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the plugin family you want to retrieve the list of plugins for. | [required] |

### Return type

[**crate::models::IoPluginsFamilyDetailsId200Response**](io_plugins_family_details_id_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_plugins_family_details_name

> crate::models::IoPluginsFamilyDetailsId200Response io_plugins_family_details_name(io_plugins_family_details_name_request)
List plugins in family (name)

Returns the list of plugins for the specified family name.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See [Permissions](doc:permissions).</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**io_plugins_family_details_name_request** | [**IoPluginsFamilyDetailsNameRequest**](IoPluginsFamilyDetailsNameRequest.md) |  | [required] |

### Return type

[**crate::models::IoPluginsFamilyDetailsId200Response**](io_plugins_family_details_id_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## io_plugins_list

> crate::models::IoPluginsList200Response io_plugins_list(last_updated, size, page)
List plugins

Returns a paginated list of Tenable plugins with detailed plugin information. You can filter the list on the value of the `last_updated` date field. The list is sorted by plugin ID. Note that the `last_updated` parameter does not take VPR updates into account. See note on the `last_updated` parameter description for special considerations.<div class=\"perms-callout\">Requires BASIC [16] user permissions. See <a href=\"/docs/permissions\">Permissions</a>.</div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_updated** | Option<**String**> | The last updated date to filter on in the `YYYY-MM-DD` format. Tenable Vulnerability Management returns only the plugins that have been updated after the specified date.   **Note:** This parameter does not take VPR updates into account. If you need to filter the plugin list by VPR update date, you should use a `last_updated` date of `1970-01-01` to return all plugins, and then filter the result set manually based on the `updated` field in the `vpr` object. |  |
**size** | Option<**i32**> | The number of records to include in the result set. Default is 1,000. The maximum size is 10,000. |  |
**page** | Option<**i32**> | The index of the page to return relative to the specified page size. For example, to return records 10-19 with page size 10, you must specify page 2. If you omit this parameter, Tenable Vulnerability Management applies the default value of 1. |  |

### Return type

[**crate::models::IoPluginsList200Response**](io_plugins_list_200_response.md)

### Authorization

[cloud](../README.md#cloud)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

