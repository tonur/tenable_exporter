# EditorDetails200ResponseSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**basic** | Option<[**serde_json::Value**](.md)> | The Basic scan settings are used to specify certain organizational and security-related aspects of the scan or policy, including the name of the scan, its targets, whether the scan is scheduled, and who has access to the scan, among other settings. | [optional]
**discovery** | Option<[**serde_json::Value**](.md)> | The Discovery settings relate to discovery and port scanning, including port ranges and methods. | [optional]
**assessment** | Option<[**serde_json::Value**](.md)> | You can use Assessment settings to configure how a scan identifies vulnerabilities, as well as what vulnerabilities are identified. This includes identifying malware, assessing the vulnerability of a system to brute force attacks, and the susceptibility of web applications. | [optional]
**advanced** | Option<[**serde_json::Value**](.md)> | The Advanced settings provide increased control over scan efficiency and the operations of a scan, as well as the ability to enabled plugin debugging. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


