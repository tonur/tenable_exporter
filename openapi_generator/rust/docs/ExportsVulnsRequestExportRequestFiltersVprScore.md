# ExportsVulnsRequestExportRequestFiltersVprScore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**eq** | Option<**Vec<f32>**> | Returns vulnerabilities with a VPR equal to the specified score or scores. This property cannot be combined with the following range operators: `lt`, `gt`, `lte`, or `gte`. | [optional]
**neq** | Option<**Vec<f32>**> | Returns vulnerabilities with a VPR not equal to the specified score or scores. This property can be combined with the `eq` property. | [optional]
**gt** | Option<**f32**> | Returns vulnerabilities with a VPR greater than the specified score. This property cannot be combined with the `eq` property. | [optional]
**gte** | Option<**f32**> | Returns vulnerabilities with a VPR greater than or equal to the specified score. This property cannot be combined with the `eq` property. | [optional]
**lt** | Option<**f32**> | Returns vulnerabilities with a VPR lesser than the specified score. This property cannot be combined with the `eq` property. | [optional]
**lte** | Option<**f32**> | Returns vulnerabilities with a VPR lesser than or equal to the specified score. This property cannot be combined with the `eq` property. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


