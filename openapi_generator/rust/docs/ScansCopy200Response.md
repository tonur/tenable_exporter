# ScansCopy200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the scan. | [optional]
**status** | Option<**String**> | The status of the scan. For a list of possible values, see [Scan Status](doc:scan-status-tio). | [optional]
**id** | Option<**i32**> | The unique ID of the scan. | [optional]
**last_modification_date** | Option<**i32**> | For newly-created scans, the date on which the scan configuration was created. For scans that have been launched at least once, this attribute does not represent the date on which the scan configuration was last modified. Instead, it represents the date on which the scan was last launched, in Unix time format. Tenable.io updates this attribute each time the scan launches. | [optional]
**uuid** | Option<**String**> | The UUID of the scan. | [optional]
**r#type** | Option<**String**> | The type of scan (local, remote, or agent). | [optional]
**owner** | Option<**String**> | The owner of the scan. | [optional]
**enabled** | Option<**bool**> | If `true`, the schedule for the scan is enabled. | [optional]
**read** | Option<**bool**> | If `true`, the scan has been read. | [optional]
**shared** | Option<**bool**> | If `1`, the scan is shared with users other than the scan owner. The level of sharing is specified in the `acls` attribute of the scan details. | [optional]
**user_permissions** | Option<**i32**> | The sharing permissions for the scan. | [optional]
**creation_date** | Option<**i32**> | The creation date for the scan in Unix time. | [optional]
**control** | Option<**bool**> | If `true`, the scan has a schedule and can be launched. | [optional]
**starttime** | Option<**String**> | For one-time scans, the starting time and date for the scan. For recurrent scans, the first date on which the scan schedule is active and the time that recurring scans launch based on the `rrules` attribute.  This attribute has the following format: `YYYYMMDDTHHMMSS`. | [optional]
**timezone** | Option<**String**> | The timezone of the scheduled start time for the scan. | [optional]
**rrules** | Option<**String**> | The interval at which the scan repeats. The interval is formatted as a string of three values delimited by semi-colons. These values are: the frequency (FREQ=ONETIME or DAILY or WEEKLY or MONTHLY or YEARLY), the interval (INTERVAL=1 or 2 or 3 ... x), and the days of the week (BYDAY=SU,MO,TU,WE,TH,FR,SA). For a scan that runs every three weeks on Monday Wednesday and Friday, the string would be `FREQ=WEEKLY;INTERVAL=3;BYDAY=MO,WE,FR`. If the scan is not scheduled to recur, this attribute is `null`. For more information, see [rrules Format](doc:example-assessment-scan-recurring#rrules-format).  **Note:** To set the `rrules` parameter for an agent scan, the request must also include the following body parameters:<ul><li>The `uuid` parameter must specify an agent scan template. For more information, see [Tenable-Provided Agent Templates](https://docs.tenable.com/vulnerability-management/Content/Scans/AgentTemplates.htm) and the [GET /editor/scan/templates](ref:editor-list-templates) endpoint.</li><li>The `agent_group_id` parameter must specify an agent group. For more information, see [Agent Groups](ref:agent-groups).</li></ul>For an example request body for an agent scan, see [Example Agent Scan: Recurring](doc:example-agent-scan-recurring). | [optional]
**schedule_uuid** | Option<**String**> | The UUID for a specific instance in the scan schedule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


