# IoPluginsList200ResponseDataPluginDetailsInnerAttributesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plugin_modification_date** | Option<**String**> | The date when Tenable last updated the plugin. | [optional]
**plugin_version** | Option<**String**> | The version of the plugin. | [optional]
**exploited_by_malware** | Option<**bool**> | Indicates whether the vulnerability discovered by this plugin is known to be exploited by malware. | [optional]
**description** | Option<**String**> | The extended description of the plugin. | [optional]
**unsupported_by_vendor** | Option<**bool**> | Indicates whether the software found by this plugin is unsupported by the software's vendor (for example, Windows 95 or Firefox 3). | [optional]
**cvss_temporal_score** | Option<**f32**> | The raw CVSSv2 temporal metrics for the vulnerability. | [optional]
**patch_publication_date** | Option<**String**> | The date when the vendor published a patch for the vulnerability. | [optional]
**see_also** | Option<**Vec<String>**> | Links to external websites that contain helpful information about the vulnerability. | [optional]
**default_account** | Option<**String**> | Indicates whether the plugin checks for default accounts requiring the use of credentials other than the credentials provided in the scan policy. For more information, see [What are the plugins that test for default accounts?](https://community.tenable.com/s/article/What-are-the-plugins-that-test-for-default-accounts) in the Tenable Community Portal. | [optional]
**exploit_available** | Option<**bool**> | Indicates whether a known public exploit exists for the vulnerability. | [optional]
**cve** | Option<**Vec<String>**> | A list of Common Vulnerabilities and Exposures (CVE) IDs for the vulnerabilities associated with the plugin. | [optional]
**exploit_framework_canvas** | Option<**bool**> | Indicates whether an exploit exists in the Immunity CANVAS framework. | [optional]
**cvss_base_score** | Option<**String**> | The CVSSv2 base score (intrinsic and fundamental characteristics of a vulnerability that are constant over time and user environments). | [optional]
**solution** | Option<**String**> | Remediation information for the vulnerability. | [optional]
**cvss_vector** | Option<[**crate::models::IoPluginsList200ResponseDataPluginDetailsInnerAttributesInnerCvssVector**](io_plugins_list_200_response_data_plugin_details_inner_attributes_inner_cvss_vector.md)> |  | [optional]
**exploit_framework_exploithub** | Option<**bool**> | Indicates whether an exploit exists in the ExploitHub framework. | [optional]
**cpe** | Option<**Vec<String>**> | A list of plugin target systems identified by Common Platform Enumeration (CPE). | [optional]
**plugin_publication_date** | Option<**String**> | The date when Tenable originally published the plugin. | [optional]
**exploit_framework_core** | Option<**bool**> | Indicates whether an exploit exists in the CORE Impact framework. | [optional]
**in_the_news** | Option<**bool**> | Indicates whether this plugin has received media attention (for example, ShellShock, Meltdown). | [optional]
**has_patch** | Option<**bool**> | Indicates whether the vendor has published a patch for the vulnerability. | [optional]
**xref** | Option<**Vec<String>**> | References to third-party information about the vulnerability, exploit, or update associated with the plugin. Each reference includes a type, for example, 'FEDORA', and an ID, for example, '2003-047'. | [optional]
**malware** | Option<**bool**> | Indicates whether the plugin targets potentially malicious files or processes. | [optional]
**exploit_framework_d2_elliot** | Option<**bool**> | Indicates an exploit exists in the D2 Elliot Web Exploitation framework. | [optional]
**xrefs** | Option<**Vec<String>**> | References to third-party information about the vulnerability, exploit, or update associated with the plugin. Each reference includes a type and an ID. For example, 'FEDORA' and '2003-047'. | [optional]
**risk_factor** | Option<**String**> | The risk factor associated with the plugin. Possible values are: Low, Medium, High, or Critical. | [optional]
**synopsis** | Option<**String**> | A brief summary of the vulnerability or vulnerabilities associated with the plugin. | [optional]
**cvss3_temporal_score** | Option<**f32**> | The CVSSv3 temporal metrics for the vulnerability. | [optional]
**exploited_by_nessus** | Option<**bool**> | Indicates whether Nessus exploited the vulnerability during the process of identification. | [optional]
**cvss3_base_score** | Option<**String**> | The CVSSv3 base score (intrinsic and fundamental characteristics of a vulnerability that are constant over time and user environments). | [optional]
**exploit_framework_metasploit** | Option<**bool**> | Indicates whether an exploit exists in the Metasploit framework. | [optional]
**plugin_type** | Option<**String**> | Plugin type, for example, local, remote, or combined. For more information about plugin type, see [Nessus Plugin Types and Categories](https://community.tenable.com/s/article/Nessus-Plugin-Types-and-Categories) in the Tenable Community Portal. | [optional]
**vpr** | Option<[**crate::models::IoPluginsList200ResponseDataPluginDetailsInnerAttributesInnerVpr**](io_plugins_list_200_response_data_plugin_details_inner_attributes_inner_vpr.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


