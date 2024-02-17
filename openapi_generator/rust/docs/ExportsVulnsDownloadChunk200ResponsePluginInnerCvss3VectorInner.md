# ExportsVulnsDownloadChunk200ResponsePluginInnerCvss3VectorInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_vector** | Option<**String**> | The CVSSv2 Attack Vector (AV) metric for the vulnerability the plugin covers. Possible values include:  - Network—Corresponds to the Network (N) value for the AV metric.   - Adjacent Network—Corresponds to the Adjacent Network (A) value for the AV metric.   - Local—Corresponds to the Local (L) value for the AV metric | [optional]
**access_complexity** | Option<**String**> | The CVSSv3 Access Complexity (AC) metric for the vulnerability the plugin covers. Possible values include:   - H—High  - M—Medium  - L—Low | [optional]
**authentication** | Option<**String**> | The CVSSv2 Authentication (Au) metric for the vulnerability the plugin covers. Possible values include:  - None required—Corresponds to the None (N) value for the Au metric.   - Requires-single-instance—Corresponds to the Single (S) value for the Au metric.   - Requires-multiple-instances—Corresponds to the Multiple (M) value for the Au metric | [optional]
**confidentiality_impact** | Option<**String**> | The CVSSv3 confidentiality impact metric of the vulnerability the plugin covers to the vulnerable component. Possible values include:   - H—High  - L—Low  - N—None | [optional]
**integrity_impact** | Option<**String**> | The CVSSv3 integrity impact metric for the vulnerability the plugin covers. Possible values include:  - H—High  - L—Low  - N—None | [optional]
**availability_impact** | Option<**String**> | The CVSSv2 availability impact metric for the vulnerability the plugin covers. Possible values include:   - H—High  - L—Low  - N—None | [optional]
**raw** | Option<**String**> | The complete `cvss3_vector` metrics and result values for the vulnerability the plugin covers in a condensed and coded format. For example, `AV:N/AC:M/Au:N/C:C/I:C/A:C`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


