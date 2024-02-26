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
pub struct IoFiltersAssetsListV2Request {
    /// A list of tag UUIDs that are guaranteed to be returned in the initial data set of the `tag_uuid` filter, if the tags exist.
    #[serde(rename = "tag_uuids", skip_serializing_if = "Option::is_none")]
    pub tag_uuids: Option<Vec<String>>,
}

impl IoFiltersAssetsListV2Request {
    pub fn new() -> IoFiltersAssetsListV2Request {
        IoFiltersAssetsListV2Request {
            tag_uuids: None,
        }
    }
}

