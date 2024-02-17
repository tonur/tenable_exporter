# AssetsBulkDeleteRequestQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | Option<**String**> | The name of the asset attribute to match. Asset attributes can include tags, for example, `tag.city`. | [optional]
**operator** | Option<**String**> | The operator to apply to the matched value, for example, `eq` (equals), `neq` (does not equal), or `match` (contains). | [optional]
**value** | Option<**String**> | The asset attribute value to match.   **Note:** The value is case sensitive when used with the `match` (contains) or `nmatch` (does not contain) operators. | [optional]
**and** | Option<[**Vec<crate::models::AssetsBulkDeleteRequestQueryAndInner>**](assets_bulk_delete_request_query_and_inner.md)> | To select assets that match all of multiple conditions, specify the conditions inside the `and` array. | [optional]
**or** | Option<[**Vec<crate::models::AssetsBulkDeleteRequestQueryAndInner>**](assets_bulk_delete_request_query_and_inner.md)> | To select assets that match any of multiple conditions, specify the conditions inside the `or` array. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


