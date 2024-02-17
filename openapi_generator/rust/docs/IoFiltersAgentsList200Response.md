# IoFiltersAgentsList200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wildcard_fields** | Option<**Vec<String>**> | Array of strings which represent each field which supports \"wildcard\" search. Wildcard search is a mechanism where multiple fields of a record are filtered against one specific filter string. If any one of the supported fields' values matches against the filter string, then the record matches the wildcard filter. Note that for a record to be returned, it must pass the wildcard filter (if there is one) AND the set of standard filters. | [optional]
**filters** | Option<[**Vec<crate::models::IoFiltersAgentsList200ResponseFiltersInner>**](io_filters_agents_list_200_response_filters_inner.md)> | A list of filters available for the record type. | [optional]
**sort** | Option<[**crate::models::IoFiltersAgentsList200ResponseSort**](io_filters_agents_list_200_response_sort.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


