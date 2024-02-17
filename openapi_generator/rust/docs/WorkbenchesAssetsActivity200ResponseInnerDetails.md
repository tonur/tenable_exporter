# WorkbenchesAssetsActivity200ResponseInnerDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | Option<**String**> | The UUID of the asset. | [optional]
**container_id** | Option<**String**> | The UUID of your Tenable.io instance. | [optional]
**created_at** | Option<**i32**> | The timestamp of the asset creation. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**updated_at** | Option<**i32**> | The timestamp of the asset update time. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**has_agent** | Option<**bool**> | Specifies whether the asset has an agent installed. | [optional]
**has_plugin_results** | Option<**bool**> | Specifies whether or not any plugin results match this asset. | [optional]
**first_scan_time** | Option<**i32**> | The timestamp of the completion of the scan that discovered or observed the asset for the first time. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**last_scan_time** | Option<**i32**> | The timestamp of the completion of the last asset scan. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**last_authenticated_scan_time** | Option<**i32**> | The timestamp of the completion of the last authenticated scan of the asset. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**last_licensed_scan_time** | Option<**i32**> | The timestamp of the scan completion time when asset was last scanned and matched license v1 requirements. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**last_licensed_scan_time_v2** | Option<**i32**> | The timestamp of the scan completion time when asset was last scanned and matched license v2 requirements. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**sources** | Option<[**Vec<crate::models::WorkbenchesAssetsActivity200ResponseInnerDetailsSourcesInner>**](workbenches_assets_activity_200_response_inner_details_sources_inner.md)> | An array of source objects representing the entity that logged the event. | [optional]
**terminated_at** | Option<**i32**> | If terminated, the timestamp of asset termination. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**terminated_by** | Option<**String**> | The UUID of the user that terminated the asset. | [optional]
**deleted_at** | Option<**i32**> | If deleted, the timestamp of asset deletion. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**deleted_by** | Option<**String**> | The UUID of the user that deleted the asset. | [optional]
**properties** | Option<[**serde_json::Value**](.md)> | Additional asset attributes. For attribute definitions, see [Common Asset Attributes](doc:common-asset-attributes). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


