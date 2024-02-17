# ScansHistoryDetails200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_id** | Option<**i32**> | The unique ID of the owner of the scan. | [optional]
**schedule_uuid** | Option<**String**> | The UUID for a specific instance in the scan schedule. | [optional]
**status** | Option<**String**> | The terminal status of the scan run. For possible values, see [Scan Status](doc:scan-status-tio). | [optional]
**is_archived** | Option<**bool**> | Indicates whether the scan results are older than 35 days (`true`). If this parameter is `true`, Tenable.io returns limited data for the scan run. For complete scan results that are older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. | [optional]
**scan_start** | Option<**i32**> | The Unix timestamp when the scan run started. | [optional]
**owner_uuid** | Option<**i32**> | The UUID of the owner of the scan when the scan run occurred. | [optional]
**owner** | Option<**String**> | The username of the owner of the scan when the scan run occurred. | [optional]
**targets** | Option<**String**> | The hosts that the scan targeted. | [optional]
**object_id** | Option<**i32**> | The unique ID of the scan result object. | [optional]
**uuid** | Option<**String**> | The UUID of the historical data. | [optional]
**scan_end** | Option<**i32**> | The Unix timestamp when the scan run finished. | [optional]
**scan_type** | Option<**String**> | The type of scan: `ps` (a scan performed over the network by a cloud scanner), `remote` (a  scan performed over the network by a local scanner), `agent` (a scan on a local host that a Nessus agent performs directly), or `null` (the scan has never been launched, or the scan is imported). | [optional]
**name** | Option<**String**> | The name of the scan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


