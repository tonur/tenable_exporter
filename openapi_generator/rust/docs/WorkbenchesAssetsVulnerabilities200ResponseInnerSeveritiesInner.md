# WorkbenchesAssetsVulnerabilities200ResponseInnerSeveritiesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The number of vulnerabilities with the specified severity. | [optional]
**level** | Option<**i32**> | The code for the severity. Possible values include:   - 0—The vulnerability has a CVSS score of 0, which corresponds to the \"info\" severity level.  - 1—The vulnerability has a CVSS score between 0.1 and 3.9, which corresponds to the \"low\" severity level.  - 2—The vulnerability has a CVSS score between 4.0 and 6.9, which corresponds to the \"medium\" severity level.  - 3—The vulnerability has a CVSS score between 7.0 and 9.9, which corresponds to the \"high\" severity level.  - 4—The vulnerability has a CVSS score of 10.0, which corresponds to the \"critical\" severity level. | [optional]
**name** | Option<**String**> | The severity of the vulnerability as defined using the Common Vulnerability Scoring System (CVSS) base score. Possible values include `info` (CVSS score of 0), `low` (CVSS score between 0.1 and 3.9), `medium` (CVSS score between 4.0 and 6.9), `high` (CVSS score between 7.0 and 9.9), and `critical` (CVSS score of 10.0). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


