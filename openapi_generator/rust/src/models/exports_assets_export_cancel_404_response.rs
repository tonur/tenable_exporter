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
pub struct ExportsAssetsExportCancel404Response {
    /// The HTTP error code.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// Text describing the error condition Tenable Vulnerability Management encountered.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ExportsAssetsExportCancel404Response {
    pub fn new() -> ExportsAssetsExportCancel404Response {
        ExportsAssetsExportCancel404Response {
            status: None,
            message: None,
        }
    }
}

