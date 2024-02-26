# WorkbenchesAssetInfo200ResponseInfoTagsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag_uuid** | Option<**String**> | The UUID of the tag. | [optional]
**tag_key** | Option<**String**> | The tag category (the first half of the category:value pair). | [optional]
**tag_value** | Option<**String**> | The tag value (the second half of the category:value pair). | [optional]
**added_by** | Option<**String**> | The UUID of the user who assigned the tag to the asset. | [optional]
**added_at** | Option<**String**> | The ISO timestamp when the tag was assigned to the asset. | [optional]
**source** | Option<**String**> | The tag type:  - static—A user manually applied the tag to an asset. You can use the Tenable.io API to create and assign static tags to assets.  - dynamic—Tenable.io automatically applies the tag based on asset attribute rules. For more information, see [Apply Dynamic Tags](doc:apply-dynamic-tags). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

