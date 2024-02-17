# VulnerabilitiesImportV2RequestAssetsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_interfaces** | Option<[**Vec<crate::models::VulnerabilitiesImportRequestAssetsInnerNetworkInterfacesInner>**](vulnerabilities_import_request_assets_inner_network_interfaces_inner.md)> | A valid network_interface object must contain at least one of the following parameters: `ipv4`, `netbios_name`, `fqdn`. | [optional]
**hostname** | Option<**String**> | The asset's hostname. | [optional]
**servicenow_sysid** | Option<**String**> | The unique record identifier of the asset in ServiceNow. For more information, see the ServiceNow documentation. | [optional]
**ssh_fingerprint** | Option<**String**> | The SSH key fingerprint that the scan has associated with the asset. | [optional]
**bios_uuid** | Option<**String**> | The BIOS UUID of the asset. | [optional]
**netbios_name** | Option<**String**> | The NetBIOS name that the scan has associated with the asset. | [optional]
**operating_systems** | Option<**String**> | The operating system the asset is running. | [optional]
**authenticated** | Option<**bool**> | Specifies that the asset has been scanned with credentials for OS or application authentication. | [optional]
**tenable_agent_id** | Option<**String**> | The unique ID of the Nessus agent installed on the asset. This parameter is supported only if the `vendor` parameter for the request is `tenable`. | [optional]
**tenable_network_id** | Option<**String**> | The unique identifier for the [network](doc:manage-networks-tio) where Tenable.io assigns the asset during import. | [optional]
**vulnerabilities** | Option<[**Vec<crate::models::VulnerabilitiesImportV2RequestAssetsInnerVulnerabilitiesInner>**](vulnerabilities_import_v2_request_assets_inner_vulnerabilities_inner.md)> | A valid vulnerability object must contain at least one of the following parameters: `tenable_plugin_id` or `cve`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


