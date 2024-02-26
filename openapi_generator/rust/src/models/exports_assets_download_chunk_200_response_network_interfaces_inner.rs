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
pub struct ExportsAssetsDownloadChunk200ResponseNetworkInterfacesInner {
    /// The name of the interface.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The MAC addresses of the interface.
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Vec<String>>,
    /// One or more IPv4 addresses belonging to the interface.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// One or more IPv6 addresses belonging to the interface.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
    /// One or more FQDN belonging to the interface.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<Vec<String>>,
}

impl ExportsAssetsDownloadChunk200ResponseNetworkInterfacesInner {
    pub fn new() -> ExportsAssetsDownloadChunk200ResponseNetworkInterfacesInner {
        ExportsAssetsDownloadChunk200ResponseNetworkInterfacesInner {
            name: None,
            mac_address: None,
            ipv4: None,
            ipv6: None,
            fqdn: None,
        }
    }
}

