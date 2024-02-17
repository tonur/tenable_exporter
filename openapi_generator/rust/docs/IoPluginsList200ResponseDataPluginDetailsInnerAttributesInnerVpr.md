# IoPluginsList200ResponseDataPluginDetailsInnerAttributesInnerVpr

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**score** | Option<**f32**> | The Vulnerability Priority Rating (VPR) for the vulnerability. If a plugin is designed to detect multiple vulnerabilities, the VPR represents the highest value calculated for a vulnerability associated with the plugin. For more information, see <a href=\"https://docs.tenable.com/cloud/Content/Analysis/RiskMetrics.htm\" target=\"_blank\">Severity vs. VPR</a> in the <i>Tenable Vulnerability Management User Guide</i>. | [optional]
**drivers** | Option<[**serde_json::Value**](.md)> | The key drivers Tenable uses to calculate a vulnerability's VPR. For more information, see <a href=\"/docs/vpr-drivers-tio\">Vulnerability Priority Rating Drivers</a>. | [optional]
**updated** | Option<**String**> | The ISO timestamp when Tenable Vulnerability Management last imported the VPR for this vulnerability. Vulnerability Management imports updated VPR values every time you run a scan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


