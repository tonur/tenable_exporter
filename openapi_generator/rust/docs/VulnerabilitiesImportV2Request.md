# VulnerabilitiesImportV2Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor** | **String** | The company that owns the product that is the source of the vulnerability data. To categorize the imported vulnerabilities in the same way that Tenable.io categorizes vulnerabilities detected in scans it manages, use the following values:   - tenable—A Nessus scan identified the vulnerabilities you want to import. Use this value for all Nessus scans, regardless of the scan manager (Tenable.io, Tenable.sc, or Nessus Manager). | 
**product** | **String** | The name of the product from the vendor that is the source of the vulnerability data being imported.   - tenable.sc—The vulnerability data source is Tenable.sc. | 
**data_type** | **String** | The type of scan that identified the vulnerabilities you want to import. To categorize the imported vulnerabilities in the same way that Tenable.io categorizes vulnerabilities detected in scans it manages, use the following values:   - vm—A Vulnerability Management scan identified the vulnerabilities. | 
**source** | **String** | A unique string value used to track the set of assets and vulnerabilities that Tenable.io is importing and processing. For data imported from Tenable.sc via [Lumin synchronization](https://docs.tenable.com/security-center/Content/LuminSynchronization.htm), this value has the following format: scan_uuid:scan_chunk_uuid  where scan_uuid is the unique identifier for the scan in Tenable.sc (equivalent to the scan id used in [Tenable.sc API requests](https://docs.tenable.com/security-center/api/Scan.html)), and scan_chunk_uuid is the unique identifer that Tenable.sc assigns to individual chunks of scan data during the Lumin synchronization process. | 
**assets** | [**Vec<crate::models::VulnerabilitiesImportV2RequestAssetsInner>**](vulnerabilities_import_v2_request_assets_inner.md) | An array of asset objects with vulnerabilities information. A valid asset record requires at least one valid network_interface object.  **Note:** Tenable.io supports a maximum of 50 individual asset objects per request message. In addition, because Tenable.io supports a total size limit of 15 MB for the request message, you may want to limit the number of asset objects you include in an individual request, depending on the number of vulnerabilities identified on the assets and the size of the related vulnerability output.  **Note:** This endpoint does not support the network_id attribute in asset objects for import. Tenable.io automatically assigns imported assets to the default network object. For more information about network objects, see [Manage Networks](doc:manage-networks-tio). | 
**coverage** | Option<[**crate::models::VulnerabilitiesImportV2RequestCoverage**](vulnerabilities_import_v2_request_coverage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


