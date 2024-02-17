# IoExportsComplianceStatus200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The status of the compliance export request. Possible values include:  - PROCESSING—Tenable Vulnerability Management has started processing the compliance export request.  - FINISHED—Tenable Vulnerability Management has completed processing the export request, the list of chunks is complete, and all chunks are available for download.  - READY—Some chunks are now available for download, but Tenable Vulnerability Management is still processing the export request.  - CANCELLED—An administrator has canceled the export request.  - ERROR—Tenable Vulnerability Management encountered an error while processing the export request. Tenable recommends that you retry the request. If the status persists on retry, contact Support. | [optional]
**chunks_available** | Option<**Vec<i32>**> | A comma-separated list of completed chunks available for download. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


