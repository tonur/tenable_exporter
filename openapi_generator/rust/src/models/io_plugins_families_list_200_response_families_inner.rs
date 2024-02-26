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
pub struct IoPluginsFamiliesList200ResponseFamiliesInner {
    /// The number of plugins in the family.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The name of the family.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique ID of the family.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl IoPluginsFamiliesList200ResponseFamiliesInner {
    pub fn new() -> IoPluginsFamiliesList200ResponseFamiliesInner {
        IoPluginsFamiliesList200ResponseFamiliesInner {
            count: None,
            name: None,
            id: None,
        }
    }
}

