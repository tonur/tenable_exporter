# ScansLaunchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alt_targets** | Option<**Vec<String>**> | If you include this parameter, Tenable.io scans these targets instead of the default. Value can be an array where each index is a target, or an array with a single index of comma-separated targets. | [optional]
**rollover** | Option<**bool**> | Indicates whether or not to launch a rollover scan instead of full scan. A rollover scan only runs against the targets that Tenable.io did not scan due to a previous scan timeout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


