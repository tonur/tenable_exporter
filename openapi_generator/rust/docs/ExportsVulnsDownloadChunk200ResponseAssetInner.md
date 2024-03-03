# ExportsVulnsDownloadChunk200ResponseAssetInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_uuid** | Option<**String**> | The UUID of the agent that performed the scan where the vulnerability was found. | [optional]
**bios_uuid** | Option<**String**> | The BIOS UUID of the asset where the vulnerability was found. | [optional]
**device_type** | Option<**String**> | The type of asset where the vulnerability was found. | [optional]
**fqdn** | Option<**String**> | The fully-qualified domain name of the asset where a scan found the vulnerability. | [optional]
**hostname** | Option<**String**> | The host name of the asset where a scan found the vulnerability. | [optional]
**uuid** | Option<**String**> | The UUID of the asset where a scan found the vulnerability. | [optional]
**ipv4** | Option<**String**> | The IPv4 address of the asset where a scan found the vulnerability. | [optional]
**ipv6** | Option<**String**> | The IPv6 address of the asset where a scan found the vulnerability. | [optional]
**last_authenticated_results** | Option<**String**> | The last date credentials were used successfully to scan the asset. | [optional]
**last_unauthenticated_results** | Option<**String**> | The last date when the asset was scanned without using credentials | [optional]
**mac_address** | Option<**String**> | The MAC address of the asset where a scan found the vulnerability. | [optional]
**netbios_name** | Option<**String**> | The NETBIOS name of the asset where a scan found the vulnerability. | [optional]
**netbios_workgroup** | Option<**String**> | The NETBIOS workgroup of the asset where a scan found the vulnerability. | [optional]
**operating_system** | Option<**String**> | The operating system of the asset where a scan found the vulnerability. | [optional]
**network_id** | Option<**String**> | The ID of the network object associated with scanners that identified the asset. The default network ID is `00000000-0000-0000-0000-000000000000`. For more information about network objects, see [Manage Networks](doc:manage-networks-tio). | [optional]
**tracked** | Option<**bool**> | A value specifying whether Tenable Vulnerability Management tracks the asset in the asset management system. Tenable Vulnerability Management still assigns untracked assets identifiers in scan results, but these identifiers change with each new scan of the asset. This parameter is relevant to PCI-type scans and in certain cases where there is not enough information in a scan to identify the asset. Untracked assets appear in the scan history, but do not appear in workbenches or reports. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


