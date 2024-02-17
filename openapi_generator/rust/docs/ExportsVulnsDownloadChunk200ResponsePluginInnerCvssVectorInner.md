# ExportsVulnsDownloadChunk200ResponsePluginInnerCvssVectorInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_vector** | Option<**String**> | The CVSSv2 Access Vector (AV) metric for the vulnerability the plugin covers. Possible values include:   - L—Local  - A—Adjacent Network  - N—Network | [optional]
**access_complexity** | Option<**String**> | The CVSSv2 Access Complexity (AC) metric for the vulnerability the plugin covers. Possible values include:  - H—High  - M—Medium  - L—Low | [optional]
**authentication** | Option<**String**> | The CVSSv2 Authentication (Au) metric for the vulnerability the plugin covers. Possible values include:   - N—None  - S—Single  - M—Multiple | [optional]
**confidentiality_impact** | Option<**String**> | The CVSSv2 confidentiality impact metric for the vulnerability the plugin covers. Possible values include:   - N—None  - P—Partial  - C—Complete | [optional]
**integrity_impact** | Option<**String**> | The CVSSv2 integrity impact metric for the vulnerability the plugin covers. Possible values include:   - N—None  - P—Partial  - C—Complete | [optional]
**availability_impact** | Option<**String**> | The CVSSv2 availability impact metric for the vulnerability the plugin covers. Possible values include:   - N—None  - P—Partial  - C—Complete | [optional]
**raw** | Option<**String**> | The complete `cvss_vector` metrics and result values for the vulnerability the plugin covers in a condensed and coded format. For example, `AV:N/AC:M/Au:N/C:C/I:C/A:C`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


