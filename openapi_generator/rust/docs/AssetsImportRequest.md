# AssetsImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**Vec<crate::models::AssetsImportRequestAssetsInner>**](assets_import_request_assets_inner.md) | An array of asset objects to import. Each asset object requires a value for at least one of the following properties: fqdn, ipv4, netbios\\_name, mac\\_address.  For an example of this request body, see [Add Asset Data to Tenable.io](doc:add-asset-data-to-tenableio). For the complete list of importable asset attributes, see [Common Asset Attributes](doc:common-asset-attributes#section-asset-attribute-definitions).  **Caution:** AWS, Azure, and GCP asset data must be imported separately even if the data is associated with the same asset. For example, Azure imports should only contain Azure data in the payload. The import fails if AWS data is included with the Azure data in the payload. | 
**source** | **String** | A user-defined name for the source of the import containing the asset records. You can specify only one source for each import. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


