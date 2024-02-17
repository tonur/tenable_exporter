# IoExportsComplianceDownload200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_uuid** | Option<**String**> | The UUID of the asset on which the compliance check was executed. | [optional]
**first_seen** | Option<**i64**> | The Unix timestamp when a compliance scan first assessed the asset with the compliance check. | [optional]
**last_seen** | Option<**i64**> | The Unix timestamp when a compliance scan last assessed the asset with the compliance check. | [optional]
**audit_file** | Option<**String**> | The name of the audit file containing the compliance check. | [optional]
**check_id** | Option<**String**> | The unique identifier for the compliance finding. | [optional]
**check_name** | Option<**String**> | The descriptive name of the compliance check. | [optional]
**check_info** | Option<**String**> | Full text description of the compliance check. | [optional]
**expected_value** | Option<**String**> | The desired value (integer or string) for the compliance check. For example, if a password length compliance check requires passwords to be 8 characters long then `8` is the expected value. For manual checks, this field will contain the command used for the compliance check. | [optional]
**actual_value** | Option<**String**> | The actual value (integer, string, or table) evaluated from the compliance check. For example, if a password length compliance check requires passwords to be 8 characters long, but the evaluated value was 7 then `7` is the actual value. For manual checks, this field will contain the output of the command that was executed. | [optional]
**status** | Option<**String**> | Indicates the result of the compliance check for the given asset.   Possible values include:  - PASSED—Returned if the asset has passed the compliance check.  - FAILED—Returned if the asset has failed the compliance check.  - WARNING—Returned in cases where there is no definable passing criteria; for example, an audit where you must verify that members of the administrator group are appropriate for your organization.  - SKIPPED—Returned if the plugin determines that the check is not applicable to the asset. It can also be returned in other various cases; for example, when a check requires that a direct command be run to gather data on an offline network device or if a check contains commands that won't run on the specified OS.  - UNKNOWN—Returned when a status cannot be determined for the OVAL check. This status is set by the OVAL engine. | [optional]
**reference** | Option<[**Vec<crate::models::IoExportsComplianceDownload200ResponseInnerReferenceInner>**](io_exports_compliance_download_200_response_inner_reference_inner.md)> | Industry references for the compliance check. | [optional]
**see_also** | Option<**String**> | Links to external websites that contain reference information about the compliance check. | [optional]
**solution** | Option<**String**> | Remediation information for the compliance check. | [optional]
**check_error** | Option<**String**> | An error message if the compliance evaluation fails. | [optional]
**profile_name** | Option<**String**> | The name of the profile for the benchmark standard. | [optional]
**db_type** | Option<**String**> | The type of database if the compliance check assessed a database. | [optional]
**plugin_id** | Option<**i32**> | The unique ID of the compliance plugin. | [optional]
**state** | Option<**String**> | Indicates whether or not the finding applies to the asset based on the last assessment. This field is `NULL` for findings last seen before December 2021. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


