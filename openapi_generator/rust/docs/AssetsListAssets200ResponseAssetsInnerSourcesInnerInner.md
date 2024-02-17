# AssetsListAssets200ResponseAssetsInnerSourcesInnerInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the entity that reported the asset details. Sources can include sensors, connectors, and API imports. Source names can be customized by your organization (for example, you specify a name when you import asset records). If your organization does not customize source names, system-generated names include:  - AWS—You obtained the asset data from an Amazon Web Services connector.  - NESSUS_AGENT—You obtained the asset data obtained from a Nessus agent scan.  - PVS—You obtained the asset data from a Nessus Network Monitor (NNM) scan.  - NESSUS_SCAN—You obtained the asset data from a Nessus scan.  - WAS—You obtained the asset data from a  Web Application Scanning scan. | [optional]
**first_seen** | Option<**String**> | The ISO timestamp when the source first reported the asset. | [optional]
**last_seen** | Option<**String**> | The ISO timestamp when the source last reported the asset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


