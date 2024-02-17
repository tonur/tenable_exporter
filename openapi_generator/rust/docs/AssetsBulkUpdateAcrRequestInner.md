# AssetsBulkUpdateAcrRequestInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acr_score** | **i32** | The ACR score you want to assign to the asset. The ACR must be an integer from 1 to 10. | 
**reason** | Option<**Vec<String>**> | The reasons you are updating the ACR for the assets. Supported values include:   - Business Critical  - In Scope For Compliance  - Existing Mitigation Control  - Dev only   - Key drivers does not match   - Other  This parameter corresponds to the **Overwrite Reasoning** parameter when editing an ACR in the Tenable.io Lumin user interface. For more information, see [Edit an ACR](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminEditACR.htm). | [optional]
**note** | Option<**String**> | Any notes you want to add to clarify the circumstances behind the update. This parameter corresponds to the **Note** parameter when editing an ACR in the Tenable.io Lumin user interface. For more information, see [Edit an ACR](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminEditACR.htm).   | [optional]
**asset** | [**Vec<crate::models::AssetsBulkUpdateAcrRequestInnerAssetInner>**](assets_bulk_update_acr_request_inner_asset_inner.md) | The identifiers of the assets to update to the specified ACR. At least one asset object is required in this array. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


