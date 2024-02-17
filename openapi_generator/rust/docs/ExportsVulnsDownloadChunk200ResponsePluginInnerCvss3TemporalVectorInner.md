# ExportsVulnsDownloadChunk200ResponsePluginInnerCvss3TemporalVectorInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exploitability** | Option<**String**> | The CVSSv3 Exploit Maturity Code (E) for the vulnerability the plugin covers. Possible values include:   - Unproven—Corresponds to the Unproven (U) value for the E metric  - Proof-of-concept—Corresponds to the Proof-of-Concept (POC) value for the E metric  - Functional—Corresponds to the Functional (F) value for the E metric  - High—Corresponds to the High (H) value for the E metric  - Not-defined—Corresponds to the Not Defined (ND) value for the E metric | [optional]
**remediation_level** | Option<**String**> | The CVSSv3 Remediation Level (RL) temporal metric for the vulnerability the plugin covers. Possible values include:   - O—Official Fix  - T—Temporary Fix  - W—Workaround  - U—Unavailable  - X—Not Defined | [optional]
**report_confidence** | Option<**String**> | The CVSSv3 Report Confidence (RC) temporal metric for the vulnerability the plugin covers. Possible values include:   - U—Unknown  - R—Reasonable  - C—Confirmed  - X—Not Defined | [optional]
**raw** | Option<**String**> | The complete `cvss3_temporal_vector` metrics and result values for the vulnerability the plugin covers in a condensed and coded format. For example, `E:U/RL:OF/RC:C`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


