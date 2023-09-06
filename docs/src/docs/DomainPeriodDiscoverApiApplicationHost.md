# DomainPeriodDiscoverApiApplicationHost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_version** | Option<**String**> | The version of the Falcon sensor that's installed on the asset. | [optional]
**aid** | Option<**String**> | The agent ID of the Falcon sensor installed on the asset. | [optional]
**country** | Option<**String**> | The name of the country where the asset is located. | [optional]
**current_mac_address** | Option<**String**> | The last seen MAC address of the asset. | [optional]
**current_network_prefix** | Option<**String**> | The last seen network prefix of the asset. | [optional]
**external_ip** | Option<**String**> | The external IPv4 address of the asset. | [optional]
**groups** | Option<**Vec<String>**> | The host management groups the asset is part of. | [optional]
**hostname** | Option<**String**> | The asset's hostname. | [optional]
**id** | **String** | The unique ID of the asset. |
**internet_exposure** | Option<**String**> | Whether the asset is exposed to the internet (Yes or Unknown). | [optional]
**kernel_version** | Option<**String**> | For Linux and Mac hosts: the major version, minor version, and patch version of the kernel for the asset. For Windows hosts: the build number of the asset. | [optional]
**machine_domain** | Option<**String**> | The domain name the asset is currently joined to. | [optional]
**os_version** | Option<**String**> | The OS version of the asset. | [optional]
**ou** | Option<**String**> | The organizational unit of the asset. | [optional]
**platform_name** | Option<**String**> | The platform name of the asset (Windows, Mac, Linux). | [optional]
**product_type_desc** | Option<**String**> | The product type of the asset (Workstation, Domain Controller, Server). | [optional]
**site_name** | Option<**String**> | The site name of the domain the asset is joined to (applies only to Windows hosts). | [optional]
**system_manufacturer** | Option<**String**> | The asset's system manufacturer. | [optional]
**tags** | Option<**Vec<String>**> | The sensor and cloud tags of the asset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
