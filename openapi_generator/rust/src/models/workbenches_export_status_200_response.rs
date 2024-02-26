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
pub struct WorkbenchesExportStatus200Response {
    /// The export processing status, for example, READY or LOADING.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The total number of items included in export.
    #[serde(rename = "progress_total", skip_serializing_if = "Option::is_none")]
    pub progress_total: Option<String>,
    /// The number of already processed items.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}

impl WorkbenchesExportStatus200Response {
    pub fn new() -> WorkbenchesExportStatus200Response {
        WorkbenchesExportStatus200Response {
            status: None,
            progress_total: None,
            progress: None,
        }
    }
}

