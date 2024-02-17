# AssetsImportRequestAssetsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mac_address** | Option<**Vec<String>**> | A list of MAC addresses for the asset. | [optional]
**netbios_name** | Option<**String**> | The NetBIOS name for the asset. | [optional]
**fqdn** | Option<**Vec<String>**> | A list of FQDNs for the asset. | [optional]
**ip_address** | Option<**Vec<String>**> | A list of IPv4 addresses for the asset. Tenable.io supports this legacy field for backwards compatibility, but for new requests, this field should be replaced by the ipv4 field. | [optional]
**ipv4** | Option<**Vec<String>**> | A list of IPv4 addresses for the asset. | [optional]
**ipv6** | Option<**Vec<String>**> | A list of IPv6 addresses for the asset. | [optional]
**hostname** | Option<**Vec<String>**> | A list of hostnames for the asset. | [optional]
**operating_system** | Option<**Vec<String>**> | The operating systems that scans have associated with the asset record. | [optional]
**ssh_fingerprint** | Option<**String**> | The SSH key fingerprints that scans have associated with the asset record. | [optional]
**bios_uuid** | Option<**String**> | The BIOS UUID of the asset. | [optional]
**manufacturer_tpm_id** | Option<**String**> | The manufacturer's unique identifier of the Trusted Platform Module (TPM) associated with the asset. | [optional]
**mcafee_epo_guid** | Option<**String**> | The unique identifier of the asset in McAfee ePolicy Orchestrator (ePO). For more information, see the McAfee documentation. | [optional]
**mcafee_epo_agent_guid** | Option<**String**> | The unique identifier of the McAfee ePO agent that identified the asset. For more information, see the McAfee documentation. | [optional]
**symantec_ep_hardware_key** | Option<**String**> | The hardware key for the asset in Symantec Endpoint Protection. | [optional]
**qualys_asset_id** | Option<**String**> | The Asset ID of the asset in Qualys. For more information, see the Qualys documentation. | [optional]
**qualys_host_id** | Option<**String**> | The Host ID of the asset in Qualys. For more information, see the Qualys documentation. | [optional]
**servicenow_sys_id** | Option<**String**> | The unique record identifier of the asset in ServiceNow. For more information, see the ServiceNow documentation. | [optional]
**gcp_project_id** | Option<**String**> | The customized name of the project to which the virtual machine instance belongs in Google Cloud Platform (GCP). For more information see \"Creating and Managing Projects\" in the GCP documentation. | [optional]
**gcp_zone** | Option<**String**> | The zone where the virtual machine instance runs in GCP. For more information, see \"Regions and Zones\" in the GCP documentation. | [optional]
**gcp_instance_id** | Option<**String**> | The unique identifier of the virtual machine instance in GCP. | [optional]
**azure_vm_id** | Option<**String**> | The unique identifier of the Microsoft Azure virtual machine instance. For more information, see \"Accessing and Using Azure VM Unique ID\" in the Microsoft Azure documentation. | [optional]
**azure_resource_id** | Option<**String**> | The unique identifier of the resource in the Azure Resource Manager. For more information, see the Azure Resource Manager Documentation.  **Note:** Do not include AWS or GCP data in the payload when importing Azure asset data. | [optional]
**aws_availability_zone** | Option<**String**> | The availability zone where Amazon Web Services hosts the virtual machine instance, for example, `us-east-1a`. Availability zones are subdivisions of AWS regions. For more information, see \"Regions and Availability Zones\" in the AWS documentation. | [optional]
**aws_ec2_instance_id** | Option<**String**> | The unique identifier of the Linux instance in Amazon EC2. For more information, see the Amazon Elastic Compute Cloud Documentation. | [optional]
**aws_ec2_instance_ami_id** | Option<**String**> | The unique identifier of the Linux AMI image in Amazon Elastic Compute Cloud (Amazon EC2). For more information, see the Amazon Elastic Compute Cloud Documentation. | [optional]
**aws_ec2_instance_group_name** | Option<**String**> | The virtual machine instance's group in AWS. | [optional]
**aws_ec2_instance_state_name** | Option<**String**> | The state of the virtual machine instance in AWS at the time of the scan. For more information on instance states, see the AWS documentation. | [optional]
**aws_ec2_instance_type** | Option<**String**> | The type of instance in AWS EC2. | [optional]
**aws_ec2_name** | Option<**String**> | The name of the virtual machine instance in AWS EC2. | [optional]
**aws_ec2_product_code** | Option<**String**> | The product code associated with the AMI used to launch the virtual machine instance in AWS EC2. | [optional]
**aws_owner_id** | Option<**String**> | The canonical user identifier for the AWS account associated with the asset. For more information, see \"AWS Account Identifiers\" in the AWS documentation.  **Note:** Do not include Azure or GCP data in the payload when importing AWS asset data. | [optional]
**aws_region** | Option<**String**> | The region where AWS hosts the virtual machine instance, for example, `us-east-1`. For more information, see \"Regions and Availability Zones\" in the AWS documentation. | [optional]
**aws_subnet_id** | Option<**String**> | The unique identifier of the AWS subnet where the virtual machine instance was running at the time of the scan. | [optional]
**aws_vpc_id** | Option<**String**> | The unique identifier of the public cloud that hosts the AWS virtual machine instance. For more information, see the Amazon Virtual Private Cloud User Guide. | [optional]
**installed_software** | Option<**Vec<String>**> | A list of Common Platform Enumeration (CPE) values that represent software applications a scan identified as present on an asset. The strings in this array must be valid CPE 2.2 values. For more information, see the \"Component Syntax\" section of the [CPE Specification, Version 2.2](https://cpe.mitre.org/files/cpe-specification_2.2.pdf).  **Note:** If no scan detects an application within 30 days of the scan that originally detected the application, Tenable.io considers the detection of that application expired. As a result, the next time a scan evaluates the asset, Tenable.io removes the expired application from the installed_software attribute. This activity is logged as a `remove` type of `attribute_change` update in the asset activity log. | [optional]
**bigfix_asset_id** | Option<**Vec<String>**> | The unique identifiers of the asset in HCL BigFix. For more information, see the HCL BigFix documentation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


