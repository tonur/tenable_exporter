# ExportsVulnsExportStatus200ResponseStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<**String**> | The UUID for the export request. | [optional]
**status** | Option<**String**> | The status of the export request. Possible values include:  - QUEUED—Tenable Vulnerability Management has queued the export request until it completes other requests currently in process.  - PROCESSING—Tenable Vulnerability Management has started processing the export request.  - FINISHED—Tenable Vulnerability Management has completed processing the export request. The list of chunks is complete.  - CANCELLED—An administrator has cancelled the export request.  - ERROR—Tenable Vulnerability Management encountered an error while processing the export request. Tenable recommends that you retry the request. If the status persists on retry, contact Support. | [optional]
**chunks_available** | Option<**Vec<i32>**> | A list of completed chunks available for download. | [optional]
**chunks_failed** | Option<**Vec<i32>**> | A list of chunks for which the export process failed. If a chunk fails processing, submit the export request again. If the chunk continues to fail, contact Support. | [optional]
**chunks_cancelled** | Option<**Vec<i32>**> | A list of chunks for which the export process was cancelled. If a chunk fails processing, Tenable Vulnerability Management automatically cancels all subsequent chunks queued for export in the same request.  | [optional]
**total_chunks** | Option<**i32**> | The total number of chunks associated with the export job as a whole. | [optional]
**chunks_available_count** | Option<**i32**> | The total number of completed chunks available for download. | [optional]
**empty_chunks_count** | Option<**i32**> | The total number of empty chunks. | [optional]
**finished_chunks** | Option<**i32**> | The number of chunks that have been processed and are available for download. | [optional]
**filters** | Option<[**serde_json::Value**](.md)> | The filters used in the export job request. For a list of possible filters, see the [POST /vulns/export](ref:exports-vulns-export-request-export) and [POST /assets/export](ref:exports-assets-request-export) endpoints. | [optional]
**num_assets_per_chunk** | Option<**i32**> | The number of assets contained in each export chunk. | [optional]
**created** | Option<**i32**> | The Unix timestamp when the export job was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


