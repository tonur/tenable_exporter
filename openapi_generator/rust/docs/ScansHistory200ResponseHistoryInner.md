# ScansHistory200ResponseHistoryInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_end** | Option<**i32**> | The Unix timestamp when the scan finished running. | [optional]
**scan_uuid** | Option<**String**> | The UUID for the specific scan run. | [optional]
**id** | Option<**i32**> | The unique identifier for the specific scan run. | [optional]
**is_archived** | Option<**bool**> | Indicates whether the scan results are older than 35 days (`true`). If this parameter is `true`, Tenable.io returns limited data for the scan run. For complete scan results that are older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. | [optional]
**time_start** | Option<**i32**> | The Unix timestamp when the scan started running. | [optional]
**visibility** | Option<**String**> | The visibility of the scan results in workbenches (`public` or `private`). | [optional]
**targets** | Option<[**crate::models::ScansHistory200ResponseHistoryInnerTargets**](scans_history_200_response_history_inner_targets.md)> |  | [optional]
**status** | Option<**String**> | The status of the scan. For a list of possible values, see [Scan Status](doc:scan-status-tio). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


