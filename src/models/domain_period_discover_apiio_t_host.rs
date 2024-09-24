/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainPeriodDiscoverApiioTHost {
    /// Whether the asset is account-enabled in Active Directory (Yes or No).
    #[serde(rename = "account_enabled", skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<String>,
    /// The user account control properties in Active Directory.
    #[serde(
        rename = "ad_user_account_control",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_user_account_control: Option<i32>,
    /// The version of the Falcon sensor that's installed on the asset.
    #[serde(rename = "agent_version", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// The agent ID of the Falcon sensor installed on the asset.
    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    /// The asset role or roles currently assigned to the asset either automatically or by a user (Jump host, Highly connected, Highly active, Server by behavior, DHCP server, DNS server, FTP server, SSH server, or Web server).
    #[serde(rename = "asset_roles", skip_serializing_if = "Option::is_none")]
    pub asset_roles: Option<Vec<String>>,
    /// The first and last name of the person who is assigned to this asset.
    #[serde(rename = "assigned_to", skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// The available disk space in the last 15 minutes on the host
    #[serde(
        rename = "available_disk_space",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_disk_space: Option<i32>,
    /// The available disk space percent in the last 15 minutes on the host
    #[serde(
        rename = "available_disk_space_pct",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_disk_space_pct: Option<i32>,
    /// The average memory usage in the last 15 minutes on the host
    #[serde(
        rename = "average_memory_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_memory_usage: Option<i32>,
    /// The average memory usage percent in the last 15 minutes on the host
    #[serde(
        rename = "average_memory_usage_pct",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_memory_usage_pct: Option<i32>,
    /// The average processor usage in the last 15 minutes on the host
    #[serde(
        rename = "average_processor_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_processor_usage: Option<i32>,
    /// The list of found sha256 and their measurement types
    #[serde(rename = "bios_hashes_data", skip_serializing_if = "Option::is_none")]
    pub bios_hashes_data: Option<Vec<models::DomainPeriodDiscoverApiBiosHashesData>>,
    /// The id of the bios on the host
    #[serde(rename = "bios_id", skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    /// The name of the asset's BIOS manufacturer.
    #[serde(rename = "bios_manufacturer", skip_serializing_if = "Option::is_none")]
    pub bios_manufacturer: Option<String>,
    /// The asset's BIOS version.
    #[serde(rename = "bios_version", skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    /// The business criticality of the IoT asset.
    #[serde(
        rename = "business_criticality",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_criticality: Option<String>,
    /// The asset's customer ID.
    #[serde(rename = "cid")]
    pub cid: String,
    /// The name of the city where the asset is located.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The external ID of the IoT Device in 3rd Party System(Claroty).
    #[serde(rename = "claroty_id", skip_serializing_if = "Option::is_none")]
    pub claroty_id: Option<String>,
    /// How the server is classified, such as production, development, disaster recovery, or user acceptance testing.
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// The asset role or roles assigned to the asset automatically (Jump host, Highly connected, Highly active, Server by behavior, DHCP server, DNS server, FTP server, SSH server, or Web server).
    #[serde(
        rename = "computed_asset_roles",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_asset_roles: Option<Vec<String>>,
    /// Whether the asset is exposed to the internet as determined automatically (Yes, No, or Pending).
    #[serde(
        rename = "computed_internet_exposure",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_internet_exposure: Option<String>,
    /// External IP exposed to the internet.
    #[serde(
        rename = "computed_internet_exposure_external_ip",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_internet_exposure_external_ip: Option<String>,
    /// When the asset was last seen as internet exposed.
    #[serde(
        rename = "computed_internet_exposure_last_seen",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_internet_exposure_last_seen: Option<String>,
    /// The level of confidence that the asset is a corporate asset (25 = low confidence, 50 = medium confidence, 75 = high confidence).
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    /// The name of the country where the asset is located.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The manufacturer of the asset's CPU.
    #[serde(rename = "cpu_manufacturer", skip_serializing_if = "Option::is_none")]
    pub cpu_manufacturer: Option<String>,
    /// The name of the processor on the system
    #[serde(rename = "cpu_processor_name", skip_serializing_if = "Option::is_none")]
    pub cpu_processor_name: Option<String>,
    /// The time the asset was created in Active Directory, according to LDAP info.
    #[serde(rename = "creation_timestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// The criticality level of the asset (Critical, High, Noncritical, or Unassigned)
    #[serde(rename = "criticality", skip_serializing_if = "Option::is_none")]
    pub criticality: Option<String>,
    /// The description the user entered when manually assigning a criticality level
    #[serde(
        rename = "criticality_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub criticality_description: Option<String>,
    /// The ID of the criticality rule that has most recently applied to the asset.
    #[serde(
        rename = "criticality_rule_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub criticality_rule_id: Option<String>,
    /// The date and time the criticality level was manually assigned
    #[serde(
        rename = "criticality_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub criticality_timestamp: Option<String>,
    /// The username of the account that manually assigned the criticality level
    #[serde(
        rename = "criticality_username",
        skip_serializing_if = "Option::is_none"
    )]
    pub criticality_username: Option<String>,
    /// The last seen local IPv4 address of the asset.
    #[serde(rename = "current_local_ip", skip_serializing_if = "Option::is_none")]
    pub current_local_ip: Option<String>,
    /// The last seen network prefix of the asset.
    #[serde(
        rename = "current_network_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_network_prefix: Option<String>,
    /// Where the data about the asset came from (such as CrowdStrike, ServiceNow, or Active Directory).
    #[serde(rename = "data_providers", skip_serializing_if = "Option::is_none")]
    pub data_providers: Option<Vec<String>>,
    /// How many services provided data about the asset.
    #[serde(
        rename = "data_providers_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_providers_count: Option<i32>,
    /// The department where the asset is used.
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// The descriptions of the asset in Active Directory (Cannot be used for filtering, sorting, or querying).
    #[serde(rename = "descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The Device Class of IoT Asset
    #[serde(rename = "device_class", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    /// The Device Family of IoT Asset
    #[serde(rename = "device_family", skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    /// The device mode of the host
    #[serde(rename = "device_mode", skip_serializing_if = "Option::is_none")]
    pub device_mode: Option<String>,
    /// The slots of IoT Asset
    #[serde(rename = "device_slots", skip_serializing_if = "Option::is_none")]
    pub device_slots: Option<Vec<models::DomainPeriodDiscoverApiDeviceSlot>>,
    /// The Device Type of IoT Asset
    #[serde(rename = "device_type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// The agent IDs of the Falcon sensors installed on the sources that discovered the asset.
    #[serde(rename = "discoverer_aids", skip_serializing_if = "Option::is_none")]
    pub discoverer_aids: Option<Vec<String>>,
    /// The number of sources that discovered the asset.
    #[serde(rename = "discoverer_count", skip_serializing_if = "Option::is_none")]
    pub discoverer_count: Option<i32>,
    /// The criticalities of the sources that discovered the asset
    #[serde(
        rename = "discoverer_criticalities",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_criticalities: Option<Vec<String>>,
    /// The hostnames of the sources that discovered the asset.
    #[serde(
        rename = "discoverer_hostnames",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_hostnames: Option<Vec<String>>,
    /// A list of agent IDs of the Falcon sensors installed on the source hosts that discovered the asset via ICS Asset discovery mechanism
    #[serde(
        rename = "discoverer_ics_collector_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_ics_collector_ids: Option<Vec<String>>,
    /// The platform names of the sources that discovered the asset.
    #[serde(
        rename = "discoverer_platform_names",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_platform_names: Option<Vec<String>>,
    /// The product type descriptions of the sources that discovered the asset.
    #[serde(
        rename = "discoverer_product_type_descs",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_product_type_descs: Option<Vec<String>>,
    /// The tags of the sources that discovered the asset.
    #[serde(rename = "discoverer_tags", skip_serializing_if = "Option::is_none")]
    pub discoverer_tags: Option<Vec<String>>,
    /// Represents the status of a managed host (“Not Discovering“, “Passive“, “Active“ or both).
    #[serde(rename = "discovering_by", skip_serializing_if = "Option::is_none")]
    pub discovering_by: Option<Vec<String>>,
    /// The names and sizes of the disks on the asset
    #[serde(rename = "disk_sizes", skip_serializing_if = "Option::is_none")]
    pub disk_sizes: Option<Vec<models::DomainPeriodDiscoverApiDiskSize>>,
    /// The ID generated by dragos asset discovery mechanism
    #[serde(rename = "dragos_id", skip_serializing_if = "Option::is_none")]
    pub dragos_id: Option<String>,
    /// The email of the asset as listed in Active Directory.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The list of encrypted drives on the host
    #[serde(rename = "encrypted_drives", skip_serializing_if = "Option::is_none")]
    pub encrypted_drives: Option<Vec<String>>,
    /// The count of encrypted drives on the host
    #[serde(
        rename = "encrypted_drives_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypted_drives_count: Option<i32>,
    /// The encryption status of the host
    #[serde(rename = "encryption_status", skip_serializing_if = "Option::is_none")]
    pub encryption_status: Option<String>,
    /// The type of asset (managed, unmanaged, unsupported).
    #[serde(rename = "entity_type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The external IPv4 address of the asset.
    #[serde(rename = "external_ip", skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// Lists the data providers for each property in the response (Cannot be used for filtering, sorting, or querying).
    #[serde(rename = "field_metadata", skip_serializing_if = "Option::is_none")]
    pub field_metadata:
        Option<std::collections::HashMap<String, models::DomainPeriodDiscoverApiFieldMetadata>>,
    /// The agent ID of the Falcon sensor on the source that first discovered the asset.
    #[serde(
        rename = "first_discoverer_aid",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_discoverer_aid: Option<String>,
    /// The first time the asset was seen in your environment.
    #[serde(
        rename = "first_seen_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_seen_timestamp: Option<String>,
    /// The form factor of the host
    #[serde(rename = "form_factor", skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// The fully qualified domain name of the asset.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    /// The host management groups the asset is part of.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// The asset's hostname.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The ID generated by ICS collector asset discovery mechanism
    #[serde(rename = "ics_id", skip_serializing_if = "Option::is_none")]
    pub ics_id: Option<String>,
    /// The unique ID of the asset.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the asset is exposed to the internet (Yes, No or Pending).
    #[serde(rename = "internet_exposure", skip_serializing_if = "Option::is_none")]
    pub internet_exposure: Option<String>,
    /// The description the user entered when manually assigning a internet exposure level
    #[serde(
        rename = "internet_exposure_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_exposure_description: Option<String>,
    /// The date and time the internet exposure level was manually assigned
    #[serde(
        rename = "internet_exposure_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_exposure_timestamp: Option<String>,
    /// The username of the account that manually assigned the internet exposure level
    #[serde(
        rename = "internet_exposure_username",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_exposure_username: Option<String>,
    /// For Linux and Mac hosts: the major version, minor version, and patch version of the kernel for the asset. For Windows hosts: the build number of the asset.
    #[serde(rename = "kernel_version", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// The agent ID of the Falcon sensor installed on the source that most recently discovered the asset.
    #[serde(
        rename = "last_discoverer_aid",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_discoverer_aid: Option<String>,
    /// The hostname of the last source that discovered the asset.
    #[serde(
        rename = "last_discoverer_hostname",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_discoverer_hostname: Option<String>,
    /// The agent ID of the Falcon sensor installed on the source host that most recently discovered the asset via ICS Asset discovery mechanism
    #[serde(
        rename = "last_discoverer_ics_collector_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_discoverer_ics_collector_id: Option<String>,
    /// The most recent time the asset was seen in your environment.
    #[serde(
        rename = "last_seen_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_seen_timestamp: Option<String>,
    /// Historical local IPv4 addresses associated with the asset.
    #[serde(rename = "local_ip_addresses", skip_serializing_if = "Option::is_none")]
    pub local_ip_addresses: Option<Vec<String>>,
    /// The number of historical local IPv4 addresses the asset has had.
    #[serde(rename = "local_ips_count", skip_serializing_if = "Option::is_none")]
    pub local_ips_count: Option<i32>,
    /// The location of the asset.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The number of logical cores available on the system
    #[serde(rename = "logical_core_count", skip_serializing_if = "Option::is_none")]
    pub logical_core_count: Option<i32>,
    /// Historical MAC addresses associated with the asset.
    #[serde(rename = "mac_addresses", skip_serializing_if = "Option::is_none")]
    pub mac_addresses: Option<Vec<String>>,
    /// The domain name the asset is currently joined to.
    #[serde(rename = "machine_domain", skip_serializing_if = "Option::is_none")]
    pub machine_domain: Option<String>,
    /// The first and last name of the person who manages this asset.
    #[serde(rename = "managed_by", skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// The max memory usage in the last 15 minutes on the host
    #[serde(rename = "max_memory_usage", skip_serializing_if = "Option::is_none")]
    pub max_memory_usage: Option<i32>,
    /// The max memory usage percent in the last 15 minutes on the host
    #[serde(
        rename = "max_memory_usage_pct",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_memory_usage_pct: Option<i32>,
    /// The max processor usage in the last 15 minutes on the host
    #[serde(
        rename = "max_processor_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_processor_usage: Option<i32>,
    /// The Total memory.
    #[serde(rename = "memory_total", skip_serializing_if = "Option::is_none")]
    pub memory_total: Option<i64>,
    /// The path, used and available space on mounted disks
    #[serde(rename = "mount_storage_info", skip_serializing_if = "Option::is_none")]
    pub mount_storage_info: Option<Vec<models::DomainPeriodDiscoverApiMountStorageInfo>>,
    /// The network ID to which device is connected.
    #[serde(rename = "network_id", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// The asset's network interfaces (Cannot be used for filtering, sorting, or querying).
    #[serde(rename = "network_interfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<models::DomainPeriodDiscoverApiNetworkInterface>>,
    /// The number of active physical drives available on the system.
    #[serde(
        rename = "number_of_disk_drives",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_disk_drives: Option<i32>,
    /// The globally unique identifier (GUID) of the asset in Active Directory.
    #[serde(rename = "object_guid", skip_serializing_if = "Option::is_none")]
    pub object_guid: Option<String>,
    /// The security identifier (SID) of the asset in Active Directory.
    #[serde(rename = "object_sid", skip_serializing_if = "Option::is_none")]
    pub object_sid: Option<String>,
    /// Whether the asset is at end of support (Yes, No, or Unknown).
    #[serde(rename = "os_is_eol", skip_serializing_if = "Option::is_none")]
    pub os_is_eol: Option<String>,
    #[serde(rename = "os_security", skip_serializing_if = "Option::is_none")]
    pub os_security: Option<Box<models::DomainPeriodDiscoverApiosSecurity>>,
    /// The OS service pack on the asset.
    #[serde(rename = "os_service_pack", skip_serializing_if = "Option::is_none")]
    pub os_service_pack: Option<String>,
    /// The OS version of the asset.
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// A list of sources through which host is discovered
    #[serde(
        rename = "ot_information_sources",
        skip_serializing_if = "Option::is_none"
    )]
    pub ot_information_sources: Option<Vec<String>>,
    /// A list of network ids to which host belongs
    #[serde(rename = "ot_network_ids", skip_serializing_if = "Option::is_none")]
    pub ot_network_ids: Option<Vec<String>>,
    /// A list of serial numbers that discovered with host
    #[serde(rename = "ot_serial_numbers", skip_serializing_if = "Option::is_none")]
    pub ot_serial_numbers: Option<Vec<String>>,
    /// The organizational unit of the asset.
    #[serde(rename = "ou", skip_serializing_if = "Option::is_none")]
    pub ou: Option<String>,
    /// Whether a user overrode automatically assigned asset roles to manually assign a role to the asset (true or false).
    #[serde(
        rename = "override_asset_roles",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_asset_roles: Option<bool>,
    /// Whether a user overrode a criticality rule to manually assign a criticality level on the asset (true or false).
    #[serde(
        rename = "override_criticality_rules",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_criticality_rules: Option<bool>,
    /// Whether a user overrode the automatically assigned internet exposure (True or False).
    #[serde(
        rename = "override_internet_exposure",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_internet_exposure: Option<bool>,
    /// The first and last name of the person who owns this asset.
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    /// The number of physical CPU cores available on the system.
    #[serde(
        rename = "physical_core_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_core_count: Option<i32>,
    /// The platform name of the asset (Windows, Mac, Linux).
    #[serde(rename = "platform_name", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    /// The number of physical processors available on the system.
    #[serde(
        rename = "processor_package_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_package_count: Option<i32>,
    /// The product type of the asset represented as a number (1 = Workstation, 2 = Domain Controller, 3 = Server).
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// The product type of the asset (Workstation, Domain Controller, Server).
    #[serde(rename = "product_type_desc", skip_serializing_if = "Option::is_none")]
    pub product_type_desc: Option<String>,
    /// The list of protocols supported by the device
    #[serde(rename = "protocols", skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// The purdue level of IoT Asset
    #[serde(rename = "purdue_level", skip_serializing_if = "Option::is_none")]
    pub purdue_level: Option<String>,
    /// Whether the asset is in reduced functionality mode (Yes or No).
    #[serde(
        rename = "reduced_functionality_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub reduced_functionality_mode: Option<String>,
    /// The unique identifier of the asset from ServiceNow, if any.
    #[serde(rename = "servicenow_id", skip_serializing_if = "Option::is_none")]
    pub servicenow_id: Option<String>,
    /// The site name of the domain the asset is joined to (applies only to Windows hosts).
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// The name of the U.S. state where the asset is located.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The subnet to which device is connected.
    #[serde(rename = "subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    /// The asset's system manufacturer.
    #[serde(
        rename = "system_manufacturer",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_manufacturer: Option<String>,
    /// The asset's system product name.
    #[serde(
        rename = "system_product_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_product_name: Option<String>,
    /// The asset's system serial number.
    #[serde(
        rename = "system_serial_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_serial_number: Option<String>,
    /// The sensor and cloud tags of the asset.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The count of bios files measured by the firmware image
    #[serde(rename = "total_bios_files", skip_serializing_if = "Option::is_none")]
    pub total_bios_files: Option<i32>,
    /// Total amount of disk space available on the system
    #[serde(rename = "total_disk_space", skip_serializing_if = "Option::is_none")]
    pub total_disk_space: Option<i32>,
    /// The total memory of the asset
    #[serde(rename = "total_memory", skip_serializing_if = "Option::is_none")]
    pub total_memory: Option<i32>,
    #[serde(rename = "triage", skip_serializing_if = "Option::is_none")]
    pub triage: Option<Box<models::DomainPeriodDiscoverApiioTHostTriage>>,
    /// The list of unencrypted drives on the host
    #[serde(rename = "unencrypted_drives", skip_serializing_if = "Option::is_none")]
    pub unencrypted_drives: Option<Vec<String>>,
    /// The count of unencrypted drives on the host
    #[serde(
        rename = "unencrypted_drives_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub unencrypted_drives_count: Option<i32>,
    /// The used disk space in the last 15 minutes on the host
    #[serde(rename = "used_disk_space", skip_serializing_if = "Option::is_none")]
    pub used_disk_space: Option<i32>,
    /// The used disk space percent in the last 15 minutes on the host
    #[serde(
        rename = "used_disk_space_pct",
        skip_serializing_if = "Option::is_none"
    )]
    pub used_disk_space_pct: Option<i32>,
    /// What the asset is used for, such as production, staging, or QA.
    #[serde(rename = "used_for", skip_serializing_if = "Option::is_none")]
    pub used_for: Option<String>,
    /// The asset role or roles manually assigned to the asset.
    #[serde(rename = "user_asset_roles", skip_serializing_if = "Option::is_none")]
    pub user_asset_roles: Option<Vec<String>>,
    /// The internet exposure manually assigned to the asset
    #[serde(
        rename = "user_internet_exposure",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_internet_exposure: Option<String>,
    /// The Virtual Zone name in which device is installed.
    #[serde(rename = "virtual_zone", skip_serializing_if = "Option::is_none")]
    pub virtual_zone: Option<String>,
    /// The VLAN IDs to which device is connected.
    #[serde(rename = "vlan", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<Vec<String>>,
    /// The external ID of the IoT Device in 3rd Party System(Claroty Xdome)
    #[serde(rename = "xdome_id", skip_serializing_if = "Option::is_none")]
    pub xdome_id: Option<String>,
}

impl DomainPeriodDiscoverApiioTHost {
    pub fn new(cid: String, id: String) -> DomainPeriodDiscoverApiioTHost {
        DomainPeriodDiscoverApiioTHost {
            account_enabled: None,
            ad_user_account_control: None,
            agent_version: None,
            aid: None,
            asset_roles: None,
            assigned_to: None,
            available_disk_space: None,
            available_disk_space_pct: None,
            average_memory_usage: None,
            average_memory_usage_pct: None,
            average_processor_usage: None,
            bios_hashes_data: None,
            bios_id: None,
            bios_manufacturer: None,
            bios_version: None,
            business_criticality: None,
            cid,
            city: None,
            claroty_id: None,
            classification: None,
            computed_asset_roles: None,
            computed_internet_exposure: None,
            computed_internet_exposure_external_ip: None,
            computed_internet_exposure_last_seen: None,
            confidence: None,
            country: None,
            cpu_manufacturer: None,
            cpu_processor_name: None,
            creation_timestamp: None,
            criticality: None,
            criticality_description: None,
            criticality_rule_id: None,
            criticality_timestamp: None,
            criticality_username: None,
            current_local_ip: None,
            current_network_prefix: None,
            data_providers: None,
            data_providers_count: None,
            department: None,
            descriptions: None,
            device_class: None,
            device_family: None,
            device_mode: None,
            device_slots: None,
            device_type: None,
            discoverer_aids: None,
            discoverer_count: None,
            discoverer_criticalities: None,
            discoverer_hostnames: None,
            discoverer_ics_collector_ids: None,
            discoverer_platform_names: None,
            discoverer_product_type_descs: None,
            discoverer_tags: None,
            discovering_by: None,
            disk_sizes: None,
            dragos_id: None,
            email: None,
            encrypted_drives: None,
            encrypted_drives_count: None,
            encryption_status: None,
            entity_type: None,
            external_ip: None,
            field_metadata: None,
            first_discoverer_aid: None,
            first_seen_timestamp: None,
            form_factor: None,
            fqdn: None,
            groups: None,
            hostname: None,
            ics_id: None,
            id,
            internet_exposure: None,
            internet_exposure_description: None,
            internet_exposure_timestamp: None,
            internet_exposure_username: None,
            kernel_version: None,
            last_discoverer_aid: None,
            last_discoverer_hostname: None,
            last_discoverer_ics_collector_id: None,
            last_seen_timestamp: None,
            local_ip_addresses: None,
            local_ips_count: None,
            location: None,
            logical_core_count: None,
            mac_addresses: None,
            machine_domain: None,
            managed_by: None,
            max_memory_usage: None,
            max_memory_usage_pct: None,
            max_processor_usage: None,
            memory_total: None,
            mount_storage_info: None,
            network_id: None,
            network_interfaces: None,
            number_of_disk_drives: None,
            object_guid: None,
            object_sid: None,
            os_is_eol: None,
            os_security: None,
            os_service_pack: None,
            os_version: None,
            ot_information_sources: None,
            ot_network_ids: None,
            ot_serial_numbers: None,
            ou: None,
            override_asset_roles: None,
            override_criticality_rules: None,
            override_internet_exposure: None,
            owned_by: None,
            physical_core_count: None,
            platform_name: None,
            processor_package_count: None,
            product_type: None,
            product_type_desc: None,
            protocols: None,
            purdue_level: None,
            reduced_functionality_mode: None,
            servicenow_id: None,
            site_name: None,
            state: None,
            subnet: None,
            system_manufacturer: None,
            system_product_name: None,
            system_serial_number: None,
            tags: None,
            total_bios_files: None,
            total_disk_space: None,
            total_memory: None,
            triage: None,
            unencrypted_drives: None,
            unencrypted_drives_count: None,
            used_disk_space: None,
            used_disk_space_pct: None,
            used_for: None,
            user_asset_roles: None,
            user_internet_exposure: None,
            virtual_zone: None,
            vlan: None,
            xdome_id: None,
        }
    }
}
