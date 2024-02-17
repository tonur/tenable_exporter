# IoScansRemediationList200ResponseScansInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**control** | Option<**bool**> | If `true`, the scan has a schedule and can be launched. | [optional]
**creation_date** | Option<**i32**> | For newly-created scans, the date on which the scan configuration was originally created. For scans that have been launched at least once, this attribute does not represent the date on which the scan configuration was originally created. Instead, it represents the date on which the scan was first launched, in Unix time format. | [optional]
**enabled** | Option<**bool**> | Indicates whether the scan schedule is active (`true`) or inactive (`false`). | [optional]
**id** | Option<**i32**> | The unique ID of the scan. | [optional]
**last_modification_date** | Option<**i32**> | For newly-created scans, the date on which the scan configuration was created. For scans that have been launched at least once, this attribute does not represent the date on which the scan configuration was last modified. Instead, it represents the date on which the scan was last launched, in Unix time format. Tenable.io updates this attribute each time the scan launches. | [optional]
**name** | Option<**String**> | The name of the scan. | [optional]
**owner** | Option<**String**> | The owner of the scan. | [optional]
**policy_id** | Option<**i32**> | The unique ID of the user-defined template (policy) on which the scan configuration is based. | [optional]
**read** | Option<**bool**> | A value indicating whether the user account associated with the request message has viewed the scan in the Tenable.io user interface. If `1`, the user account has viewed the scan results. | [optional]
**schedule_uuid** | Option<**String**> | The UUID for a specific instance in the scan schedule. | [optional]
**shared** | Option<**bool**> | If `true`, the scan is shared with users other than the scan owner. The level of sharing is specified in the `acls` attribute of the scan details. | [optional]
**status** | Option<**String**> | The status of the scan. For a list of possible values, see [Scan Status](doc:scan-status-tio). | [optional]
**template_uuid** | Option<**String**> | The UUID of the template. | [optional]
**r#type** | Option<**String**> | The type of scan. | [optional]
**permissions** | Option<**i32**> | The requesting user's permissions for the scan. | [optional]
**user_permissions** | Option<**i32**> | The sharing permissions for the scan. | [optional]
**uuid** | Option<**String**> | The UUID of the remediation scan. | [optional]
**wizard_uuid** | Option<**String**> | The UUID of the Tenable-provided template used to create either the scan or the user-defined template (policy) on which the scan configuration is based. | [optional]
**progress** | Option<**i32**> | The progress of the scan ranging from 0 to 100. | [optional]
**scan_creation_date** | Option<**i32**> | The Unix timestamp when the remediation scan run was created. | [optional]
**remediation** | Option<**i32**> | If `1`, your vulnerability remediation actions on scan targets have been successful. | [optional]
**total_targets** | Option<**i32**> | The total number of targets in the scan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


