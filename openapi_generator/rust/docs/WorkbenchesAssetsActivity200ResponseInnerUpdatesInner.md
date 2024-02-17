# WorkbenchesAssetsActivity200ResponseInnerUpdatesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | Option<**String**> | The update method. Possible values include:   - add—A scan identified a new software application installed on the asset.  - remove—Tenable.io identified the specified application as expired and removed it from the installed_software attribute of the asset. Tenable.io considers an application detection expired if no scan detects the application within 30 days of the scan that originally detected the application. | [optional]
**property** | Option<**String**> | The name of the updated attribute. | [optional]
**value** | Option<**String**> | The updated value of the attribute. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


