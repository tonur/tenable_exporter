# ScansDetails200ResponseInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The owner of the scan. | [optional]
**name** | Option<**String**> | The name of the scan. | [optional]
**no_target** | Option<**bool**> | Indicates whether the scan based on this policy can specify targets. | [optional]
**folder_id** | Option<**i32**> | The unique ID of the destination folder for the scan. | [optional]
**control** | Option<**bool**> | If `true`, the scan has a schedule and can be launched. | [optional]
**user_permissions** | Option<**i32**> | The sharing permissions for the scan. | [optional]
**schedule_uuid** | Option<**String**> | The UUID for a specific instance in the scan schedule. | [optional]
**edit_allowed** | Option<**bool**> | If `true`, the requesting user can edit this scan configuration. | [optional]
**scanner_name** | Option<**String**> | The name of the scanner configured to run the scan. | [optional]
**policy** | Option<**String**> | The name of the scan template associated with the scan. | [optional]
**shared** | Option<**bool**> | If `true`, the scan is shared with users other than the owner. The level of sharing is specified in the `acls` attribute of the scan details. | [optional]
**object_id** | Option<**i32**> | The unique ID of the scan result object. | [optional]
**tag_targets** | Option<**Vec<String>**> | The list of asset tag identifiers the scan uses to determine which assets it evaluates. For more information about tag-based scans, see [Manage Tag-Based Scans](doc:manage-tag-based-scans-tio). | [optional]
**acls** | Option<[**Vec<crate::models::ScansCreateRequestSettingsAclsInner>**](scans_create_request_settings_acls_inner.md)> | An array of objects that control sharing permissions for the scan. | [optional]
**hostcount** | Option<**i32**> | The total number of assets scanned for vulnerabilities. | [optional]
**uuid** | Option<**String**> | The UUID of the scan. | [optional]
**status** | Option<**String**> | The status of the scan. For a list of possible values, see [Scan Status](doc:scan-status-tio). | [optional]
**scan_type** | Option<**String**> | The type of scan: `ps` (a scan performed over the network by a cloud scanner), `remote` (a  scan performed over the network by a local scanner), `agent` (a scan on a local host that a Nessus agent performs directly), or `null` (the scan has never been launched, or the scan is imported). | [optional]
**targets** | Option<**String**> | A comma-delimited list of IPv4 addresses that are configured as targets for the scan. | [optional]
**alt_targets_used** | Option<**bool**> | If `true`, Tenable.io did not launch the scan with a target list. This parameter is `true` for agent scans. | [optional]
**pci_can_upload** | Option<**bool**> | If `true`, you can submit the results of the scan for PCI ASV review. For more information, see [PCI ASV](https://docs.tenable.com/vulnerability-management/Content/PCI_ASV/GetStarted.htm) in the Tenable Vulnerability Management User Guide. | [optional]
**scan_start** | Option<**i32**> | The Unix timestamp when the scan run started. | [optional]
**timestamp** | Option<**i32**> | The Unix timestamp when the scan run finished. | [optional]
**is_archived** | Option<**bool**> | Indicates whether the scan results are older than 35 days (`true`). If this attribute is `true`, the response message for this endpoint excludes the `hosts`, `vulnerabilities`, `comphosts`, `compliance`, and `filters` objects. For complete scan results older than 35 days, use the [POST /scans/{scan_id}/export](ref:scans-export-request) endpoint instead. | [optional]
**scan_end** | Option<**i32**> | The Unix timestamp when the scan run finished. | [optional]
**haskb** | Option<**bool**> | Indicates whether a scan has a Knowledge Base (KB) associated with it. A KB is an ASCII text file containing a log of information relevant to the scan performed and results found. | [optional]
**hasaudittrail** | Option<**bool**> | Indicates whether the scan is configured to create an audit trail. | [optional]
**scanner_start** | Option<**String**> | The scan's start time, if the scan is imported. | [optional]
**scanner_end** | Option<**String**> | The scan's end time, if the scan is imported. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


