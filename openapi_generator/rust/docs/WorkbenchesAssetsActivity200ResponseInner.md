# WorkbenchesAssetsActivity200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Event type:  - discovered—Asset created (for example, by a network scan or import).  - seen—Asset observed by a network scan without any changes to its attributes.  - tagging—Tag added to or removed from asset.  - attribute_change—A scan identified new or changed attributes for the asset (for example, new software applications installed on the asset).  - updated—Asset updated either manually by a user or automatically by a new scan. | [optional]
**timestamp** | Option<**i32**> | The timestamp of the event. The timestamp is reported in ISO 8601 format in UTC time. | [optional]
**scan_id** | Option<**String**> | The UUID of the scan that logged the event. | [optional]
**schedule_id** | Option<**String**> | The ID of the scheduled scan associated with the event. | [optional]
**source** | Option<**String**> | The entity that logged the event, for example, NESSUS_AGENT, NESSUS_AGENT, PVS, or WAS. | [optional]
**details** | Option<[**crate::models::WorkbenchesAssetsActivity200ResponseInnerDetails**](workbenches_assets_activity_200_response_inner_details.md)> |  | [optional]
**updates** | Option<[**Vec<crate::models::WorkbenchesAssetsActivity200ResponseInnerUpdatesInner>**](workbenches_assets_activity_200_response_inner_updates_inner.md)> | (attribute_change entries only) A list of updates to the asset's attributes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


