# ExportsVulnsDownloadChunk200ResponsePluginInnerCvssTemporalVectorInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exploitability** | Option<**String**> | The CVSSv2 Exploitability (E) temporal metric for the vulnerability the plugin covers. Possible values include:  - U—Unproven  - POC—Proof-of-Concept  - F—Functional  - H—High  - ND—Not Defined | [optional]
**remediation_level** | Option<**String**> | The CVSSv2 Remediation Level (RL) temporal metric for the vulnerability the plugin covers. Possible values include:   - OF—Official Fix  - TF—Temporary Fix  - W—Workaround  - U—Unavailable  - ND—Not Defined | [optional]
**report_confidence** | Option<**String**> | The CVSSv2 Report Confidence (RC) temporal metric for the vulnerability the plugin covers. Possible values include:   - UC—Unconfirmed  - UR—Uncorroborated  - C—Confirmed  - ND—Not Defined | [optional]
**raw** | Option<**String**> | The complete `cvss_temporal_vector` metrics and result values for the vulnerability the plugin covers in a condensed and coded format. For example, `E:U/RL:OF/RC:C`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


