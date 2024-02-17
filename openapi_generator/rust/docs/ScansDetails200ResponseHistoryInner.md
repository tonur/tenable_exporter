# ScansDetails200ResponseHistoryInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**history_id** | Option<**i32**> | The unique ID of the historical data. | [optional]
**owner_id** | Option<**i32**> | The unique ID of the owner of the scan. | [optional]
**creation_date** | Option<**i32**> | The creation date for the historical data in Unix time. | [optional]
**last_modification_date** | Option<**i32**> | The last modification date for the historical data in Unix time. | [optional]
**uuid** | Option<**String**> | The UUID of the historical data. | [optional]
**r#type** | Option<**String**> | The type of scan: `local` (a credentialed scan performed over the network), `remote` (an uncredentialed scan performed over the network, `agent` (a scan on a local host that a Nessus agent performs directly), or `null` (the scan has never been launched, or the scan is imported). | [optional]
**status** | Option<**String**> | The terminal status of the scan run. For a list of possible values, see [Scan Status](doc:scan-status-tio). | [optional]
**scheduler** | Option<**i32**> | If `true`, Tenable.io launched the scan automatically from a schedule. | [optional]
**alt_targets_used** | Option<**bool**> | If `true`, Tenable.io did not not launched with a target list. This parameter is `true` for agent scans. | [optional]
**is_archived** | Option<**bool**> | Indicates whether the scan results are older than 35 days (`true`). If this parameter is `true`, Tenable.io returns limited data for the scan run. For complete scan results that are older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


