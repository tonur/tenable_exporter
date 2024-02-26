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
pub struct PoliciesList200ResponseInner {
    /// The unique ID of the policy.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The UUID for the Tenable-provided template used to create the policy.
    #[serde(rename = "template_uuid", skip_serializing_if = "Option::is_none")]
    pub template_uuid: Option<String>,
    /// The name of the policy.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the policy.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique ID of the owner of the policy.
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// The username for the owner of the policy.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// The shared status of the policy (`1` if shared with users other than owner, `0` if not shared).
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<i32>,
    /// The sharing permissions for the policy.
    #[serde(rename = "user_permissions", skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<i32>,
    /// The creation date of the policy in Unix time format.
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<i32>,
    /// The last modification date for the policy in Unix time format.
    #[serde(rename = "last_modification_date", skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<i32>,
    /// The visibility of the target (`private` or `shared`).
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<i32>,
    /// If `true`, the policy configuration does not include targets.
    #[serde(rename = "no_target", skip_serializing_if = "Option::is_none")]
    pub no_target: Option<bool>,
}

impl PoliciesList200ResponseInner {
    pub fn new() -> PoliciesList200ResponseInner {
        PoliciesList200ResponseInner {
            id: None,
            template_uuid: None,
            name: None,
            description: None,
            owner_id: None,
            owner: None,
            shared: None,
            user_permissions: None,
            creation_date: None,
            last_modification_date: None,
            visibility: None,
            no_target: None,
        }
    }
}

