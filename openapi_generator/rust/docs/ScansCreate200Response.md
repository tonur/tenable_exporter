# ScansCreate200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag_type** | Option<**String**> | The type of tag. | [optional]
**container_id** | Option<**String**> | The unique ID of your Tenable.io instance. | [optional]
**owner_uuid** | Option<**String**> | The unique ID of the scan owner. | [optional]
**uuid** | Option<**String**> | The UUID of the schedule for the scan. | [optional]
**name** | Option<**String**> | The user-defined scan name. | [optional]
**description** | Option<**String**> | A brief user-defined description of the scan. | [optional]
**policy_id** | Option<**i32**> | The unique ID of the policy associated with the scan. | [optional]
**scanner_uuid** | Option<**String**> | The UUID of the scanner that the scan is configured to use, if the scan is *not* configured for [scan routing](doc:manage-scan-routing-tio). | [optional]
**target_network_uuid** | Option<**String**> | The UUID of the network object that Tenable.io associates with the scan results if the scan is configured for [scan routing](doc:manage-scan-routing-tio). | [optional]
**emails** | Option<**String**> | A comma-separated list of accounts that receive the email summary report. | [optional]
**sms** | Option<**String**> | A comma-separated list of mobile phone numbers that receive notification of the scan. | [optional]
**enabled** | Option<**bool**> | A value indicating whether the scan schedule is active (`true`) or inactive (`false`). | [optional]
**dashboard_file** | Option<**String**> | The name of the dashboard file associated with the scan. | [optional]
**remediation** | Option<**i32**> | If `1`, your vulnerability remediation actions on scan targets have been successful. | [optional]
**include_aggregate** | Option<**bool**> | A value indicating whether the scan results appear in dashboards. | [optional]
**scan_time_window** | Option<**String**> | Depends on the type of scan: <ul><li>For Nessus agent scans, `scan_time_window` is the time frame, in minutes, during which agents must transmit scan results to Tenable.io in order to be included in dashboards and reports. If your request omits this parameter, the default value is 180 minutes.</li><li>For Nessus scanner scans, `scan_time_window` is the time frame, in minutes, after which the scan will automatically stop. If your request omits this parameter, the default value is 0 and the scan will not stop after a certain time frame.</li></ul> | [optional]
**custom_targets** | Option<**String**> | Targets specified in the `alt_targets` parameter of the [POST /scans/{scan_id}/launch](/reference#scans-launch) request body used to run the scan. | [optional]
**starttime** | Option<**String**> | For one-time scans, the starting time and date for the scan. For recurrent scans, the first date on which the scan schedule is active and the time that recurring scans launch based on the `rrules` parameter.  This attribute has the following format: `YYYYMMDDTHHMMSS`. | [optional]
**rrules** | Option<**String**> | The interval at which the scan repeats. The interval is formatted as a string of three values delimited by semi-colons. These values are: the frequency (FREQ=ONETIME or DAILY or WEEKLY or MONTHLY or YEARLY), the interval (INTERVAL=1 or 2 or 3 ... x), and the days of the week (BYDAY=SU,MO,TU,WE,TH,FR,SA). For a scan that runs every three weeks on Monday Wednesday and Friday, the string would be `FREQ=WEEKLY;INTERVAL=3;BYDAY=MO,WE,FR`. If the scan is not scheduled to recur, this attribute is `null`. For more information, see [rrules Format](doc:example-assessment-scan-recurring#rrules-format).  **Note:** To set the `rrules` parameter for an agent scan, the request must also include the following body parameters:<ul><li>The `uuid` parameter must specify an agent scan template. For more information, see [Tenable-Provided Agent Templates](https://docs.tenable.com/vulnerability-management/Content/Scans/AgentTemplates.htm) and the [GET /editor/scan/templates](ref:editor-list-templates) endpoint.</li><li>The `agent_group_id` parameter must specify an agent group. For more information, see [Agent Groups](ref:agent-groups).</li></ul>For an example request body for an agent scan, see [Example Agent Scan: Recurring](doc:example-agent-scan-recurring). | [optional]
**timezone** | Option<**String**> | The timezone of the scheduled start time for the scan. | [optional]
**notification_filters** | Option<[**Vec<crate::models::ScansCreate200ResponseNotificationFiltersInner>**](scans_create_200_response_notification_filters_inner.md)> | A list of filters that Tenable.io applies to determine whether it sends a notification email on scan completion to the recipients specified in the `emails` attribute. | [optional]
**tag_targets** | Option<**Vec<String>**> | The list of asset tag identifiers the scan uses to determine which assets it evaluates. For more information about tag-based scans, see [Manage Tag-Based Scans](doc:manage-tag-based-scans-tio). | [optional]
**shared** | Option<**bool**> | If `1`, the scan is shared with users other than the scan owner. The level of sharing is specified in the `acls` attribute of the scan details. | [optional]
**user_permissions** | Option<**i32**> | The sharing permissions for the scan. | [optional]
**default_permissions** | Option<**i32**> | The default permissions for the scan. | [optional]
**owner** | Option<**String**> | The owner of the scan. | [optional]
**owner_id** | Option<**i32**> | The unique ID of the owner of the scan. | [optional]
**last_modification_date** | Option<**i32**> | For newly-created scans, the date on which the scan configuration was created. For scans that have been launched at least once, this attribute does not represent the date on which the scan configuration was last modified. Instead, it represents the date on which the scan was last launched, in Unix time format. Tenable.io updates this attribute each time the scan launches. | [optional]
**creation_date** | Option<**i32**> | For newly-created scans, the date on which the scan configuration was originally created. For scans that have been launched at least once, this attribute does not represent the date on which the scan configuration was originally created. Instead, it represents the date on which the scan was first launched, in Unix time format. | [optional]
**r#type** | Option<**String**> | The type of scan. | [optional]
**id** | Option<**i32**> | The unique ID of the scan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


