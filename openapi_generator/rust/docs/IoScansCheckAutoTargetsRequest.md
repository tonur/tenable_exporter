# IoScansCheckAutoTargetsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_uuid** | **String** | Specify a value as follows:<ul><li>If your scans involve separate environments with overlapping IP ranges, specify the UUID of the [network](doc:manage-networks-tio) you want to associate with the results of the auto-routed scan. This value must match the network where you have assigned the scanner groups that you configured for scan routing.</li><li>Otherwise, specify the default network (00000000-0000-0000-0000-000000000000).</li></ul> | 
**tags** | Option<**Vec<String>**> | A list of asset tags UUIDs. | [optional]
**target_list** | Option<**String**> | A comma-delimited list of targets to scan. For a full list of supported target formats, see the [Tenable Vulnerability Management User Guide](https://docs.tenable.com/vulnerability-management/Content/Scans/AboutScanTargets.htm). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


