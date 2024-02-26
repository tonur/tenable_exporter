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
pub struct WorkbenchesAssetsActivity200ResponseInner {
    /// Event type:  - discovered—Asset created (for example, by a network scan or import).  - seen—Asset observed by a network scan without any changes to its attributes.  - tagging—Tag added to or removed from asset.  - attribute_change—A scan identified new or changed attributes for the asset (for example, new software applications installed on the asset).  - updated—Asset updated either manually by a user or automatically by a new scan.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The timestamp of the event. The timestamp is reported in ISO 8601 format in UTC time.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// The UUID of the scan that logged the event.
    #[serde(rename = "scan_id", skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    /// The ID of the scheduled scan associated with the event.
    #[serde(rename = "schedule_id", skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// The entity that logged the event, for example, NESSUS_AGENT, NESSUS_AGENT, PVS, or WAS.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::WorkbenchesAssetsActivity200ResponseInnerDetails>>,
    /// (attribute_change entries only) A list of updates to the asset's attributes.
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::WorkbenchesAssetsActivity200ResponseInnerUpdatesInner>>,
}

impl WorkbenchesAssetsActivity200ResponseInner {
    pub fn new() -> WorkbenchesAssetsActivity200ResponseInner {
        WorkbenchesAssetsActivity200ResponseInner {
            r#type: None,
            timestamp: None,
            scan_id: None,
            schedule_id: None,
            source: None,
            details: None,
            updates: None,
        }
    }
}

