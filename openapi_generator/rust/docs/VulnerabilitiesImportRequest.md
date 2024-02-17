# VulnerabilitiesImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | **String** | The source of the scan that generated the vulnerability data. If you want to categorize the imported vulnerabilities in the same way that Tenable.io categorizes vulnerabilities detected in scans it manages, use the following values:   - security_center—A Nessus scan identified the vulnerabilities you want to import. Use this value for all Nessus scans, regardless of the scan manager (Tenable.io, SecurityCenter, or Nessus Manager). | 
**r#type** | **String** | The type of scan that identified the vulnerabilities you want to import. If you want to categorize the imported vulnerabilities in the same way that Tenable.io categorizes vulnerabilities detected in scans it manages, use the following values:   - vm—A Vulnerability Management scan identified the vulnerabilities.  - was—A Web Application Scanning scan identified the vulnerabilities.  - pc—A scan of a personal computer identified the vulnerabilities. | 
**assets** | [**Vec<crate::models::VulnerabilitiesImportRequestAssetsInner>**](vulnerabilities_import_request_assets_inner.md) | An array of asset objects with vulnerabilities information. A valid asset record requires at least one valid network_interface object.  **Note:** Tenable.io supports a maximum of 50 individual asset objects per request message. In addition, because Tenable.io supports a total size limit of 15 MB for the request message, you may want to limit the number of asset objects you include in an individual request, depending on the number of vulnerabilities identified on the assets and the size of the related vulnerability output.  **Note:** This endpoint does not support the network_id attribute in asset objects for import. Tenable.io automatically assigns imported assets to the default network object. For more information about network objects, see [Manage Networks](doc:manage-networks-tio). | 
**checks_ran** | Option<[**Vec<crate::models::VulnerabilitiesImportRequestChecksRanInner>**](vulnerabilities_import_request_checks_ran_inner.md)> | An array of objects, each representing a check that the scan used to detect the vulnerabilities you are importing. This parameter supports Tenable plugin checks only. For more information, see [Plugins](https://www.tenable.com/plugins). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


