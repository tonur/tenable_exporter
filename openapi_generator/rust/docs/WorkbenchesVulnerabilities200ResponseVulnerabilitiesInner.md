# WorkbenchesVulnerabilities200ResponseVulnerabilitiesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The number of times that a scan detected the vulnerability on an asset. | [optional]
**plugin_family** | Option<**String**> | The plugin's family. | [optional]
**plugin_id** | Option<**i32**> | The unique plugin ID. | [optional]
**plugin_name** | Option<**String**> | The name of the plugin that detected the vulnerability. | [optional]
**vulnerability_state** | Option<**String**> | The current state of the reported plugin. Possible states include:  - Active—The vulnerability is currently present on an asset.  - New—The vulnerability is active on an asset, and was first detected within the last 14 days.  - Fixed—A subsequent scan detected that the formerly-active vulnerability is no longer present on an asset.  - Resurfaced—The vulnerability was previously marked as fixed on an asset, but a subsequent scan detected the vulnerability on the asset again. | [optional]
**vpr_score** | Option<**f32**> | The Vulnerability Priority Rating (VPR) for the vulnerability. If a plugin is designed to detect multiple vulnerabilities, the VPR represents the highest value calculated for a vulnerability associated with the plugin. For more information, see <a href=\"https://docs.tenable.com/vulnerability-management/Content/Analysis/RiskMetrics.htm\" target=\"_blank\">Severity vs. VPR</a> in the <i>Tenable Vulnerability Management User Guide</i>. | [optional]
**accepted_count** | Option<**i32**> | The number of times that a user in the user interface created an accept rule for this vulnerability. For more information, see <a href=\"https://docs.tenable.com/vulnerability-management/Content/Platform/Settings/Recast/AboutRecastRules.htm\" target=\"_blank\">Recast Rules</a> in the <i>Tenable Vulnerability Management User Guide</i>. | [optional]
**recasted_count** | Option<**i32**> | The number of times that a user in the user interface created a recast rule for this vulnerability. For more information, see <a href=\"https://docs.tenable.com/vulnerability-management/Content/Platform/Settings/Recast/AboutRecastRules.htm\" target=\"_blank\">Recast Rules</a> in the <i>Tenable Vulnerability Management User Guide</i>. | [optional]
**counts_by_severity** | Option<[**Vec<crate::models::WorkbenchesVulnerabilities200ResponseVulnerabilitiesInnerCountsBySeverityInner>**](workbenches_vulnerabilities_200_response_vulnerabilities_inner_counts_by_severity_inner.md)> | The number of times that a scan detected the vulnerability on an asset, grouped by severity level. | [optional]
**severity** | Option<**i32**> | The severity level of the vulnerability, as defined using the Common Vulnerability Scoring System (CVSS) base score. Possible values include:   - 0—The vulnerability has a CVSS score of 0, which corresponds to the \"info\" severity level.  - 1—The vulnerability has a CVSS score between 0.1 and 3.9, which corresponds to the \"low\" severity level.  - 2—The vulnerability has a CVSS score between 4.0 and 6.9, which corresponds to the \"medium\" severity level.  - 3—The vulnerability has a CVSS score between 7.0 and 9.9, which corresponds to the \"high\" severity level.  - 4—The vulnerability has a CVSS score of 10.0, which corresponds to the \"critical\" severity level. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


