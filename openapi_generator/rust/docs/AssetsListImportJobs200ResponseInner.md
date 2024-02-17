# AssetsListImportJobs200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | Option<**String**> | The UUID of the asset import job. | [optional]
**container_id** | Option<**String**> | The UUID of your Tenable.io container. | [optional]
**source** | Option<**String**> | A user-defined name for the source of the import containing the asset records. You can specify only one source for each import. | [optional]
**batches** | Option<**i32**> | The number of batches in the asset import job. | [optional]
**uploaded_assets** | Option<**i32**> | The number of assets from the import job that Tenable.io successfully imported. | [optional]
**failed_assets** | Option<**i32**> | The number of assets from the import job that Tenable.io failed to import. | [optional]
**start_time** | Option<**i32**> | The Unix timestamp when Tenable.io started processing the import job. | [optional]
**last_update_time** | Option<**i32**> | The Unix timestamp when Tenable.io performed an action on the import job. | [optional]
**end_time** | Option<**i32**> | The Unix timestamp when Tenable.io completed processing the import job. | [optional]
**status** | Option<**String**> | The status of the import job. Possible values include: COMPLETE, IN_PROGRESS, or ERROR. | [optional]
**status_message** | Option<**String**> | The description of why a job failed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


