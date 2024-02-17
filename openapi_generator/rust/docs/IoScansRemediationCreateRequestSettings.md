# IoScansRemediationCreateRequestSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the scan. | 
**description** | Option<**String**> | The description of the scan. | [optional]
**scanner_id** | Option<**String**> | The unique ID of the scanner to use. Use the [GET /scanners](ref:scanners-list) endpoint to find the scanner ID. You can use the special value `AUTO-ROUTED` to assign scan targets to scanner groups based on the groups' configured scan routes. For more information, see [Manage Scan Routing](doc:manage-scan-routing-tio).  **Note:** If you omit this parameter from the request, Tenable.io assigns the US Cloud Scanner by default. | [optional]
**target_network_uuid** | Option<**String**> | For remediation scans, enter a valid target network UUID from a previous scan you wish to remediate. | [optional]
**scan_time_window** | Option<**i32**> | This parameter depends on the type of scan: <ul><li>For Nessus agent scans, `scan_time_window` is the time frame, in minutes, during which agents must transmit scan results to Tenable.io in order to be included in dashboards and reports. If your request omits this parameter, the default value is 180 minutes.</li><li>For Nessus scanner scans, `scan_time_window` is the time frame, in minutes, after which the scan will automatically stop. If your request omits this parameter, the default value is 0 and the scan will not stop after a certain time frame.</li></ul> | [optional]
**text_targets** | Option<**String**> | The list of targets to scan. For a full list of supported target formats, see the [Tenable Vulnerability Management User Guide](https://docs.tenable.com/vulnerability-management/Content/Scans/AboutScanTargets.htm). You can specify multiple targets (of differing formats) as a comma-delimited list.   This parameter is required if your request omits other target parameters. For more information, see \"Required Scan Target Parameters\" in [Create a Scan](doc:create-scan-tio).    For remediation scans, enter a valid target from a previous scan you wish to remediate.   **Note:** Tenable.io does not perform validation on values you submit for this parameter. If you submit invalid values, Tenable.io stores the invalid values in the scan configuration, and when the scan runs, scanning fails on the invalid targets. | [optional]
**target_groups** | Option<**Vec<i32>**> | For remediation scans, enter a valid target group ID from a previous scan you wish to remediate. | [optional]
**file_targets** | Option<**String**> | The name of a file containing the list of targets to scan. Before you use this parameter, use the [POST /files/upload](ref:file-upload) endpoint to upload the file to Tenable.io; then, use the `fileuploaded` attribute of the response message as the `file_targets` parameter value.  This parameter is required if your request omits other target parameters. For more information, see \"Required Scan Target Parameters\" in [Create a Scan](doc:create-scan-tio).   **Note:** Unicode/UTF-8 encoding is not supported in the targets file. | [optional]
**tag_targets** | Option<**Vec<String>**> | The list of asset tag identifiers that the scan uses to determine which assets it evaluates. For more information about tag-based scans, see [Manage Tag-Based Scans](doc:manage-tag-based-scans-tio).  This parameter is required if your request omits other target parameters. For more information, see \"Required Scan Target Parameters\" in [Create a Scan](doc:create-scan-tio). | [optional]
**agent_group_id** | Option<**Vec<String>**> | An array of agent group UUIDs to scan. Required if the scan is an agent scan. | [optional]
**emails** | Option<**String**> | A comma-separated list of accounts that receive the email summary report. | [optional]
**acls** | Option<[**Vec<crate::models::ScansCreateRequestSettingsAclsInner>**](scans_create_request_settings_acls_inner.md)> | An array containing permissions to apply to the scan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


