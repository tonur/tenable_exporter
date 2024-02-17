# WorkbenchesAssets200ResponseAssetsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The UUID of the asset. | [optional]
**has_agent** | Option<**bool**> | A value specifying whether a Nessus agent scan detected the asset (`true`). | [optional]
**last_seen** | Option<**String**> | The ISO timestamp of the scan that most recently detected the asset. | [optional]
**last_scan_target** | Option<**String**> | The IPv4 address, IPv6 address, or FQDN that the scanner last used to evaluate the asset. | [optional]
**sources** | Option<[**Vec<crate::models::AssetsListAssets200ResponseAssetsInnerSourcesInnerInner>**](assets_list_assets_200_response_assets_inner_sources_inner_inner.md)> | A list of sources for the asset record. | [optional]
**acr_score** | Option<**i32**> | The Asset Criticality Rating (ACR) for the asset. Tenable assigns an ACR to each asset on your network to represent the asset's relative risk as an integer from 1 to 10. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license. | [optional]
**acr_drivers** | Option<[**Vec<crate::models::AssetsListAssets200ResponseAssetsInnerAcrDriversInner>**](assets_list_assets_200_response_assets_inner_acr_drivers_inner.md)> | The key drivers that Tenable uses to calculate an asset's Tenable-provided ACR. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license. | [optional]
**exposure_score** | Option<**i32**> | The Asset Exposure Score (AES) for the asset. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license. | [optional]
**scan_frequency** | Option<[**Vec<crate::models::AssetsListAssets200ResponseAssetsInnerScanFrequencyInner>**](assets_list_assets_200_response_assets_inner_scan_frequency_inner.md)> | Information about how often scans ran against the asset during specified intervals. For more information, see [Lumin Metrics](https://docs.tenable.com/vulnerability-management/Content/Lumin/LuminMetrics.htm) in the *Tenable Vulnerability Management User Guide*.  This attribute is only present if you have a Lumin license. | [optional]
**ipv4** | Option<**Vec<String>**> | A list of ipv4 addresses for the asset. | [optional]
**ipv6** | Option<**Vec<String>**> | A list of ipv6 addresses for the asset. | [optional]
**fqdn** | Option<**Vec<String>**> | A list of fully-qualified domain names (FQDNs) for the asset. | [optional]
**netbios_name** | Option<**Vec<String>**> | The NetBIOS name for the asset. | [optional]
**operating_system** | Option<**Vec<String>**> | The operating system installed on the asset. | [optional]
**agent_name** | Option<**Vec<String>**> | The names of any Nessus agents that scanned and identified the asset. | [optional]
**aws_ec2_name** | Option<**Vec<String>**> | The name of the virtual machine instance in AWS EC2. | [optional]
**mac_address** | Option<**Vec<String>**> | A list of MAC addresses for the asset. | [optional]
**bigfix_asset_id** | Option<**Vec<String>**> | The unique identifier of the asset in HCL BigFix. For more information, see the HCL BigFix documentation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


