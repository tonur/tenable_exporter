# ExportsAssetsRequestExportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chunk_size** | **i32** | Specifies the number of assets per exported chunk. The range is 100-10000. If you specify a value outside of that range, Tenable Vulnerability Management returns a 400 error.   **Note:** Using smaller chunks size can improve performance. Tenable does not recommend using a chunk size larger than 5000 as the potential for an error increases above this amount. | 
**include_open_ports** | Option<**bool**> | Specifies whether or not to include open port findings from info-level plugins. If this parameter is omitted, Tenable Vulnerability Management uses a default value of `false`.  **Caution:** Including open port findings can significantly increase the size of the API response and exports take longer to complete. | [optional]
**filters** | Option<[**crate::models::ExportsAssetsRequestExportRequestFilters**](exports_assets_request_export_request_filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


