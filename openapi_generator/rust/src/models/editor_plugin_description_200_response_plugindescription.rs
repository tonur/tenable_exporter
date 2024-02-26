/*
 * Vulnerability Management
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditorPluginDescription200ResponsePlugindescription : The detailed information for a Tenable.io plugin.



#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorPluginDescription200ResponsePlugindescription {
    /// The severity level of the vulnerabilities targeted by the plugin
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// The name of the plugin.
    #[serde(rename = "pluginname", skip_serializing_if = "Option::is_none")]
    pub pluginname: Option<String>,
    /// The attributes of the plugin, including synopsis, description, solution, and risk information.
    #[serde(rename = "pluginattributes", skip_serializing_if = "Option::is_none")]
    pub pluginattributes: Option<serde_json::Value>,
    /// The name of the plugin family.
    #[serde(rename = "pluginfamily", skip_serializing_if = "Option::is_none")]
    pub pluginfamily: Option<String>,
    /// The ID of the plugin.
    #[serde(rename = "pluginid", skip_serializing_if = "Option::is_none")]
    pub pluginid: Option<i32>,
}

impl EditorPluginDescription200ResponsePlugindescription {
    /// The detailed information for a Tenable.io plugin.
    pub fn new() -> EditorPluginDescription200ResponsePlugindescription {
        EditorPluginDescription200ResponsePlugindescription {
            severity: None,
            pluginname: None,
            pluginattributes: None,
            pluginfamily: None,
            pluginid: None,
        }
    }
}

