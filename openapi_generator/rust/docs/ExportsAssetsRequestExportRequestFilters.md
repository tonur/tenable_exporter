# ExportsAssetsRequestExportRequestFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**i64**> | Returns all assets created later than the date specified. The specified date must be in the Unix timestamp format. | [optional]
**updated_at** | Option<**i64**> | Returns all assets updated later than the date specified. The specified date must be in the Unix timestamp format. | [optional]
**terminated_at** | Option<**i64**> | Returns all assets terminated later than the date specified. The specified date must be in the Unix timestamp format. | [optional]
**is_terminated** | Option<**bool**> | When set to `true`, returns assets which have any value for the `terminated_at` attribute. | [optional]
**deleted_at** | Option<**i64**> | Returns all assets deleted later than the date specified. The specified date must in the Unix timestamp format. | [optional]
**is_deleted** | Option<**bool**> | When set to `true`, returns assets which have any value for the `deleted_at` attribute. | [optional]
**is_licensed** | Option<**bool**> | Specifies whether the asset is included in the asset count for the Tenable Vulnerability Management instance. If `true`, Tenable Vulnerability Management returns only licensed assets. If `false`, Tenable Vulnerability Management returns only unlicensed assets. If this filter is omitted, Tenable Vulnerability Management returns both licensed and unlicensed assets. | [optional]
**first_scan_time** | Option<**i64**> | Returns all assets with a first scan time later than the date specified. The specified date must be in the Unix timestamp format. | [optional]
**last_authenticated_scan_time** | Option<**i64**> | Returns all assets with a last credentialed scan time later than the date specified. The specified date must be in the Unix timestamp format. | [optional]
**last_assessed** | Option<**i64**> | Returns all assets with a last assessed time later than the date specified. Tenable Vulnerability Management considers an asset assessed if it has been scanned by a credentialed or non-credentialed scan. The specified date must be in the Unix timestamp format. | [optional]
**servicenow_sysid** | Option<**bool**> | If `true`, returns all assets that have a ServiceNow Sys ID, regardless of value. If `false`, returns all assets that do not have a ServiceNow Sys ID. | [optional]
**sources** | Option<**Vec<String>**> | Returns assets that have the specified asset source. An asset source is the entity that reported the asset details. Sources can include sensors, connectors, and API imports. If your request specifies multiple sources, Tenable Vulnerability Management returns all assets seen by any of the specified sources.  The items in the `sources` array must correspond to the names of the sources as defined in your organization's implementation of Tenable Vulnerability Management. Commonly used names include:  - AWS—The asset data was obtained from an Amazon Web Services connector.  - NESSUS_AGENT—The asset data was obtained from a Tenable Nessus Agent scan.  - PVS—The asset data from a Tenable Nessus Network Monitor (NNM) scan.  - NESSUS_SCAN—The asset data was obtained from a Tenable Nessus scan.  - WAS—The asset data was obtained from a Tenable Web App Scanning scan. | [optional]
**has_plugin_results** | Option<**bool**> | Filter by whether or not the asset has plugin results associated with it. If `true`, Tenable Vulnerability Management returns all assets that have plugin results. If `false`, Tenable Vulnerability Management returns all assets that do not have plugin results. An asset may not have plugin results if the asset details originated from a connector or an API import. | [optional]
**tag_period_less_than_category_greater_than** | Option<**String**> | Returns all assets with the specified tag. The filter is defined as \"tag\", a period (\".\"), and the tag category name. The value of the filter is the tag value. For more information about tags, see [Tenable Vulnerability Management User Guide](https://docs.tenable.com/vulnerability-management/Content/Platform/Settings/Tagging/TagFormatAndApplication.htm). | [optional]
**network_id** | Option<**String**> | The ID of the network object associated with scanners that identified the assets you want to export. The default network ID is `00000000-0000-0000-0000-000000000000`. To determine the ID of a custom network, use the [GET /networks](ref:networks-list) endpoint. For more information about network objects, see [Manage Networks](doc:manage-networks-tio). | [optional]
**last_scan_id** | Option<**String**> | Returns all assets that were last scanned by the specified scan configuration UUID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


