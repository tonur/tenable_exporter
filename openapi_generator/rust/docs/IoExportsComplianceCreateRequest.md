# IoExportsComplianceCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**num_findings** | **i32** | Specifies the number of compliance findings per exported chunk. The range is 50-10000. If your request omits the `num_findings` body parameter, the value defaults to `5000`. If you specify a value outside of the supported range, Tenable Vulnerability Management uses the upper or lower-bound value. | [default to 5000]
**asset** | Option<**Vec<String>**> | A list of asset UUIDs for which you want to return compliance data. The list can contain a maximum of 200 asset UUIDs. | [optional]
**filters** | Option<[**crate::models::IoExportsComplianceCreateRequestFilters**](io_exports_compliance_create_request_filters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


