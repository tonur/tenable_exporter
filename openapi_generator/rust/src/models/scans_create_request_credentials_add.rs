/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScansCreateRequestCredentialsAdd : A credentials object you want to add to the scan. For scan-specific credentials, the parameters of the object vary based on credentials category, credentials type, and type-specific settings. For more information, see [Determine Settings for a Credential Type](doc:determine-settings-for-credential-type). For managed credentials, the object contains a single parameter `id`, which specifies the UUID of the managed credentials you want to add.  **Note:** This form displays limited parameters that support a Windows type of credentials that uses password authentication.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScansCreateRequestCredentialsAdd {
    #[serde(rename = "Host", skip_serializing_if = "Option::is_none")]
    pub host: Option<Box<crate::models::ScansCreateRequestCredentialsAddHost>>,
}

impl ScansCreateRequestCredentialsAdd {
    /// A credentials object you want to add to the scan. For scan-specific credentials, the parameters of the object vary based on credentials category, credentials type, and type-specific settings. For more information, see [Determine Settings for a Credential Type](doc:determine-settings-for-credential-type). For managed credentials, the object contains a single parameter `id`, which specifies the UUID of the managed credentials you want to add.  **Note:** This form displays limited parameters that support a Windows type of credentials that uses password authentication.
    pub fn new() -> ScansCreateRequestCredentialsAdd {
        ScansCreateRequestCredentialsAdd {
            host: None,
        }
    }
}

