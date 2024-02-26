/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportsVulnsExportCancel200Response {
    /// Text describing the export job status, `CANCELLED`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ExportsVulnsExportCancel200Response {
    pub fn new() -> ExportsVulnsExportCancel200Response {
        ExportsVulnsExportCancel200Response {
            status: None,
        }
    }
}

