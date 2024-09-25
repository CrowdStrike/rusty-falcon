
# Documentation for API Endpoints

All URIs are relative to *<https://api.crowdstrike.com>*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AlertsApi* | [**get_queries_alerts_v1**](./AlertsApi.md#get_queries_alerts_v1) | **GET** /alerts/queries/alerts/v1 | retrieves all Alerts ids that match a given query
*AlertsApi* | [**patch_entities_alerts_v2**](./AlertsApi.md#patch_entities_alerts_v2) | **PATCH** /alerts/entities/alerts/v2 | Perform actions on detections identified by detection ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.
*AlertsApi* | [**post_aggregates_alerts_v1**](./AlertsApi.md#post_aggregates_alerts_v1) | **POST** /alerts/aggregates/alerts/v1 | retrieves aggregates for Alerts across all CIDs
*AlertsApi* | [**post_entities_alerts_v1**](./AlertsApi.md#post_entities_alerts_v1) | **POST** /alerts/entities/alerts/v1 | retrieves all Alerts given their ids
*CloudConnectAwsApi* | [**create_or_update_aws_settings**](./CloudConnectAwsApi.md#create_or_update_aws_settings) | **POST** /cloud-connect-aws/entities/settings/v1 | Create or update Global Settings which are applicable to all provisioned AWS accounts
*CloudConnectAwsApi* | [**delete_aws_accounts**](./CloudConnectAwsApi.md#delete_aws_accounts) | **DELETE** /cloud-connect-aws/entities/accounts/v1 | Delete a set of AWS Accounts by specifying their IDs
*CloudConnectAwsApi* | [**get_aws_accounts**](./CloudConnectAwsApi.md#get_aws_accounts) | **GET** /cloud-connect-aws/entities/accounts/v1 | Retrieve a set of AWS Accounts by specifying their IDs
*CloudConnectAwsApi* | [**get_aws_settings**](./CloudConnectAwsApi.md#get_aws_settings) | **GET** /cloud-connect-aws/combined/settings/v1 | Retrieve a set of Global Settings which are applicable to all provisioned AWS accounts
*CloudConnectAwsApi* | [**provision_aws_accounts**](./CloudConnectAwsApi.md#provision_aws_accounts) | **POST** /cloud-connect-aws/entities/accounts/v1 | Provision AWS Accounts by specifying details about the accounts to provision
*CloudConnectAwsApi* | [**query_aws_accounts**](./CloudConnectAwsApi.md#query_aws_accounts) | **GET** /cloud-connect-aws/combined/accounts/v1 | Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS accounts which match the filter criteria
*CloudConnectAwsApi* | [**query_aws_accounts_for_ids**](./CloudConnectAwsApi.md#query_aws_accounts_for_ids) | **GET** /cloud-connect-aws/queries/accounts/v1 | Search for provisioned AWS Accounts by providing an FQL filter and paging details. Returns a set of AWS account IDs which match the filter criteria
*CloudConnectAwsApi* | [**update_aws_accounts**](./CloudConnectAwsApi.md#update_aws_accounts) | **PATCH** /cloud-connect-aws/entities/accounts/v1 | Update AWS Accounts by specifying the ID of the account and details to update
*CloudConnectAwsApi* | [**verify_aws_account_access**](./CloudConnectAwsApi.md#verify_aws_account_access) | **POST** /cloud-connect-aws/entities/verify-account-access/v1 | Performs an Access Verification check on the specified AWS Account IDs
*ConfigurationAssessmentApi* | [**get_combined_assessments_query**](./ConfigurationAssessmentApi.md#get_combined_assessments_query) | **GET** /configuration-assessment/combined/assessments/v1 | Search for assessments in your environment by providing an FQL filter and paging details. Returns a set of HostFinding entities which match the filter criteria
*ConfigurationAssessmentEvaluationLogicApi* | [**get_evaluation_logic_mixin0**](./ConfigurationAssessmentEvaluationLogicApi.md#get_evaluation_logic_mixin0) | **GET** /configuration-assessment/entities/evaluation-logic/v1 | Get details on evaluation logic items by providing one or more finding IDs.
*CspmRegistrationApi* | [**azure_download_certificate**](./CspmRegistrationApi.md#azure_download_certificate) | **GET** /cloud-connect-cspm-azure/entities/download-certificate/v1 | Returns JSON object(s) that contain the base64 encoded certificate for a service principal.
*CspmRegistrationApi* | [**create_cspm_aws_account**](./CspmRegistrationApi.md#create_cspm_aws_account) | **POST** /cloud-connect-cspm-aws/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.
*CspmRegistrationApi* | [**create_cspm_azure_account**](./CspmRegistrationApi.md#create_cspm_azure_account) | **POST** /cloud-connect-cspm-azure/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.
*CspmRegistrationApi* | [**delete_cspm_aws_account**](./CspmRegistrationApi.md#delete_cspm_aws_account) | **DELETE** /cloud-connect-cspm-aws/entities/account/v1 | Deletes an existing AWS account or organization in our system.
*CspmRegistrationApi* | [**delete_cspm_azure_account**](./CspmRegistrationApi.md#delete_cspm_azure_account) | **DELETE** /cloud-connect-cspm-azure/entities/account/v1 | Deletes an Azure subscription from the system.
*CspmRegistrationApi* | [**get_behavior_detections**](./CspmRegistrationApi.md#get_behavior_detections) | **GET** /detects/entities/ioa/v1 | Get list of detected behaviors
*CspmRegistrationApi* | [**get_configuration_detection_entities**](./CspmRegistrationApi.md#get_configuration_detection_entities) | **GET** /detects/entities/iom/v2 | Get misconfigurations based on the ID - including custom policy detections in addition to default policy detections.
*CspmRegistrationApi* | [**get_configuration_detection_ids_v2**](./CspmRegistrationApi.md#get_configuration_detection_ids_v2) | **GET** /detects/queries/iom/v2 | Get list of active misconfiguration ids - including custom policy detections in addition to default policy detections.
*CspmRegistrationApi* | [**get_configuration_detections**](./CspmRegistrationApi.md#get_configuration_detections) | **GET** /detects/entities/iom/v1 | Get list of active misconfigurations
*CspmRegistrationApi* | [**get_cspm_aws_account**](./CspmRegistrationApi.md#get_cspm_aws_account) | **GET** /cloud-connect-cspm-aws/entities/account/v1 | Returns information about the current status of an AWS account.
*CspmRegistrationApi* | [**get_cspm_aws_account_scripts_attachment**](./CspmRegistrationApi.md#get_cspm_aws_account_scripts_attachment) | **GET** /cloud-connect-cspm-aws/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.
*CspmRegistrationApi* | [**get_cspm_aws_console_setup_urls**](./CspmRegistrationApi.md#get_cspm_aws_console_setup_urls) | **GET** /cloud-connect-cspm-aws/entities/console-setup-urls/v1 | Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.
*CspmRegistrationApi* | [**get_cspm_azure_account**](./CspmRegistrationApi.md#get_cspm_azure_account) | **GET** /cloud-connect-cspm-azure/entities/account/v1 | Return information about Azure account registration
*CspmRegistrationApi* | [**get_cspm_azure_user_scripts_attachment**](./CspmRegistrationApi.md#get_cspm_azure_user_scripts_attachment) | **GET** /cloud-connect-cspm-azure/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment
*CspmRegistrationApi* | [**get_cspm_policies_details**](./CspmRegistrationApi.md#get_cspm_policies_details) | **GET** /settings/entities/policy-details/v2 | Given an array of policy IDs, returns detailed policies information.
*CspmRegistrationApi* | [**get_cspm_policy**](./CspmRegistrationApi.md#get_cspm_policy) | **GET** /settings/entities/policy-details/v1 | Given a policy ID, returns detailed policy information.
*CspmRegistrationApi* | [**get_cspm_policy_settings**](./CspmRegistrationApi.md#get_cspm_policy_settings) | **GET** /settings/entities/policy/v1 | Returns information about current policy settings.
*CspmRegistrationApi* | [**get_cspm_scan_schedule**](./CspmRegistrationApi.md#get_cspm_scan_schedule) | **GET** /settings/scan-schedule/v1 | Returns scan schedule configuration for one or more cloud platforms.
*CspmRegistrationApi* | [**patch_cspm_aws_account**](./CspmRegistrationApi.md#patch_cspm_aws_account) | **PATCH** /cloud-connect-cspm-aws/entities/account/v1 | Patches a existing account in our system for a customer.
*CspmRegistrationApi* | [**update_cspm_azure_account_client_id**](./CspmRegistrationApi.md#update_cspm_azure_account_client_id) | **PATCH** /cloud-connect-cspm-azure/entities/client-id/v1 | Update an Azure service account in our system by with the user-created client_id created with the public key we've provided
*CspmRegistrationApi* | [**update_cspm_azure_tenant_default_subscription_id**](./CspmRegistrationApi.md#update_cspm_azure_tenant_default_subscription_id) | **PATCH** /cloud-connect-cspm-azure/entities/default-subscription-id/v1 | Update an Azure default subscription_id in our system for given tenant_id
*CspmRegistrationApi* | [**update_cspm_policy_settings**](./CspmRegistrationApi.md#update_cspm_policy_settings) | **PATCH** /settings/entities/policy/v1 | Updates a policy setting - can be used to override policy severity or to disable a policy entirely.
*CspmRegistrationApi* | [**update_cspm_scan_schedule**](./CspmRegistrationApi.md#update_cspm_scan_schedule) | **POST** /settings/scan-schedule/v1 | Updates scan schedule configuration for one or more cloud platforms.
*CustomIoaApi* | [**create_rule**](./CustomIoaApi.md#create_rule) | **POST** /ioarules/entities/rules/v1 | Create a rule within a rule group. Returns the rule.
*CustomIoaApi* | [**create_rule_group_mixin0**](./CustomIoaApi.md#create_rule_group_mixin0) | **POST** /ioarules/entities/rule-groups/v1 | Create a rule group for a platform with a name and an optional description. Returns the rule group.
*CustomIoaApi* | [**delete_rule_groups_mixin0**](./CustomIoaApi.md#delete_rule_groups_mixin0) | **DELETE** /ioarules/entities/rule-groups/v1 | Delete rule groups by ID.
*CustomIoaApi* | [**delete_rules**](./CustomIoaApi.md#delete_rules) | **DELETE** /ioarules/entities/rules/v1 | Delete rules from a rule group by ID.
*CustomIoaApi* | [**get_patterns**](./CustomIoaApi.md#get_patterns) | **GET** /ioarules/entities/pattern-severities/v1 | Get pattern severities by ID.
*CustomIoaApi* | [**get_platforms_mixin0**](./CustomIoaApi.md#get_platforms_mixin0) | **GET** /ioarules/entities/platforms/v1 | Get platforms by ID.
*CustomIoaApi* | [**get_rule_groups_mixin0**](./CustomIoaApi.md#get_rule_groups_mixin0) | **GET** /ioarules/entities/rule-groups/v1 | Get rule groups by ID.
*CustomIoaApi* | [**get_rule_types**](./CustomIoaApi.md#get_rule_types) | **GET** /ioarules/entities/rule-types/v1 | Get rule types by ID.
*CustomIoaApi* | [**get_rules_get**](./CustomIoaApi.md#get_rules_get) | **POST** /ioarules/entities/rules/GET/v1 | Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`.
*CustomIoaApi* | [**get_rules_mixin0**](./CustomIoaApi.md#get_rules_mixin0) | **GET** /ioarules/entities/rules/v1 | Get rules by ID and optionally with cid and/or version in the following format: `[cid:]ID[:version]`. The max number of IDs is constrained by URL size.
*CustomIoaApi* | [**query_patterns**](./CustomIoaApi.md#query_patterns) | **GET** /ioarules/queries/pattern-severities/v1 | Get all pattern severity IDs.
*CustomIoaApi* | [**query_platforms_mixin0**](./CustomIoaApi.md#query_platforms_mixin0) | **GET** /ioarules/queries/platforms/v1 | Get all platform IDs.
*CustomIoaApi* | [**query_rule_groups_full**](./CustomIoaApi.md#query_rule_groups_full) | **GET** /ioarules/queries/rule-groups-full/v1 | Find all rule groups matching the query with optional filter.
*CustomIoaApi* | [**query_rule_groups_mixin0**](./CustomIoaApi.md#query_rule_groups_mixin0) | **GET** /ioarules/queries/rule-groups/v1 | Finds all rule group IDs matching the query with optional filter.
*CustomIoaApi* | [**query_rule_types**](./CustomIoaApi.md#query_rule_types) | **GET** /ioarules/queries/rule-types/v1 | Get all rule type IDs.
*CustomIoaApi* | [**query_rules_mixin0**](./CustomIoaApi.md#query_rules_mixin0) | **GET** /ioarules/queries/rules/v1 | Finds all rule IDs matching the query with optional filter.
*CustomIoaApi* | [**update_rule_group_mixin0**](./CustomIoaApi.md#update_rule_group_mixin0) | **PATCH** /ioarules/entities/rule-groups/v1 | Update a rule group. The following properties can be modified: name, description, enabled.
*CustomIoaApi* | [**update_rules**](./CustomIoaApi.md#update_rules) | **PATCH** /ioarules/entities/rules/v1 | Update rules within a rule group. Return the updated rules.
*CustomIoaApi* | [**validate**](./CustomIoaApi.md#validate) | **POST** /ioarules/entities/rules/validate/v1 | Validates field values and checks for matches if a test string is provided.
*D4cRegistrationApi* | [**create_d4_c_aws_account**](./D4cRegistrationApi.md#create_d4_c_aws_account) | **POST** /cloud-connect-aws/entities/account/v2 | Creates a new account in our system for a customer and generates a script for them to run in their AWS cloud environment to grant us access.
*D4cRegistrationApi* | [**create_d4_cgcp_account**](./D4cRegistrationApi.md#create_d4_cgcp_account) | **POST** /cloud-connect-gcp/entities/account/v1 | Creates a new account in our system for a customer and generates a new service account for them to add access to in their GCP environment to grant us access.
*D4cRegistrationApi* | [**create_discover_cloud_azure_account**](./D4cRegistrationApi.md#create_discover_cloud_azure_account) | **POST** /cloud-connect-azure/entities/account/v1 | Creates a new account in our system for a customer and generates a script for them to run in their cloud environment to grant us access.
*D4cRegistrationApi* | [**delete_d4_c_aws_account**](./D4cRegistrationApi.md#delete_d4_c_aws_account) | **DELETE** /cloud-connect-aws/entities/account/v2 | Deletes an existing AWS account or organization in our system.
*D4cRegistrationApi* | [**discover_cloud_azure_download_certificate**](./D4cRegistrationApi.md#discover_cloud_azure_download_certificate) | **GET** /cloud-connect-azure/entities/download-certificate/v1 | Returns JSON object(s) that contain the base64 encoded certificate for a service principal.
*D4cRegistrationApi* | [**get_d4_c_aws_account**](./D4cRegistrationApi.md#get_d4_c_aws_account) | **GET** /cloud-connect-aws/entities/account/v2 | Returns information about the current status of an AWS account.
*D4cRegistrationApi* | [**get_d4_c_aws_console_setup_urls**](./D4cRegistrationApi.md#get_d4_c_aws_console_setup_urls) | **GET** /cloud-connect-aws/entities/console-setup-urls/v1 | Return a URL for customer to visit in their cloud environment to grant us access to their AWS environment.
*D4cRegistrationApi* | [**get_d4_caws_account_scripts_attachment**](./D4cRegistrationApi.md#get_d4_caws_account_scripts_attachment) | **GET** /cloud-connect-aws/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their AWS environment as a downloadable attachment.
*D4cRegistrationApi* | [**get_d4_ccgp_account**](./D4cRegistrationApi.md#get_d4_ccgp_account) | **GET** /cloud-connect-gcp/entities/account/v1 | Returns information about the current status of an GCP account.
*D4cRegistrationApi* | [**get_d4_cgcp_user_scripts**](./D4cRegistrationApi.md#get_d4_cgcp_user_scripts) | **GET** /cloud-connect-gcp/entities/user-scripts/v1 | Return a script for customer to run in their cloud environment to grant us access to their GCP environment
*D4cRegistrationApi* | [**get_discover_cloud_azure_account**](./D4cRegistrationApi.md#get_discover_cloud_azure_account) | **GET** /cloud-connect-azure/entities/account/v1 | Return information about Azure account registration
*D4cRegistrationApi* | [**get_discover_cloud_azure_tenant_ids**](./D4cRegistrationApi.md#get_discover_cloud_azure_tenant_ids) | **GET** /cloud-connect-azure/entities/tenant-id/v1 | Return available tenant ids for discover for cloud
*D4cRegistrationApi* | [**get_discover_cloud_azure_user_scripts**](./D4cRegistrationApi.md#get_discover_cloud_azure_user_scripts) | **GET** /cloud-connect-azure/entities/user-scripts/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment
*D4cRegistrationApi* | [**get_discover_cloud_azure_user_scripts_attachment**](./D4cRegistrationApi.md#get_discover_cloud_azure_user_scripts_attachment) | **GET** /cloud-connect-azure/entities/user-scripts-download/v1 | Return a script for customer to run in their cloud environment to grant us access to their Azure environment as a downloadable attachment
*D4cRegistrationApi* | [**get_horizon_d4_c_scripts**](./D4cRegistrationApi.md#get_horizon_d4_c_scripts) | **GET** /settings-discover/entities/gen/scripts/v1 | Returns static install scripts for Horizon.
*D4cRegistrationApi* | [**update_discover_cloud_azure_account_client_id**](./D4cRegistrationApi.md#update_discover_cloud_azure_account_client_id) | **PATCH** /cloud-connect-azure/entities/client-id/v1 | Update an Azure service account in our system by with the user-created client_id created with the public key we've provided
*DetectsApi* | [**get_aggregate_detects**](./DetectsApi.md#get_aggregate_detects) | **POST** /detects/aggregates/detects/GET/v1 | Get detect aggregates as specified via json in request body.
*DetectsApi* | [**get_detect_summaries**](./DetectsApi.md#get_detect_summaries) | **POST** /detects/entities/summaries/GET/v1 | View information about detections
*DetectsApi* | [**query_detects**](./DetectsApi.md#query_detects) | **GET** /detects/queries/detects/v1 | Search for detection IDs that match a given query
*DetectsApi* | [**update_detects_by_ids_v2**](./DetectsApi.md#update_detects_by_ids_v2) | **PATCH** /detects/entities/detects/v2 | Modify the state, assignee, and visibility of detections
*DeviceControlPoliciesApi* | [**create_device_control_policies**](./DeviceControlPoliciesApi.md#create_device_control_policies) | **POST** /policy/entities/device-control/v1 | Create Device Control Policies by specifying details about the policy to create
*DeviceControlPoliciesApi* | [**delete_device_control_policies**](./DeviceControlPoliciesApi.md#delete_device_control_policies) | **DELETE** /policy/entities/device-control/v1 | Delete a set of Device Control Policies by specifying their IDs
*DeviceControlPoliciesApi* | [**get_default_device_control_policies**](./DeviceControlPoliciesApi.md#get_default_device_control_policies) | **GET** /policy/entities/default-device-control/v1 | Retrieve the configuration for a Default Device Control Policy
*DeviceControlPoliciesApi* | [**get_device_control_policies**](./DeviceControlPoliciesApi.md#get_device_control_policies) | **GET** /policy/entities/device-control/v1 | Retrieve a set of Device Control Policies by specifying their IDs
*DeviceControlPoliciesApi* | [**perform_device_control_policies_action**](./DeviceControlPoliciesApi.md#perform_device_control_policies_action) | **POST** /policy/entities/device-control-actions/v1 | Perform the specified action on the Device Control Policies specified in the request
*DeviceControlPoliciesApi* | [**query_combined_device_control_policies**](./DeviceControlPoliciesApi.md#query_combined_device_control_policies) | **GET** /policy/combined/device-control/v1 | Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policies which match the filter criteria
*DeviceControlPoliciesApi* | [**query_combined_device_control_policy_members**](./DeviceControlPoliciesApi.md#query_combined_device_control_policy_members) | **GET** /policy/combined/device-control-members/v1 | Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*DeviceControlPoliciesApi* | [**query_device_control_policies**](./DeviceControlPoliciesApi.md#query_device_control_policies) | **GET** /policy/queries/device-control/v1 | Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policy IDs which match the filter criteria
*DeviceControlPoliciesApi* | [**query_device_control_policy_members**](./DeviceControlPoliciesApi.md#query_device_control_policy_members) | **GET** /policy/queries/device-control-members/v1 | Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*DeviceControlPoliciesApi* | [**set_device_control_policies_precedence**](./DeviceControlPoliciesApi.md#set_device_control_policies_precedence) | **POST** /policy/entities/device-control-precedence/v1 | Sets the precedence of Device Control Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
*DeviceControlPoliciesApi* | [**update_default_device_control_policies**](./DeviceControlPoliciesApi.md#update_default_device_control_policies) | **PATCH** /policy/entities/default-device-control/v1 | Update the configuration for a Default Device Control Policy
*DeviceControlPoliciesApi* | [**update_device_control_policies**](./DeviceControlPoliciesApi.md#update_device_control_policies) | **PATCH** /policy/entities/device-control/v1 | Update Device Control Policies by specifying the ID of the policy and details to update
*DiscoverApi* | [**get_accounts**](./DiscoverApi.md#get_accounts) | **GET** /discover/entities/accounts/v1 | Get details on accounts by providing one or more IDs.
*DiscoverApi* | [**get_applications**](./DiscoverApi.md#get_applications) | **GET** /discover/entities/applications/v1 | Get details on applications by providing one or more IDs.
*DiscoverApi* | [**get_hosts**](./DiscoverApi.md#get_hosts) | **GET** /discover/entities/hosts/v1 | Get details on assets by providing one or more IDs.
*DiscoverApi* | [**get_logins**](./DiscoverApi.md#get_logins) | **GET** /discover/entities/logins/v1 | Get details on logins by providing one or more IDs.
*DiscoverApi* | [**query_accounts**](./DiscoverApi.md#query_accounts) | **GET** /discover/queries/accounts/v1 | Search for accounts in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of account IDs which match the filter criteria.
*DiscoverApi* | [**query_applications**](./DiscoverApi.md#query_applications) | **GET** /discover/queries/applications/v1 | Search for applications in your environment by providing an FQL filter and paging details. returns a set of application IDs which match the filter criteria.
*DiscoverApi* | [**query_hosts**](./DiscoverApi.md#query_hosts) | **GET** /discover/queries/hosts/v1 | Search for assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.
*DiscoverApi* | [**query_logins**](./DiscoverApi.md#query_logins) | **GET** /discover/queries/logins/v1 | Search for logins in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of login IDs which match the filter criteria.
*DiscoverIotApi* | [**get_iot_hosts**](./DiscoverIotApi.md#get_iot_hosts) | **GET** /discover/entities/iot-hosts/v1 | Get details on IoT assets by providing one or more IDs.
*DiscoverIotApi* | [**query_iot_hosts**](./DiscoverIotApi.md#query_iot_hosts) | **GET** /discover/queries/iot-hosts/v1 | Search for IoT assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.
*EventSchemaApi* | [**fdrschema_period_combined_period_event_period_get**](./EventSchemaApi.md#fdrschema_period_combined_period_event_period_get) | **GET** /fdr/combined/schema-members/v1 | Fetch combined schema
*EventSchemaApi* | [**fdrschema_period_entities_period_event_period_get**](./EventSchemaApi.md#fdrschema_period_entities_period_event_period_get) | **GET** /fdr/entities/schema-events/v1 | Fetch event schema by ID
*EventSchemaApi* | [**fdrschema_period_queries_period_event_period_get**](./EventSchemaApi.md#fdrschema_period_queries_period_event_period_get) | **GET** /fdr/queries/schema-events/v1 | Get list of event IDs given a particular query.
*EventStreamsApi* | [**list_available_streams_o_auth2**](./EventStreamsApi.md#list_available_streams_o_auth2) | **GET** /sensors/entities/datafeed/v2 | Discover all event streams in your environment
*EventStreamsApi* | [**refresh_active_stream_session**](./EventStreamsApi.md#refresh_active_stream_session) | **POST** /sensors/entities/datafeed-actions/v1/{partition} | Refresh an active event stream. Use the URL shown in a GET /sensors/entities/datafeed/v2 response.
*FalconCompleteDashboardApi* | [**aggregate_alerts**](./FalconCompleteDashboardApi.md#aggregate_alerts) | **POST** /falcon-complete-dashboards/aggregates/alerts/GET/v1 | Retrieve aggregate alerts values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_allow_list**](./FalconCompleteDashboardApi.md#aggregate_allow_list) | **POST** /falcon-complete-dashboards/aggregates/allowlist/GET/v1 | Retrieve aggregate allowlist ticket values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_block_list**](./FalconCompleteDashboardApi.md#aggregate_block_list) | **POST** /falcon-complete-dashboards/aggregates/blocklist/GET/v1 | Retrieve aggregate blocklist ticket values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_detections**](./FalconCompleteDashboardApi.md#aggregate_detections) | **POST** /falcon-complete-dashboards/aggregates/detects/GET/v1 | Retrieve aggregate detection values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_device_count_collection**](./FalconCompleteDashboardApi.md#aggregate_device_count_collection) | **POST** /falcon-complete-dashboards/aggregates/devicecount-collections/GET/v1 | Retrieve aggregate host/devices count based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_escalations**](./FalconCompleteDashboardApi.md#aggregate_escalations) | **POST** /falcon-complete-dashboards/aggregates/escalations/GET/v1 | Retrieve aggregate escalation ticket values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_fc_incidents**](./FalconCompleteDashboardApi.md#aggregate_fc_incidents) | **POST** /falcon-complete-dashboards/aggregates/incidents/GET/v1 | Retrieve aggregate incident values based on the matched filter
*FalconCompleteDashboardApi* | [**aggregate_remediations**](./FalconCompleteDashboardApi.md#aggregate_remediations) | **POST** /falcon-complete-dashboards/aggregates/remediations/GET/v1 | Retrieve aggregate remediation ticket values based on the matched filter
*FalconCompleteDashboardApi* | [**get_device_count_collection_queries_by_filter**](./FalconCompleteDashboardApi.md#get_device_count_collection_queries_by_filter) | **GET** /falcon-complete-dashboards/queries/devicecount-collections/v1 | Retrieve device count collection Ids that match the provided FQL filter, criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_alert_ids_by_filter**](./FalconCompleteDashboardApi.md#query_alert_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/alerts/v1 | Retrieve Alerts Ids that match the provided FQL filter criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_allow_list_filter**](./FalconCompleteDashboardApi.md#query_allow_list_filter) | **GET** /falcon-complete-dashboards/queries/allowlist/v1 | Retrieve allowlist tickets that match the provided filter criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_block_list_filter**](./FalconCompleteDashboardApi.md#query_block_list_filter) | **GET** /falcon-complete-dashboards/queries/blocklist/v1 | Retrieve block list tickets that match the provided filter criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_detection_ids_by_filter**](./FalconCompleteDashboardApi.md#query_detection_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/detects/v1 | Retrieve DetectionsIds that match the provided FQL filter, criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_escalations_filter**](./FalconCompleteDashboardApi.md#query_escalations_filter) | **GET** /falcon-complete-dashboards/queries/escalations/v1 | Retrieve escalation tickets that match the provided filter criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_incident_ids_by_filter**](./FalconCompleteDashboardApi.md#query_incident_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/incidents/v1 | Retrieve incidents that match the provided filter criteria with scrolling enabled
*FalconCompleteDashboardApi* | [**query_remediations_filter**](./FalconCompleteDashboardApi.md#query_remediations_filter) | **GET** /falcon-complete-dashboards/queries/remediations/v1 | Retrieve remediation tickets that match the provided filter criteria with scrolling enabled
*FalconContainerApi* | [**get_credentials**](./FalconContainerApi.md#get_credentials) | **GET** /container-security/entities/image-registry-credentials/v1 | Gets the registry credentials
*FalconContainerCliApi* | [**read_image_vulnerabilities**](./FalconContainerCliApi.md#read_image_vulnerabilities) | **POST** /image-assessment/combined/vulnerability-lookups/v1 | Retrieve known vulnerabilities for the provided image
*FalconContainerImageApi* | [**create_registry_entities**](./FalconContainerImageApi.md#create_registry_entities) | **POST** /container-security/entities/registries/v1 | Create a registry entity using the provided details
*FalconContainerImageApi* | [**delete_registry_entities**](./FalconContainerImageApi.md#delete_registry_entities) | **DELETE** /container-security/entities/registries/v1 | Delete the registry entity identified by the entity UUID
*FalconContainerImageApi* | [**get_combined_images**](./FalconContainerImageApi.md#get_combined_images) | **GET** /container-security/combined/image-assessment/images/v1 | Get image assessment results by providing an FQL filter and paging details
*FalconContainerImageApi* | [**read_registry_entities**](./FalconContainerImageApi.md#read_registry_entities) | **GET** /container-security/queries/registries/v1 | Retrieve registry entities identified by the customer id
*FalconContainerImageApi* | [**read_registry_entities_by_uuid**](./FalconContainerImageApi.md#read_registry_entities_by_uuid) | **GET** /container-security/entities/registries/v1 | Retrieve the registry entity identified by the entity UUID
*FalconContainerImageApi* | [**update_registry_entities**](./FalconContainerImageApi.md#update_registry_entities) | **PATCH** /container-security/entities/registries/v1 | Update the registry entity, as identified by the entity UUID, using the provided details
*FalconxSandboxApi* | [**delete_report**](./FalconxSandboxApi.md#delete_report) | **DELETE** /falconx/entities/reports/v1 | Delete report based on the report ID. Operation can be checked for success by polling for the report ID on the report-summaries endpoint.
*FalconxSandboxApi* | [**delete_sample_v2**](./FalconxSandboxApi.md#delete_sample_v2) | **DELETE** /samples/entities/samples/v2 | Removes a sample, including file, meta and submissions from the collection
*FalconxSandboxApi* | [**get_artifacts**](./FalconxSandboxApi.md#get_artifacts) | **GET** /falconx/entities/artifacts/v1 | Download IOC packs, PCAP files, memory dumps, and other analysis artifacts.
*FalconxSandboxApi* | [**get_memory_dump**](./FalconxSandboxApi.md#get_memory_dump) | **GET** /falconx/entities/memory-dump/v1 | Get memory dump content, as binary
*FalconxSandboxApi* | [**get_memory_dump_extracted_strings**](./FalconxSandboxApi.md#get_memory_dump_extracted_strings) | **GET** /falconx/entities/memory-dump/extracted-strings/v1 | Get extracted strings from a memory dump
*FalconxSandboxApi* | [**get_memory_dump_hex_dump**](./FalconxSandboxApi.md#get_memory_dump_hex_dump) | **GET** /falconx/entities/memory-dump/hex-dump/v1 | Get hex view of a memory dump
*FalconxSandboxApi* | [**get_reports**](./FalconxSandboxApi.md#get_reports) | **GET** /falconx/entities/reports/v1 | Get a full sandbox report.
*FalconxSandboxApi* | [**get_sample_v2**](./FalconxSandboxApi.md#get_sample_v2) | **GET** /samples/entities/samples/v2 | Retrieves the file associated with the given ID (SHA256)
*FalconxSandboxApi* | [**get_submissions**](./FalconxSandboxApi.md#get_submissions) | **GET** /falconx/entities/submissions/v1 | Check the status of a sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.
*FalconxSandboxApi* | [**get_summary_reports**](./FalconxSandboxApi.md#get_summary_reports) | **GET** /falconx/entities/report-summaries/v1 | Get a short summary version of a sandbox report.
*FalconxSandboxApi* | [**interactive_detonation_delete**](./FalconxSandboxApi.md#interactive_detonation_delete) | **DELETE** /falconx-interactive/tunnels/{id}/v1 | Stops an interactive detonation.
*FalconxSandboxApi* | [**interactive_detonation_get**](./FalconxSandboxApi.md#interactive_detonation_get) | **GET** /falconx-interactive/tunnels/{id}/v1 | Performs an interactive detonation.
*FalconxSandboxApi* | [**interactive_detonation_post**](./FalconxSandboxApi.md#interactive_detonation_post) | **POST** /falconx-interactive/tunnels/{id}/v1 | Performs an interactive detonation.
*FalconxSandboxApi* | [**query_reports**](./FalconxSandboxApi.md#query_reports) | **GET** /falconx/queries/reports/v1 | Find sandbox reports by providing an FQL filter and paging details. Returns a set of report IDs that match your criteria.
*FalconxSandboxApi* | [**query_sample_v1**](./FalconxSandboxApi.md#query_sample_v1) | **POST** /samples/queries/samples/GET/v1 | Retrieves a list with sha256 of samples that exist and customer has rights to access them, maximum number of accepted items is 200
*FalconxSandboxApi* | [**query_submissions**](./FalconxSandboxApi.md#query_submissions) | **GET** /falconx/queries/submissions/v1 | Find submission IDs for uploaded files by providing an FQL filter and paging details. Returns a set of submission IDs that match your criteria.
*FalconxSandboxApi* | [**submit**](./FalconxSandboxApi.md#submit) | **POST** /falconx/entities/submissions/v1 | Submit an uploaded file or a URL for sandbox analysis. Time required for analysis varies but is usually less than 15 minutes.
*FalconxSandboxApi* | [**upload_sample_v2**](./FalconxSandboxApi.md#upload_sample_v2) | **POST** /samples/entities/samples/v2 | Upload a file for sandbox analysis. After uploading, use `/falconx/entities/submissions/v1` to start analyzing the file.
*FieldSchemaApi* | [**fdrschema_period_entities_period_field_period_get**](./FieldSchemaApi.md#fdrschema_period_entities_period_field_period_get) | **GET** /fdr/entities/schema-fields/v1 | Fetch field schema by ID
*FieldSchemaApi* | [**fdrschema_period_queries_period_field_period_get**](./FieldSchemaApi.md#fdrschema_period_queries_period_field_period_get) | **GET** /fdr/queries/schema-fields/v1 | Get list of field IDs given a particular query.
*FilevantageApi* | [**get_changes**](./FilevantageApi.md#get_changes) | **GET** /filevantage/entities/changes/v2 | Retrieve information on changes
*FilevantageApi* | [**high_volume_query_changes**](./FilevantageApi.md#high_volume_query_changes) | **GET** /filevantage/queries/changes/v3 | Returns 1 or more change ids
*FilevantageApi* | [**query_changes**](./FilevantageApi.md#query_changes) | **GET** /filevantage/queries/changes/v2 | Returns 1 or more change ids
*FirewallManagementApi* | [**aggregate_events**](./FirewallManagementApi.md#aggregate_events) | **POST** /fwmgr/aggregates/events/GET/v1 | Aggregate events for customer
*FirewallManagementApi* | [**aggregate_policy_rules**](./FirewallManagementApi.md#aggregate_policy_rules) | **POST** /fwmgr/aggregates/policy-rules/GET/v1 | Aggregate rules within a policy for customer
*FirewallManagementApi* | [**aggregate_rule_groups**](./FirewallManagementApi.md#aggregate_rule_groups) | **POST** /fwmgr/aggregates/rule-groups/GET/v1 | Aggregate rule groups for customer
*FirewallManagementApi* | [**aggregate_rules**](./FirewallManagementApi.md#aggregate_rules) | **POST** /fwmgr/aggregates/rules/GET/v1 | Aggregate rules for customer
*FirewallManagementApi* | [**create_network_locations**](./FirewallManagementApi.md#create_network_locations) | **POST** /fwmgr/entities/network-locations/v1 | Create new network locations provided, and return the ID.
*FirewallManagementApi* | [**create_rule_group**](./FirewallManagementApi.md#create_rule_group) | **POST** /fwmgr/entities/rule-groups/v1 | Create new rule group on a platform for a customer with a name and description, and return the ID
*FirewallManagementApi* | [**create_rule_group_validation**](./FirewallManagementApi.md#create_rule_group_validation) | **POST** /fwmgr/entities/rule-groups/validation/v1 | Validates the request of creating a new rule group on a platform for a customer with a name and description
*FirewallManagementApi* | [**delete_network_locations**](./FirewallManagementApi.md#delete_network_locations) | **DELETE** /fwmgr/entities/network-locations/v1 | Delete network location entities by ID.
*FirewallManagementApi* | [**delete_rule_groups**](./FirewallManagementApi.md#delete_rule_groups) | **DELETE** /fwmgr/entities/rule-groups/v1 | Delete rule group entities by ID
*FirewallManagementApi* | [**get_events**](./FirewallManagementApi.md#get_events) | **GET** /fwmgr/entities/events/v1 | Get events entities by ID and optionally version
*FirewallManagementApi* | [**get_firewall_fields**](./FirewallManagementApi.md#get_firewall_fields) | **GET** /fwmgr/entities/firewall-fields/v1 | Get the firewall field specifications by ID
*FirewallManagementApi* | [**get_network_locations**](./FirewallManagementApi.md#get_network_locations) | **GET** /fwmgr/entities/network-locations/v1 | Get a summary of network locations entities by ID
*FirewallManagementApi* | [**get_network_locations_details**](./FirewallManagementApi.md#get_network_locations_details) | **GET** /fwmgr/entities/network-locations-details/v1 | Get network locations entities by ID
*FirewallManagementApi* | [**get_platforms**](./FirewallManagementApi.md#get_platforms) | **GET** /fwmgr/entities/platforms/v1 | Get platforms by ID, e.g., windows or mac or droid
*FirewallManagementApi* | [**get_policy_containers**](./FirewallManagementApi.md#get_policy_containers) | **GET** /fwmgr/entities/policies/v1 | Get policy container entities by policy ID
*FirewallManagementApi* | [**get_rule_groups**](./FirewallManagementApi.md#get_rule_groups) | **GET** /fwmgr/entities/rule-groups/v1 | Get rule group entities by ID. These groups do not contain their rule entities, just the rule IDs in precedence order.
*FirewallManagementApi* | [**get_rules**](./FirewallManagementApi.md#get_rules) | **GET** /fwmgr/entities/rules/v1 | Get rule entities by ID (64-bit unsigned int as decimal string) or Family ID (32-character hexadecimal string)
*FirewallManagementApi* | [**query_events**](./FirewallManagementApi.md#query_events) | **GET** /fwmgr/queries/events/v1 | Find all event IDs matching the query with filter
*FirewallManagementApi* | [**query_firewall_fields**](./FirewallManagementApi.md#query_firewall_fields) | **GET** /fwmgr/queries/firewall-fields/v1 | Get the firewall field specification IDs for the provided platform
*FirewallManagementApi* | [**query_network_locations**](./FirewallManagementApi.md#query_network_locations) | **GET** /fwmgr/queries/network-locations/v1 | Get a list of network location IDs
*FirewallManagementApi* | [**query_platforms**](./FirewallManagementApi.md#query_platforms) | **GET** /fwmgr/queries/platforms/v1 | Get the list of platform names
*FirewallManagementApi* | [**query_policy_rules**](./FirewallManagementApi.md#query_policy_rules) | **GET** /fwmgr/queries/policy-rules/v1 | Find all firewall rule IDs matching the query with filter, and return them in precedence order
*FirewallManagementApi* | [**query_rule_groups**](./FirewallManagementApi.md#query_rule_groups) | **GET** /fwmgr/queries/rule-groups/v1 | Find all rule group IDs matching the query with filter
*FirewallManagementApi* | [**query_rules**](./FirewallManagementApi.md#query_rules) | **GET** /fwmgr/queries/rules/v1 | Find all rule IDs matching the query with filter
*FirewallManagementApi* | [**update_network_locations**](./FirewallManagementApi.md#update_network_locations) | **PATCH** /fwmgr/entities/network-locations/v1 | Updates the network locations provided, and return the ID.
*FirewallManagementApi* | [**update_network_locations_metadata**](./FirewallManagementApi.md#update_network_locations_metadata) | **POST** /fwmgr/entities/network-locations-metadata/v1 | Updates the network locations metadata such as polling_intervals for the cid
*FirewallManagementApi* | [**update_network_locations_precedence**](./FirewallManagementApi.md#update_network_locations_precedence) | **POST** /fwmgr/entities/network-locations-precedence/v1 | Updates the network locations precedence according to the list of ids provided.
*FirewallManagementApi* | [**update_policy_container**](./FirewallManagementApi.md#update_policy_container) | **PUT** /fwmgr/entities/policies/v2 | Update an identified policy container, including local logging functionality.
*FirewallManagementApi* | [**update_policy_container_v1**](./FirewallManagementApi.md#update_policy_container_v1) | **PUT** /fwmgr/entities/policies/v1 | Update an identified policy container. WARNING: This endpoint is deprecated in favor of v2, using this endpoint could disable your local logging setting.
*FirewallManagementApi* | [**update_rule_group**](./FirewallManagementApi.md#update_rule_group) | **PATCH** /fwmgr/entities/rule-groups/v1 | Update name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules
*FirewallManagementApi* | [**update_rule_group_validation**](./FirewallManagementApi.md#update_rule_group_validation) | **PATCH** /fwmgr/entities/rule-groups/validation/v1 | Validates the request of updating name, description, or enabled status of a rule group, or create, edit, delete, or reorder rules
*FirewallManagementApi* | [**upsert_network_locations**](./FirewallManagementApi.md#upsert_network_locations) | **PUT** /fwmgr/entities/network-locations/v1 | Updates the network locations provided, and return the ID.
*FirewallManagementApi* | [**validate_filepath_pattern**](./FirewallManagementApi.md#validate_filepath_pattern) | **POST** /fwmgr/entities/rules/validate-filepath/v1 | Validates that the test pattern matches the executable filepath glob pattern.
*FirewallPoliciesApi* | [**create_firewall_policies**](./FirewallPoliciesApi.md#create_firewall_policies) | **POST** /policy/entities/firewall/v1 | Create Firewall Policies by specifying details about the policy to create
*FirewallPoliciesApi* | [**delete_firewall_policies**](./FirewallPoliciesApi.md#delete_firewall_policies) | **DELETE** /policy/entities/firewall/v1 | Delete a set of Firewall Policies by specifying their IDs
*FirewallPoliciesApi* | [**get_firewall_policies**](./FirewallPoliciesApi.md#get_firewall_policies) | **GET** /policy/entities/firewall/v1 | Retrieve a set of Firewall Policies by specifying their IDs
*FirewallPoliciesApi* | [**perform_firewall_policies_action**](./FirewallPoliciesApi.md#perform_firewall_policies_action) | **POST** /policy/entities/firewall-actions/v1 | Perform the specified action on the Firewall Policies specified in the request
*FirewallPoliciesApi* | [**query_combined_firewall_policies**](./FirewallPoliciesApi.md#query_combined_firewall_policies) | **GET** /policy/combined/firewall/v1 | Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policies which match the filter criteria
*FirewallPoliciesApi* | [**query_combined_firewall_policy_members**](./FirewallPoliciesApi.md#query_combined_firewall_policy_members) | **GET** /policy/combined/firewall-members/v1 | Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*FirewallPoliciesApi* | [**query_firewall_policies**](./FirewallPoliciesApi.md#query_firewall_policies) | **GET** /policy/queries/firewall/v1 | Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policy IDs which match the filter criteria
*FirewallPoliciesApi* | [**query_firewall_policy_members**](./FirewallPoliciesApi.md#query_firewall_policy_members) | **GET** /policy/queries/firewall-members/v1 | Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*FirewallPoliciesApi* | [**set_firewall_policies_precedence**](./FirewallPoliciesApi.md#set_firewall_policies_precedence) | **POST** /policy/entities/firewall-precedence/v1 | Sets the precedence of Firewall Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
*FirewallPoliciesApi* | [**update_firewall_policies**](./FirewallPoliciesApi.md#update_firewall_policies) | **PATCH** /policy/entities/firewall/v1 | Update Firewall Policies by specifying the ID of the policy and details to update
*HostGroupApi* | [**create_host_groups**](./HostGroupApi.md#create_host_groups) | **POST** /devices/entities/host-groups/v1 | Create Host Groups by specifying details about the group to create
*HostGroupApi* | [**delete_host_groups**](./HostGroupApi.md#delete_host_groups) | **DELETE** /devices/entities/host-groups/v1 | Delete a set of Host Groups by specifying their IDs
*HostGroupApi* | [**get_host_groups**](./HostGroupApi.md#get_host_groups) | **GET** /devices/entities/host-groups/v1 | Retrieve a set of Host Groups by specifying their IDs
*HostGroupApi* | [**perform_group_action**](./HostGroupApi.md#perform_group_action) | **POST** /devices/entities/host-group-actions/v1 | Perform the specified action on the Host Groups specified in the request
*HostGroupApi* | [**query_combined_group_members**](./HostGroupApi.md#query_combined_group_members) | **GET** /devices/combined/host-group-members/v1 | Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*HostGroupApi* | [**query_combined_host_groups**](./HostGroupApi.md#query_combined_host_groups) | **GET** /devices/combined/host-groups/v1 | Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Groups which match the filter criteria
*HostGroupApi* | [**query_group_members**](./HostGroupApi.md#query_group_members) | **GET** /devices/queries/host-group-members/v1 | Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*HostGroupApi* | [**query_host_groups**](./HostGroupApi.md#query_host_groups) | **GET** /devices/queries/host-groups/v1 | Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Group IDs which match the filter criteria
*HostGroupApi* | [**update_host_groups**](./HostGroupApi.md#update_host_groups) | **PATCH** /devices/entities/host-groups/v1 | Update Host Groups by specifying the ID of the group and details to update
*HostsApi* | [**entities_period_perform_action**](./HostsApi.md#entities_period_perform_action) | **POST** /devices/entities/group-actions/v1 | Performs the specified action on the provided group IDs.
*HostsApi* | [**get_device_details**](./HostsApi.md#get_device_details) | **GET** /devices/entities/devices/v1 | Deprecated: Please use new GET or POST /devices/entities/devices/v2 endpoints.  This endpoint will be removed on or sometime after February 9, 2023.  Get details on one or more hosts by providing agent IDs (AID). You can get a host's agent IDs (AIDs) from the /devices/queries/devices/v1 endpoint, the Falcon console or the Streaming API
*HostsApi* | [**get_device_details_v2**](./HostsApi.md#get_device_details_v2) | **GET** /devices/entities/devices/v2 | Get details on one or more hosts by providing host IDs as a query parameter.  Supports up to a maximum 100 IDs.
*HostsApi* | [**get_online_state_period_v1**](./HostsApi.md#get_online_state_period_v1) | **GET** /devices/entities/online-state/v1 | Get the online status for one or more hosts by specifying each host’s unique ID. Successful requests return an HTTP 200 response and the status for each host identified by a `state` of `online`, `offline`, or `unknown` for each host, identified by host `id`.  Make a `GET` request to `/devices/queries/devices/v1` to get a list of host IDs.
*HostsApi* | [**perform_action_v2**](./HostsApi.md#perform_action_v2) | **POST** /devices/entities/devices-actions/v2 | Take various actions on the hosts in your environment. Contain or lift containment on a host. Delete or restore a host.
*HostsApi* | [**post_device_details_v2**](./HostsApi.md#post_device_details_v2) | **POST** /devices/entities/devices/v2 | Get details on one or more hosts by providing host IDs in a POST body.  Supports up to a maximum 5000 IDs.
*HostsApi* | [**query_device_login_history**](./HostsApi.md#query_device_login_history) | **POST** /devices/combined/devices/login-history/v1 | Retrieve details about recent login sessions for a set of devices.
*HostsApi* | [**query_devices_by_filter**](./HostsApi.md#query_devices_by_filter) | **GET** /devices/queries/devices/v1 | Search for hosts in your environment by platform, hostname, IP, and other criteria.
*HostsApi* | [**query_devices_by_filter_scroll**](./HostsApi.md#query_devices_by_filter_scroll) | **GET** /devices/queries/devices-scroll/v1 | Search for hosts in your environment by platform, hostname, IP, and other criteria with continuous pagination capability (based on offset pointer which expires after 2 minutes with no maximum limit)
*HostsApi* | [**query_get_network_address_history_v1**](./HostsApi.md#query_get_network_address_history_v1) | **POST** /devices/combined/devices/network-address-history/v1 | Retrieve history of IP and MAC addresses of devices.
*HostsApi* | [**query_hidden_devices**](./HostsApi.md#query_hidden_devices) | **GET** /devices/queries/devices-hidden/v1 | Retrieve hidden hosts that match the provided filter criteria.
*HostsApi* | [**update_device_tags**](./HostsApi.md#update_device_tags) | **PATCH** /devices/entities/devices/tags/v1 | Append or remove one or more Falcon Grouping Tags on one or more hosts.  Tags must be of the form FalconGroupingTags/
*IdentityEntitiesApi* | [**get_sensor_aggregates**](./IdentityEntitiesApi.md#get_sensor_aggregates) | **POST** /identity-protection/aggregates/devices/GET/v1 | Get sensor aggregates as specified via json in request body.
*IdentityEntitiesApi* | [**get_sensor_details**](./IdentityEntitiesApi.md#get_sensor_details) | **POST** /identity-protection/entities/devices/GET/v1 | Get details on one or more sensors by providing device IDs in a POST body. Supports up to a maximum of 5000 IDs.
*IdentityEntitiesApi* | [**query_sensors_by_filter**](./IdentityEntitiesApi.md#query_sensors_by_filter) | **GET** /identity-protection/queries/devices/v1 | Search for sensors in your environment by hostname, IP, and other criteria.
*IdentityProtectionApi* | [**api_period_preempt_period_proxy_period_post_period_graphql**](./IdentityProtectionApi.md#api_period_preempt_period_proxy_period_post_period_graphql) | **POST** /identity-protection/combined/graphql/v1 | Identity Protection GraphQL API. Allows to retrieve entities, timeline activities, identity-based incidents and security assessment. Allows to perform actions on entities and identity-based incidents.
*IncidentsApi* | [**crowd_score**](./IncidentsApi.md#crowd_score) | **GET** /incidents/combined/crowdscores/v1 | Query environment wide CrowdScore and return the entity data
*IncidentsApi* | [**get_behaviors**](./IncidentsApi.md#get_behaviors) | **POST** /incidents/entities/behaviors/GET/v1 | Get details on behaviors by providing behavior IDs
*IncidentsApi* | [**get_incidents**](./IncidentsApi.md#get_incidents) | **POST** /incidents/entities/incidents/GET/v1 | Get details on incidents by providing incident IDs
*IncidentsApi* | [**perform_incident_action**](./IncidentsApi.md#perform_incident_action) | **POST** /incidents/entities/incident-actions/v1 | Perform a set of actions on one or more incidents, such as adding tags or comments or updating the incident name or description
*IncidentsApi* | [**query_behaviors**](./IncidentsApi.md#query_behaviors) | **GET** /incidents/queries/behaviors/v1 | Search for behaviors by providing an FQL filter, sorting, and paging details
*IncidentsApi* | [**query_incidents**](./IncidentsApi.md#query_incidents) | **GET** /incidents/queries/incidents/v1 | Search for incidents by providing an FQL filter, sorting, and paging details
*InstallationTokensApi* | [**audit_events_query**](./InstallationTokensApi.md#audit_events_query) | **GET** /installation-tokens/queries/audit-events/v1 | Search for audit events by providing an FQL filter and paging details.
*InstallationTokensApi* | [**audit_events_read**](./InstallationTokensApi.md#audit_events_read) | **GET** /installation-tokens/entities/audit-events/v1 | Gets the details of one or more audit events by id.
*InstallationTokensApi* | [**customer_settings_read**](./InstallationTokensApi.md#customer_settings_read) | **GET** /installation-tokens/entities/customer-settings/v1 | Check current installation token settings.
*InstallationTokensApi* | [**tokens_create**](./InstallationTokensApi.md#tokens_create) | **POST** /installation-tokens/entities/tokens/v1 | Creates a token.
*InstallationTokensApi* | [**tokens_delete**](./InstallationTokensApi.md#tokens_delete) | **DELETE** /installation-tokens/entities/tokens/v1 | Deletes a token immediately. To revoke a token, use PATCH /installation-tokens/entities/tokens/v1 instead.
*InstallationTokensApi* | [**tokens_query**](./InstallationTokensApi.md#tokens_query) | **GET** /installation-tokens/queries/tokens/v1 | Search for tokens by providing an FQL filter and paging details.
*InstallationTokensApi* | [**tokens_read**](./InstallationTokensApi.md#tokens_read) | **GET** /installation-tokens/entities/tokens/v1 | Gets the details of one or more tokens by id.
*InstallationTokensApi* | [**tokens_update**](./InstallationTokensApi.md#tokens_update) | **PATCH** /installation-tokens/entities/tokens/v1 | Updates one or more tokens. Use this endpoint to edit labels, change expiration, revoke, or restore.
*InstallationTokensSettingsApi* | [**customer_settings_update**](./InstallationTokensSettingsApi.md#customer_settings_update) | **PATCH** /installation-tokens/entities/customer-settings/v1 | Update installation token settings.
*IntelApi* | [**get_intel_actor_entities**](./IntelApi.md#get_intel_actor_entities) | **GET** /intel/entities/actors/v1 | Retrieve specific actors using their actor IDs.
*IntelApi* | [**get_intel_indicator_entities**](./IntelApi.md#get_intel_indicator_entities) | **POST** /intel/entities/indicators/GET/v1 | Retrieve specific indicators using their indicator IDs.
*IntelApi* | [**get_intel_report_entities**](./IntelApi.md#get_intel_report_entities) | **GET** /intel/entities/reports/v1 | Retrieve specific reports using their report IDs.
*IntelApi* | [**get_intel_report_pdf**](./IntelApi.md#get_intel_report_pdf) | **GET** /intel/entities/report-files/v1 | Return a Report PDF attachment
*IntelApi* | [**get_intel_rule_entities**](./IntelApi.md#get_intel_rule_entities) | **GET** /intel/entities/rules/v1 | Retrieve details for rule sets for the specified ids.
*IntelApi* | [**get_intel_rule_file**](./IntelApi.md#get_intel_rule_file) | **GET** /intel/entities/rules-files/v1 | Download earlier rule sets.
*IntelApi* | [**get_latest_intel_rule_file**](./IntelApi.md#get_latest_intel_rule_file) | **GET** /intel/entities/rules-latest-files/v1 | Download the latest rule set.
*IntelApi* | [**get_mitre_report**](./IntelApi.md#get_mitre_report) | **GET** /intel/entities/mitre-reports/v1 | Export Mitre ATT&CK information for a given actor.
*IntelApi* | [**get_vulnerabilities**](./IntelApi.md#get_vulnerabilities) | **POST** /intel/entities/vulnerabilities/GET/v1 | Get vulnerabilities
*IntelApi* | [**post_mitre_attacks**](./IntelApi.md#post_mitre_attacks) | **POST** /intel/entities/mitre/v1 | Retrieves report and observable IDs associated with the given actor and attacks
*IntelApi* | [**query_intel_actor_entities**](./IntelApi.md#query_intel_actor_entities) | **GET** /intel/combined/actors/v1 | Get info about actors that match provided FQL filters.
*IntelApi* | [**query_intel_actor_ids**](./IntelApi.md#query_intel_actor_ids) | **GET** /intel/queries/actors/v1 | Get actor IDs that match provided FQL filters.
*IntelApi* | [**query_intel_indicator_entities**](./IntelApi.md#query_intel_indicator_entities) | **GET** /intel/combined/indicators/v1 | Get info about indicators that match provided FQL filters.
*IntelApi* | [**query_intel_indicator_ids**](./IntelApi.md#query_intel_indicator_ids) | **GET** /intel/queries/indicators/v1 | Get indicators IDs that match provided FQL filters.
*IntelApi* | [**query_intel_report_entities**](./IntelApi.md#query_intel_report_entities) | **GET** /intel/combined/reports/v1 | Get info about reports that match provided FQL filters.
*IntelApi* | [**query_intel_report_ids**](./IntelApi.md#query_intel_report_ids) | **GET** /intel/queries/reports/v1 | Get report IDs that match provided FQL filters.
*IntelApi* | [**query_intel_rule_ids**](./IntelApi.md#query_intel_rule_ids) | **GET** /intel/queries/rules/v1 | Search for rule IDs that match provided filter criteria.
*IntelApi* | [**query_mitre_attacks**](./IntelApi.md#query_mitre_attacks) | **GET** /intel/queries/mitre/v1 | Gets MITRE tactics and techniques for the given actor, returning concatenation of id and tactic and technique ids, example: fancy-bear_TA0011_T1071
*IntelApi* | [**query_vulnerabilities**](./IntelApi.md#query_vulnerabilities) | **GET** /intel/queries/vulnerabilities/v1 | Get vulnerabilities IDs
*InventoriesApi* | [**create_inventory**](./InventoriesApi.md#create_inventory) | **POST** /snapshots/entities/inventories/v1 | Create inventory from data received from snapshot
*IoaExclusionsApi* | [**create_ioa_exclusions_v1**](./IoaExclusionsApi.md#create_ioa_exclusions_v1) | **POST** /policy/entities/ioa-exclusions/v1 | Create the IOA exclusions
*IoaExclusionsApi* | [**delete_ioa_exclusions_v1**](./IoaExclusionsApi.md#delete_ioa_exclusions_v1) | **DELETE** /policy/entities/ioa-exclusions/v1 | Delete the IOA exclusions by id
*IoaExclusionsApi* | [**get_ioa_exclusions_v1**](./IoaExclusionsApi.md#get_ioa_exclusions_v1) | **GET** /policy/entities/ioa-exclusions/v1 | Get a set of IOA Exclusions by specifying their IDs
*IoaExclusionsApi* | [**query_ioa_exclusions_v1**](./IoaExclusionsApi.md#query_ioa_exclusions_v1) | **GET** /policy/queries/ioa-exclusions/v1 | Search for IOA exclusions.
*IoaExclusionsApi* | [**update_ioa_exclusions_v1**](./IoaExclusionsApi.md#update_ioa_exclusions_v1) | **PATCH** /policy/entities/ioa-exclusions/v1 | Update the IOA exclusions
*IocApi* | [**action_period_get_period_v1**](./IocApi.md#action_period_get_period_v1) | **GET** /iocs/entities/actions/v1 | Get Actions by ids.
*IocApi* | [**action_period_query_period_v1**](./IocApi.md#action_period_query_period_v1) | **GET** /iocs/queries/actions/v1 | Query Actions.
*IocApi* | [**get_indicators_report**](./IocApi.md#get_indicators_report) | **POST** /iocs/entities/indicators-reports/v1 | Launch an indicators report creation job
*IocApi* | [**indicator_period_aggregate_period_v1**](./IocApi.md#indicator_period_aggregate_period_v1) | **POST** /iocs/aggregates/indicators/v1 | Get Indicators aggregates as specified via json in the request body.
*IocApi* | [**indicator_period_combined_period_v1**](./IocApi.md#indicator_period_combined_period_v1) | **GET** /iocs/combined/indicator/v1 | Get Combined for Indicators.
*IocApi* | [**indicator_period_create_period_v1**](./IocApi.md#indicator_period_create_period_v1) | **POST** /iocs/entities/indicators/v1 | Create Indicators.
*IocApi* | [**indicator_period_delete_period_v1**](./IocApi.md#indicator_period_delete_period_v1) | **DELETE** /iocs/entities/indicators/v1 | Delete Indicators by ids.
*IocApi* | [**indicator_period_get_period_v1**](./IocApi.md#indicator_period_get_period_v1) | **GET** /iocs/entities/indicators/v1 | Get Indicators by ids.
*IocApi* | [**indicator_period_search_period_v1**](./IocApi.md#indicator_period_search_period_v1) | **GET** /iocs/queries/indicators/v1 | Search for Indicators.
*IocApi* | [**indicator_period_update_period_v1**](./IocApi.md#indicator_period_update_period_v1) | **PATCH** /iocs/entities/indicators/v1 | Update Indicators.
*IocApi* | [**ioc_type_period_query_period_v1**](./IocApi.md#ioc_type_period_query_period_v1) | **GET** /iocs/queries/ioc-types/v1 | Query IOC Types.
*IocApi* | [**platform_period_query_period_v1**](./IocApi.md#platform_period_query_period_v1) | **GET** /iocs/queries/platforms/v1 | Query Platforms.
*IocApi* | [**severity_period_query_period_v1**](./IocApi.md#severity_period_query_period_v1) | **GET** /iocs/queries/severities/v1 | Query Severities.
*IocsApi* | [**devices_count**](./IocsApi.md#devices_count) | **GET** /indicators/aggregates/devices-count/v1 | Number of hosts in your customer account that have observed a given custom IOC
*IocsApi* | [**devices_ran_on**](./IocsApi.md#devices_ran_on) | **GET** /indicators/queries/devices/v1 | Find hosts that have observed a given custom IOC. For details about those hosts, use GET /devices/entities/devices/v1
*IocsApi* | [**entities_period_processes**](./IocsApi.md#entities_period_processes) | **GET** /processes/entities/processes/v1 | For the provided ProcessID retrieve the process details
*IocsApi* | [**processes_ran_on**](./IocsApi.md#processes_ran_on) | **GET** /indicators/queries/processes/v1 | Search for processes associated with a custom IOC
*KubernetesProtectionApi* | [**create_aws_account**](./KubernetesProtectionApi.md#create_aws_account) | **POST** /kubernetes-protection/entities/accounts/aws/v1 | Creates a new AWS account in our system for a customer and generates the installation script
*KubernetesProtectionApi* | [**create_azure_subscription**](./KubernetesProtectionApi.md#create_azure_subscription) | **POST** /kubernetes-protection/entities/accounts/azure/v1 | Creates a new Azure Subscription in our system
*KubernetesProtectionApi* | [**delete_aws_accounts_mixin0**](./KubernetesProtectionApi.md#delete_aws_accounts_mixin0) | **DELETE** /kubernetes-protection/entities/accounts/aws/v1 | Delete AWS accounts.
*KubernetesProtectionApi* | [**delete_azure_subscription**](./KubernetesProtectionApi.md#delete_azure_subscription) | **DELETE** /kubernetes-protection/entities/accounts/azure/v1 | Deletes a new Azure Subscription in our system
*KubernetesProtectionApi* | [**get_aws_accounts_mixin0**](./KubernetesProtectionApi.md#get_aws_accounts_mixin0) | **GET** /kubernetes-protection/entities/accounts/aws/v1 | Provides a list of AWS accounts.
*KubernetesProtectionApi* | [**get_azure_install_script**](./KubernetesProtectionApi.md#get_azure_install_script) | **GET** /kubernetes-protection/entities/user-script/azure/v1 | Provides the script to run for a given tenant id and subscription IDs
*KubernetesProtectionApi* | [**get_azure_tenant_config**](./KubernetesProtectionApi.md#get_azure_tenant_config) | **GET** /kubernetes-protection/entities/config/azure/v1 | Gets the Azure tenant Config
*KubernetesProtectionApi* | [**get_azure_tenant_ids**](./KubernetesProtectionApi.md#get_azure_tenant_ids) | **GET** /kubernetes-protection/entities/tenants/azure/v1 | Provides all the azure subscriptions and tenants
*KubernetesProtectionApi* | [**get_clusters**](./KubernetesProtectionApi.md#get_clusters) | **GET** /kubernetes-protection/entities/kubernetes/clusters/v1 | Provides the clusters acknowledged by the Kubernetes Protection service
*KubernetesProtectionApi* | [**get_combined_cloud_clusters**](./KubernetesProtectionApi.md#get_combined_cloud_clusters) | **GET** /kubernetes-protection/entities/cloud_cluster/v1 | Returns a combined list of provisioned cloud accounts and known kubernetes clusters
*KubernetesProtectionApi* | [**get_helm_values_yaml**](./KubernetesProtectionApi.md#get_helm_values_yaml) | **GET** /kubernetes-protection/entities/integration/agent/v1 | Provides a sample Helm values.yaml file for a customer to install alongside the agent Helm chart
*KubernetesProtectionApi* | [**get_locations**](./KubernetesProtectionApi.md#get_locations) | **GET** /kubernetes-protection/entities/cloud-locations/v1 | Provides the cloud locations acknowledged by the Kubernetes Protection service
*KubernetesProtectionApi* | [**get_static_scripts**](./KubernetesProtectionApi.md#get_static_scripts) | **GET** /kubernetes-protection/entities/gen/scripts/v1 | Gets static bash scripts that are used during registration
*KubernetesProtectionApi* | [**list_azure_accounts**](./KubernetesProtectionApi.md#list_azure_accounts) | **GET** /kubernetes-protection/entities/accounts/azure/v1 | Provides the azure subscriptions registered to Kubernetes Protection
*KubernetesProtectionApi* | [**patch_azure_service_principal**](./KubernetesProtectionApi.md#patch_azure_service_principal) | **PATCH** /kubernetes-protection/entities/service-principal/azure/v1 | Adds the client ID for the given tenant ID to our system
*KubernetesProtectionApi* | [**regenerate_api_key**](./KubernetesProtectionApi.md#regenerate_api_key) | **POST** /kubernetes-protection/entities/integration/api-key/v1 | Regenerate API key for docker registry integrations
*KubernetesProtectionApi* | [**trigger_scan**](./KubernetesProtectionApi.md#trigger_scan) | **POST** /kubernetes-protection/entities/scan/trigger/v1 | Triggers a dry run or a full scan of a customer's kubernetes footprint
*KubernetesProtectionApi* | [**update_aws_account**](./KubernetesProtectionApi.md#update_aws_account) | **PATCH** /kubernetes-protection/entities/accounts/aws/v1 | Updates the AWS account per the query parameters provided
*MalqueryApi* | [**get_mal_query_download_v1**](./MalqueryApi.md#get_mal_query_download_v1) | **GET** /malquery/entities/download-files/v1 | Download a file indexed by MalQuery. Specify the file using its SHA256. Only one file is supported at this time
*MalqueryApi* | [**get_mal_query_entities_samples_fetch_v1**](./MalqueryApi.md#get_mal_query_entities_samples_fetch_v1) | **GET** /malquery/entities/samples-fetch/v1 | Fetch a zip archive with password 'infected' containing the samples. Call this once the /entities/samples-multidownload request has finished processing
*MalqueryApi* | [**get_mal_query_metadata_v1**](./MalqueryApi.md#get_mal_query_metadata_v1) | **GET** /malquery/entities/metadata/v1 | Retrieve indexed files metadata by their hash
*MalqueryApi* | [**get_mal_query_quotas_v1**](./MalqueryApi.md#get_mal_query_quotas_v1) | **GET** /malquery/aggregates/quotas/v1 | Get information about search and download quotas in your environment
*MalqueryApi* | [**get_mal_query_request_v1**](./MalqueryApi.md#get_mal_query_request_v1) | **GET** /malquery/entities/requests/v1 | Check the status and results of an asynchronous request, such as hunt or exact-search. Supports a single request id at this time.
*MalqueryApi* | [**post_mal_query_entities_samples_multidownload_v1**](./MalqueryApi.md#post_mal_query_entities_samples_multidownload_v1) | **POST** /malquery/entities/samples-multidownload/v1 | Schedule samples for download. Use the result id with the /request endpoint to check if the download is ready after which you can call the /entities/samples-fetch to get the zip
*MalqueryApi* | [**post_mal_query_exact_search_v1**](./MalqueryApi.md#post_mal_query_exact_search_v1) | **POST** /malquery/queries/exact-search/v1 | Search Falcon MalQuery for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity. You can filter results on criteria such as file type, file size and first seen date. Returns a request id which can be used with the /request endpoint
*MalqueryApi* | [**post_mal_query_fuzzy_search_v1**](./MalqueryApi.md#post_mal_query_fuzzy_search_v1) | **POST** /malquery/combined/fuzzy-search/v1 | Search Falcon MalQuery quickly, but with more potential for false positives. Search for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity.
*MalqueryApi* | [**post_mal_query_hunt_v1**](./MalqueryApi.md#post_mal_query_hunt_v1) | **POST** /malquery/queries/hunt/v1 | Schedule a YARA-based search for execution. Returns a request id which can be used with the /request endpoint
*MessageCenterApi* | [**aggregate_cases**](./MessageCenterApi.md#aggregate_cases) | **POST** /message-center/aggregates/cases/GET/v1 | Retrieve aggregate case values based on the matched filter
*MessageCenterApi* | [**case_add_activity**](./MessageCenterApi.md#case_add_activity) | **POST** /message-center/entities/case-activity/v1 | Add an activity to case. Only activities of type comment are allowed via API
*MessageCenterApi* | [**case_add_attachment**](./MessageCenterApi.md#case_add_attachment) | **POST** /message-center/entities/case-attachment/v1 | Upload an attachment for the case.
*MessageCenterApi* | [**case_download_attachment**](./MessageCenterApi.md#case_download_attachment) | **GET** /message-center/entities/case-attachment/v1 | retrieves an attachment for the case, given the attachment id
*MessageCenterApi* | [**create_case**](./MessageCenterApi.md#create_case) | **POST** /message-center/entities/case/v1 | create a new case
*MessageCenterApi* | [**create_case_v2**](./MessageCenterApi.md#create_case_v2) | **POST** /message-center/entities/case/v2 | create a new case
*MessageCenterApi* | [**get_case_activity_by_ids**](./MessageCenterApi.md#get_case_activity_by_ids) | **POST** /message-center/entities/case-activities/GET/v1 | Retrieve activities for given id's
*MessageCenterApi* | [**get_case_entities_by_ids**](./MessageCenterApi.md#get_case_entities_by_ids) | **POST** /message-center/entities/cases/GET/v1 | Retrieve message center cases
*MessageCenterApi* | [**query_activity_by_case_id**](./MessageCenterApi.md#query_activity_by_case_id) | **GET** /message-center/queries/case-activities/v1 | Retrieve activities id's for a case
*MessageCenterApi* | [**query_cases_ids_by_filter**](./MessageCenterApi.md#query_cases_ids_by_filter) | **GET** /message-center/queries/cases/v1 | Retrieve case id's that match the provided filter criteria
*MlExclusionsApi* | [**create_ml_exclusions_v1**](./MlExclusionsApi.md#create_ml_exclusions_v1) | **POST** /policy/entities/ml-exclusions/v1 | Create the ML exclusions
*MlExclusionsApi* | [**delete_ml_exclusions_v1**](./MlExclusionsApi.md#delete_ml_exclusions_v1) | **DELETE** /policy/entities/ml-exclusions/v1 | Delete the ML exclusions by id
*MlExclusionsApi* | [**get_ml_exclusions_v1**](./MlExclusionsApi.md#get_ml_exclusions_v1) | **GET** /policy/entities/ml-exclusions/v1 | Get a set of ML Exclusions by specifying their IDs
*MlExclusionsApi* | [**query_ml_exclusions_v1**](./MlExclusionsApi.md#query_ml_exclusions_v1) | **GET** /policy/queries/ml-exclusions/v1 | Search for ML exclusions.
*MlExclusionsApi* | [**update_ml_exclusions_v1**](./MlExclusionsApi.md#update_ml_exclusions_v1) | **PATCH** /policy/entities/ml-exclusions/v1 | Update the ML exclusions
*MobileEnrollmentApi* | [**request_device_enrollment_v3**](./MobileEnrollmentApi.md#request_device_enrollment_v3) | **POST** /enrollments/entities/details/v3 | Trigger on-boarding process for a mobile device
*MsspApi* | [**add_cid_group_members**](./MsspApi.md#add_cid_group_members) | **POST** /mssp/entities/cid-group-members/v1 | Add new CID group member.
*MsspApi* | [**add_role**](./MsspApi.md#add_role) | **POST** /mssp/entities/mssp-roles/v1 | Create a link between user group and CID group, with zero or more additional roles. The call does not replace any existing link between them. User group ID and CID group ID have to be specified in request.
*MsspApi* | [**add_user_group_members**](./MsspApi.md#add_user_group_members) | **POST** /mssp/entities/user-group-members/v1 | Add new user group member. Maximum 500 members allowed per user group.
*MsspApi* | [**create_cid_groups**](./MsspApi.md#create_cid_groups) | **POST** /mssp/entities/cid-groups/v1 | Create new CID groups. Name is a required field but description is an optional field. Maximum 500 CID groups allowed.
*MsspApi* | [**create_user_groups**](./MsspApi.md#create_user_groups) | **POST** /mssp/entities/user-groups/v1 | Create new user groups. Name is a required field but description is an optional field. Maximum 500 user groups allowed per customer.
*MsspApi* | [**delete_cid_group_members**](./MsspApi.md#delete_cid_group_members) | **DELETE** /mssp/entities/cid-group-members/v1 | Deprecated : Please use DELETE /entities/cid-group-members/v2. Delete CID group members.
*MsspApi* | [**delete_cid_group_members_v2**](./MsspApi.md#delete_cid_group_members_v2) | **DELETE** /mssp/entities/cid-group-members/v2 | Delete CID group members. Prevents removal of a cid group a cid group if it is only part of one cid group.
*MsspApi* | [**delete_cid_groups**](./MsspApi.md#delete_cid_groups) | **DELETE** /mssp/entities/cid-groups/v1 | Delete CID groups by ID.
*MsspApi* | [**delete_user_group_members**](./MsspApi.md#delete_user_group_members) | **DELETE** /mssp/entities/user-group-members/v1 | Delete user group members entry.
*MsspApi* | [**delete_user_groups**](./MsspApi.md#delete_user_groups) | **DELETE** /mssp/entities/user-groups/v1 | Delete user groups by ID.
*MsspApi* | [**deleted_roles**](./MsspApi.md#deleted_roles) | **DELETE** /mssp/entities/mssp-roles/v1 | Delete links or additional roles between user groups and CID groups. User group ID and CID group ID have to be specified in request. Only specified roles are removed if specified in request payload, else association between User Group and CID group is dissolved completely (if no roles specified).
*MsspApi* | [**get_children**](./MsspApi.md#get_children) | **GET** /mssp/entities/children/v1 | Get link to child customer by child CID(s)
*MsspApi* | [**get_children_v2**](./MsspApi.md#get_children_v2) | **POST** /mssp/entities/children/GET/v2 | Get link to child customer by child CID(s)
*MsspApi* | [**get_cid_group_by_id**](./MsspApi.md#get_cid_group_by_id) | **GET** /mssp/entities/cid-groups/v1 | Deprecated : Please use GET /mssp/entities/cid-groups/v2. Get CID groups by ID.
*MsspApi* | [**get_cid_group_by_id_v2**](./MsspApi.md#get_cid_group_by_id_v2) | **GET** /mssp/entities/cid-groups/v2 | Get CID Groups by ID.
*MsspApi* | [**get_cid_group_members_by**](./MsspApi.md#get_cid_group_members_by) | **GET** /mssp/entities/cid-group-members/v1 | Deprecated : Please use GET /mssp/entities/cid-group-members/v2. Get CID group members by CID group ID.
*MsspApi* | [**get_cid_group_members_by_v2**](./MsspApi.md#get_cid_group_members_by_v2) | **GET** /mssp/entities/cid-group-members/v2 | Get CID group members by CID Group ID.
*MsspApi* | [**get_roles_by_id**](./MsspApi.md#get_roles_by_id) | **GET** /mssp/entities/mssp-roles/v1 | Get link between user group and CID group by ID. Link ID is a string consisting of multiple components, but should be treated as opaque.
*MsspApi* | [**get_user_group_members_by_id**](./MsspApi.md#get_user_group_members_by_id) | **GET** /mssp/entities/user-group-members/v1 | Deprecated : Please use GET /mssp/entities/user-group-members/v2. Get user group members by user group ID.
*MsspApi* | [**get_user_group_members_by_idv2**](./MsspApi.md#get_user_group_members_by_idv2) | **GET** /mssp/entities/user-group-members/v2 | Get user group members by user group ID.
*MsspApi* | [**get_user_groups_by_id**](./MsspApi.md#get_user_groups_by_id) | **GET** /mssp/entities/user-groups/v1 | Deprecated : Please use GET /entities/user-groups/v2. Get user groups by ID.
*MsspApi* | [**get_user_groups_by_idv2**](./MsspApi.md#get_user_groups_by_idv2) | **GET** /mssp/entities/user-groups/v2 | Get user groups by ID.
*MsspApi* | [**query_children**](./MsspApi.md#query_children) | **GET** /mssp/queries/children/v1 | Query for customers linked as children
*MsspApi* | [**query_cid_group_members**](./MsspApi.md#query_cid_group_members) | **GET** /mssp/queries/cid-group-members/v1 | Query a CID groups members by associated CID.
*MsspApi* | [**query_cid_groups**](./MsspApi.md#query_cid_groups) | **GET** /mssp/queries/cid-groups/v1 | Query CID groups.
*MsspApi* | [**query_roles**](./MsspApi.md#query_roles) | **GET** /mssp/queries/mssp-roles/v1 | Query links between user groups and CID groups. At least one of CID group ID or user group ID should also be provided. Role ID is optional.
*MsspApi* | [**query_user_group_members**](./MsspApi.md#query_user_group_members) | **GET** /mssp/queries/user-group-members/v1 | Query user group member by user UUID.
*MsspApi* | [**query_user_groups**](./MsspApi.md#query_user_groups) | **GET** /mssp/queries/user-groups/v1 | Query user groups.
*MsspApi* | [**update_cid_groups**](./MsspApi.md#update_cid_groups) | **PATCH** /mssp/entities/cid-groups/v1 | Update existing CID groups. CID group ID is expected for each CID group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. CID group member(s) remain unaffected.
*MsspApi* | [**update_user_groups**](./MsspApi.md#update_user_groups) | **PATCH** /mssp/entities/user-groups/v1 | Update existing user group(s). User group ID is expected for each user group definition provided in request body. Name is a required field but description is an optional field. Empty description will override existing value. User group member(s) remain unaffected.
*Oauth2Api* | [**oauth2_access_token**](./Oauth2Api.md#oauth2_access_token) | **POST** /oauth2/token | Generate an OAuth2 access token
*Oauth2Api* | [**oauth2_revoke_token**](./Oauth2Api.md#oauth2_revoke_token) | **POST** /oauth2/revoke | Revoke a previously issued OAuth2 access token before the end of its standard 30-minute lifespan.
*OdsApi* | [**aggregate_query_scan_host_metadata**](./OdsApi.md#aggregate_query_scan_host_metadata) | **POST** /ods/aggregates/scan-hosts/v1 | Get aggregates on ODS scan-hosts data.
*OdsApi* | [**aggregate_scans**](./OdsApi.md#aggregate_scans) | **POST** /ods/aggregates/scans/v1 | Get aggregates on ODS scan data.
*OdsApi* | [**aggregate_scheduled_scans**](./OdsApi.md#aggregate_scheduled_scans) | **POST** /ods/aggregates/scheduled-scans/v1 | Get aggregates on ODS scheduled-scan data.
*OdsApi* | [**cancel_scans**](./OdsApi.md#cancel_scans) | **POST** /ods/entities/scan-control-actions/cancel/v1 | Cancel ODS scans for the given scan ids.
*OdsApi* | [**create_scan**](./OdsApi.md#create_scan) | **POST** /ods/entities/scans/v1 | Create ODS scan and start or schedule scan for the given scan request.
*OdsApi* | [**delete_scheduled_scans**](./OdsApi.md#delete_scheduled_scans) | **DELETE** /ods/entities/scheduled-scans/v1 | Delete ODS scheduled-scans for the given scheduled-scan ids.
*OdsApi* | [**get_malicious_files_by_ids**](./OdsApi.md#get_malicious_files_by_ids) | **GET** /ods/entities/malicious-files/v1 | Get malicious files by ids.
*OdsApi* | [**get_scan_host_metadata_by_ids**](./OdsApi.md#get_scan_host_metadata_by_ids) | **GET** /ods/entities/scan-hosts/v1 | Get scan hosts by ids.
*OdsApi* | [**get_scans_by_scan_ids**](./OdsApi.md#get_scans_by_scan_ids) | **GET** /ods/entities/scans/v1 | Get Scans by IDs.
*OdsApi* | [**get_scheduled_scans_by_scan_ids**](./OdsApi.md#get_scheduled_scans_by_scan_ids) | **GET** /ods/entities/scheduled-scans/v1 | Get ScheduledScans by IDs.
*OdsApi* | [**query_malicious_files**](./OdsApi.md#query_malicious_files) | **GET** /ods/queries/malicious-files/v1 | Query malicious files.
*OdsApi* | [**query_scan_host_metadata**](./OdsApi.md#query_scan_host_metadata) | **GET** /ods/queries/scan-hosts/v1 | Query scan hosts.
*OdsApi* | [**query_scans**](./OdsApi.md#query_scans) | **GET** /ods/queries/scans/v1 | Query Scans.
*OdsApi* | [**query_scheduled_scans**](./OdsApi.md#query_scheduled_scans) | **GET** /ods/queries/scheduled-scans/v1 | Query ScheduledScans.
*OdsApi* | [**schedule_scan**](./OdsApi.md#schedule_scan) | **POST** /ods/entities/scheduled-scans/v1 | Create ODS scan and start or schedule scan for the given scan request.
*OverwatchDashboardApi* | [**aggregates_detections_global_counts**](./OverwatchDashboardApi.md#aggregates_detections_global_counts) | **GET** /overwatch-dashboards/aggregates/detections-global-counts/v1 | Get the total number of detections pushed across all customers
*OverwatchDashboardApi* | [**aggregates_events**](./OverwatchDashboardApi.md#aggregates_events) | **POST** /overwatch-dashboards/aggregates/events/GET/v1 | Get aggregate OverWatch detection event info by providing an aggregate query
*OverwatchDashboardApi* | [**aggregates_events_collections**](./OverwatchDashboardApi.md#aggregates_events_collections) | **POST** /overwatch-dashboards/aggregates/events-collections/GET/v1 | Get OverWatch detection event collection info by providing an aggregate query
*OverwatchDashboardApi* | [**aggregates_incidents_global_counts**](./OverwatchDashboardApi.md#aggregates_incidents_global_counts) | **GET** /overwatch-dashboards/aggregates/incidents-global-counts/v1 | Get the total number of incidents pushed across all customers
*OverwatchDashboardApi* | [**aggregates_ow_events_global_counts**](./OverwatchDashboardApi.md#aggregates_ow_events_global_counts) | **GET** /overwatch-dashboards/aggregates/ow-events-global-counts/v1 | Get the total number of OverWatch events across all customers
*PreventionPoliciesApi* | [**create_prevention_policies**](./PreventionPoliciesApi.md#create_prevention_policies) | **POST** /policy/entities/prevention/v1 | Create Prevention Policies by specifying details about the policy to create
*PreventionPoliciesApi* | [**delete_prevention_policies**](./PreventionPoliciesApi.md#delete_prevention_policies) | **DELETE** /policy/entities/prevention/v1 | Delete a set of Prevention Policies by specifying their IDs
*PreventionPoliciesApi* | [**get_prevention_policies**](./PreventionPoliciesApi.md#get_prevention_policies) | **GET** /policy/entities/prevention/v1 | Retrieve a set of Prevention Policies by specifying their IDs
*PreventionPoliciesApi* | [**perform_prevention_policies_action**](./PreventionPoliciesApi.md#perform_prevention_policies_action) | **POST** /policy/entities/prevention-actions/v1 | Perform the specified action on the Prevention Policies specified in the request
*PreventionPoliciesApi* | [**query_combined_prevention_policies**](./PreventionPoliciesApi.md#query_combined_prevention_policies) | **GET** /policy/combined/prevention/v1 | Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policies which match the filter criteria
*PreventionPoliciesApi* | [**query_combined_prevention_policy_members**](./PreventionPoliciesApi.md#query_combined_prevention_policy_members) | **GET** /policy/combined/prevention-members/v1 | Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*PreventionPoliciesApi* | [**query_prevention_policies**](./PreventionPoliciesApi.md#query_prevention_policies) | **GET** /policy/queries/prevention/v1 | Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policy IDs which match the filter criteria
*PreventionPoliciesApi* | [**query_prevention_policy_members**](./PreventionPoliciesApi.md#query_prevention_policy_members) | **GET** /policy/queries/prevention-members/v1 | Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*PreventionPoliciesApi* | [**set_prevention_policies_precedence**](./PreventionPoliciesApi.md#set_prevention_policies_precedence) | **POST** /policy/entities/prevention-precedence/v1 | Sets the precedence of Prevention Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
*PreventionPoliciesApi* | [**update_prevention_policies**](./PreventionPoliciesApi.md#update_prevention_policies) | **PATCH** /policy/entities/prevention/v1 | Update Prevention Policies by specifying the ID of the policy and details to update
*ProvisionApi* | [**get_credentials_mixin0**](./ProvisionApi.md#get_credentials_mixin0) | **GET** /snapshots/entities/image-registry-credentials/v1 | Gets the registry credentials
*QuarantineApi* | [**action_update_count**](./QuarantineApi.md#action_update_count) | **GET** /quarantine/aggregates/action-update-count/v1 | Returns count of potentially affected quarantined files for each action.
*QuarantineApi* | [**get_aggregate_files**](./QuarantineApi.md#get_aggregate_files) | **POST** /quarantine/aggregates/quarantined-files/GET/v1 | Get quarantine file aggregates as specified via json in request body.
*QuarantineApi* | [**get_quarantine_files**](./QuarantineApi.md#get_quarantine_files) | **POST** /quarantine/entities/quarantined-files/GET/v1 | Get quarantine file metadata for specified ids.
*QuarantineApi* | [**query_quarantine_files**](./QuarantineApi.md#query_quarantine_files) | **GET** /quarantine/queries/quarantined-files/v1 | Get quarantine file ids that match the provided filter criteria.
*QuarantineApi* | [**update_qf_by_query**](./QuarantineApi.md#update_qf_by_query) | **PATCH** /quarantine/queries/quarantined-files/v1 | Apply quarantine file actions by query.
*QuarantineApi* | [**update_quarantined_detects_by_ids**](./QuarantineApi.md#update_quarantined_detects_by_ids) | **PATCH** /quarantine/entities/quarantined-files/v1 | Apply action by quarantine file ids
*QuickScanApi* | [**get_scans**](./QuickScanApi.md#get_scans) | **GET** /scanner/entities/scans/v1 | Check the status of a volume scan. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute
*QuickScanApi* | [**get_scans_aggregates**](./QuickScanApi.md#get_scans_aggregates) | **POST** /scanner/aggregates/scans/GET/v1 | Get scans aggregations as specified via json in request body.
*QuickScanApi* | [**query_submissions_mixin0**](./QuickScanApi.md#query_submissions_mixin0) | **GET** /scanner/queries/scans/v1 | Find IDs for submitted scans by providing an FQL filter and paging details. Returns a set of volume IDs that match your criteria.
*QuickScanApi* | [**scan_samples**](./QuickScanApi.md#scan_samples) | **POST** /scanner/entities/scans/v1 | Submit a volume of files for ml scanning. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute
*RealTimeResponseApi* | [**batch_active_responder_cmd**](./RealTimeResponseApi.md#batch_active_responder_cmd) | **POST** /real-time-response/combined/batch-active-responder-command/v1 | Batch executes a RTR active-responder command across the hosts mapped to the given batch ID.
*RealTimeResponseApi* | [**batch_cmd**](./RealTimeResponseApi.md#batch_cmd) | **POST** /real-time-response/combined/batch-command/v1 | Batch executes a RTR read-only command across the hosts mapped to the given batch ID.
*RealTimeResponseApi* | [**batch_get_cmd**](./RealTimeResponseApi.md#batch_get_cmd) | **POST** /real-time-response/combined/batch-get-command/v1 | Batch executes `get` command across hosts to retrieve files. After this call is made `GET /real-time-response/combined/batch-get-command/v1` is used to query for the results.
*RealTimeResponseApi* | [**batch_get_cmd_status**](./RealTimeResponseApi.md#batch_get_cmd_status) | **GET** /real-time-response/combined/batch-get-command/v1 | Retrieves the status of the specified batch get command.  Will return successful files when they are finished processing.
*RealTimeResponseApi* | [**batch_init_sessions**](./RealTimeResponseApi.md#batch_init_sessions) | **POST** /real-time-response/combined/batch-init-session/v1 | Batch initialize a RTR session on multiple hosts.  Before any RTR commands can be used, an active session is needed on the host.
*RealTimeResponseApi* | [**batch_refresh_sessions**](./RealTimeResponseApi.md#batch_refresh_sessions) | **POST** /real-time-response/combined/batch-refresh-session/v1 | Batch refresh a RTR session on multiple hosts. RTR sessions will expire after 10 minutes unless refreshed.
*RealTimeResponseApi* | [**r_tr_aggregate_sessions**](./RealTimeResponseApi.md#r_tr_aggregate_sessions) | **POST** /real-time-response/aggregates/sessions/GET/v1 | Get aggregates on session data.
*RealTimeResponseApi* | [**r_tr_check_active_responder_command_status**](./RealTimeResponseApi.md#r_tr_check_active_responder_command_status) | **GET** /real-time-response/entities/active-responder-command/v1 | Get status of an executed active-responder command on a single host.
*RealTimeResponseApi* | [**r_tr_check_command_status**](./RealTimeResponseApi.md#r_tr_check_command_status) | **GET** /real-time-response/entities/command/v1 | Get status of an executed command on a single host.
*RealTimeResponseApi* | [**r_tr_delete_file**](./RealTimeResponseApi.md#r_tr_delete_file) | **DELETE** /real-time-response/entities/file/v1 | Delete a RTR session file.
*RealTimeResponseApi* | [**r_tr_delete_file_v2**](./RealTimeResponseApi.md#r_tr_delete_file_v2) | **DELETE** /real-time-response/entities/file/v2 | Delete a RTR session file.
*RealTimeResponseApi* | [**r_tr_delete_queued_session**](./RealTimeResponseApi.md#r_tr_delete_queued_session) | **DELETE** /real-time-response/entities/queued-sessions/command/v1 | Delete a queued session command
*RealTimeResponseApi* | [**r_tr_delete_session**](./RealTimeResponseApi.md#r_tr_delete_session) | **DELETE** /real-time-response/entities/sessions/v1 | Delete a session.
*RealTimeResponseApi* | [**r_tr_execute_active_responder_command**](./RealTimeResponseApi.md#r_tr_execute_active_responder_command) | **POST** /real-time-response/entities/active-responder-command/v1 | Execute an active responder command on a single host.
*RealTimeResponseApi* | [**r_tr_execute_command**](./RealTimeResponseApi.md#r_tr_execute_command) | **POST** /real-time-response/entities/command/v1 | Execute a command on a single host.
*RealTimeResponseApi* | [**r_tr_get_extracted_file_contents**](./RealTimeResponseApi.md#r_tr_get_extracted_file_contents) | **GET** /real-time-response/entities/extracted-file-contents/v1 | Get RTR extracted file contents for specified session and sha256.
*RealTimeResponseApi* | [**r_tr_init_session**](./RealTimeResponseApi.md#r_tr_init_session) | **POST** /real-time-response/entities/sessions/v1 | Initialize a new session with the RTR cloud.
*RealTimeResponseApi* | [**r_tr_list_all_sessions**](./RealTimeResponseApi.md#r_tr_list_all_sessions) | **GET** /real-time-response/queries/sessions/v1 | Get a list of session_ids.
*RealTimeResponseApi* | [**r_tr_list_files**](./RealTimeResponseApi.md#r_tr_list_files) | **GET** /real-time-response/entities/file/v1 | Get a list of files for the specified RTR session.
*RealTimeResponseApi* | [**r_tr_list_files_v2**](./RealTimeResponseApi.md#r_tr_list_files_v2) | **GET** /real-time-response/entities/file/v2 | Get a list of files for the specified RTR session.
*RealTimeResponseApi* | [**r_tr_list_queued_sessions**](./RealTimeResponseApi.md#r_tr_list_queued_sessions) | **POST** /real-time-response/entities/queued-sessions/GET/v1 | Get queued session metadata by session ID.
*RealTimeResponseApi* | [**r_tr_list_sessions**](./RealTimeResponseApi.md#r_tr_list_sessions) | **POST** /real-time-response/entities/sessions/GET/v1 | Get session metadata by session id.
*RealTimeResponseApi* | [**r_tr_pulse_session**](./RealTimeResponseApi.md#r_tr_pulse_session) | **POST** /real-time-response/entities/refresh-session/v1 | Refresh a session timeout on a single host.
*RealTimeResponseAdminApi* | [**batch_admin_cmd**](./RealTimeResponseAdminApi.md#batch_admin_cmd) | **POST** /real-time-response/combined/batch-admin-command/v1 | Batch executes a RTR administrator command across the hosts mapped to the given batch ID.
*RealTimeResponseAdminApi* | [**r_tr_check_admin_command_status**](./RealTimeResponseAdminApi.md#r_tr_check_admin_command_status) | **GET** /real-time-response/entities/admin-command/v1 | Get status of an executed RTR administrator command on a single host.
*RealTimeResponseAdminApi* | [**r_tr_create_put_files**](./RealTimeResponseAdminApi.md#r_tr_create_put_files) | **POST** /real-time-response/entities/put-files/v1 | Upload a new put-file to use for the RTR `put` command.
*RealTimeResponseAdminApi* | [**r_tr_create_scripts**](./RealTimeResponseAdminApi.md#r_tr_create_scripts) | **POST** /real-time-response/entities/scripts/v1 | Upload a new custom-script to use for the RTR `runscript` command.
*RealTimeResponseAdminApi* | [**r_tr_delete_put_files**](./RealTimeResponseAdminApi.md#r_tr_delete_put_files) | **DELETE** /real-time-response/entities/put-files/v1 | Delete a put-file based on the ID given.  Can only delete one file at a time.
*RealTimeResponseAdminApi* | [**r_tr_delete_scripts**](./RealTimeResponseAdminApi.md#r_tr_delete_scripts) | **DELETE** /real-time-response/entities/scripts/v1 | Delete a custom-script based on the ID given.  Can only delete one script at a time.
*RealTimeResponseAdminApi* | [**r_tr_execute_admin_command**](./RealTimeResponseAdminApi.md#r_tr_execute_admin_command) | **POST** /real-time-response/entities/admin-command/v1 | Execute a RTR administrator command on a single host.
*RealTimeResponseAdminApi* | [**r_tr_get_put_files**](./RealTimeResponseAdminApi.md#r_tr_get_put_files) | **GET** /real-time-response/entities/put-files/v1 | Get put-files based on the ID's given. These are used for the RTR `put` command.
*RealTimeResponseAdminApi* | [**r_tr_get_put_files_v2**](./RealTimeResponseAdminApi.md#r_tr_get_put_files_v2) | **GET** /real-time-response/entities/put-files/v2 | Get put-files based on the ID's given. These are used for the RTR `put` command.
*RealTimeResponseAdminApi* | [**r_tr_get_scripts**](./RealTimeResponseAdminApi.md#r_tr_get_scripts) | **GET** /real-time-response/entities/scripts/v1 | Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.
*RealTimeResponseAdminApi* | [**r_tr_get_scripts_v2**](./RealTimeResponseAdminApi.md#r_tr_get_scripts_v2) | **GET** /real-time-response/entities/scripts/v2 | Get custom-scripts based on the ID's given. These are used for the RTR `runscript` command.
*RealTimeResponseAdminApi* | [**r_tr_list_put_files**](./RealTimeResponseAdminApi.md#r_tr_list_put_files) | **GET** /real-time-response/queries/put-files/v1 | Get a list of put-file ID's that are available to the user for the `put` command.
*RealTimeResponseAdminApi* | [**r_tr_list_scripts**](./RealTimeResponseAdminApi.md#r_tr_list_scripts) | **GET** /real-time-response/queries/scripts/v1 | Get a list of custom-script ID's that are available to the user for the `runscript` command.
*RealTimeResponseAdminApi* | [**r_tr_update_scripts**](./RealTimeResponseAdminApi.md#r_tr_update_scripts) | **PATCH** /real-time-response/entities/scripts/v1 | Upload a new scripts to replace an existing one.
*ReconApi* | [**aggregate_notifications_exposed_data_records_v1**](./ReconApi.md#aggregate_notifications_exposed_data_records_v1) | **POST** /recon/aggregates/notifications-exposed-data-records/GET/v1 | Get notification exposed data record aggregates as specified via JSON in request body. The valid aggregation fields are: [cid notification_id created_date rule.id rule.name rule.topic source_category site author file.name credential_status]
*ReconApi* | [**aggregate_notifications_v1**](./ReconApi.md#aggregate_notifications_v1) | **POST** /recon/aggregates/notifications/GET/v1 | Get notification aggregates as specified via JSON in request body.
*ReconApi* | [**create_actions_v1**](./ReconApi.md#create_actions_v1) | **POST** /recon/entities/actions/v1 | Create actions for a monitoring rule. Accepts a list of actions that will be attached to the monitoring rule.
*ReconApi* | [**create_export_jobs_v1**](./ReconApi.md#create_export_jobs_v1) | **POST** /recon/entities/exports/v1 | Launch asynchronous export job. Use the job ID to poll the status of the job using GET /entities/exports/v1.
*ReconApi* | [**create_rules_v1**](./ReconApi.md#create_rules_v1) | **POST** /recon/entities/rules/v1 | Create monitoring rules.
*ReconApi* | [**delete_action_v1**](./ReconApi.md#delete_action_v1) | **DELETE** /recon/entities/actions/v1 | Delete an action from a monitoring rule based on the action ID.
*ReconApi* | [**delete_export_jobs_v1**](./ReconApi.md#delete_export_jobs_v1) | **DELETE** /recon/entities/exports/v1 | Delete export jobs (and their associated file(s)) based on their IDs.
*ReconApi* | [**delete_notifications_v1**](./ReconApi.md#delete_notifications_v1) | **DELETE** /recon/entities/notifications/v1 | Delete notifications based on IDs. Notifications cannot be recovered after they are deleted.
*ReconApi* | [**delete_rules_v1**](./ReconApi.md#delete_rules_v1) | **DELETE** /recon/entities/rules/v1 | Delete monitoring rules.
*ReconApi* | [**get_actions_v1**](./ReconApi.md#get_actions_v1) | **GET** /recon/entities/actions/v1 | Get actions based on their IDs. IDs can be retrieved using the GET /queries/actions/v1 endpoint.
*ReconApi* | [**get_export_jobs_v1**](./ReconApi.md#get_export_jobs_v1) | **GET** /recon/entities/exports/v1 | Get the status of export jobs based on their IDs. Export jobs can be launched by calling POST /entities/exports/v1. When a job is complete, use the job ID to download the file(s) associated with it using GET entities/export-files/v1.
*ReconApi* | [**get_file_content_for_export_jobs_v1**](./ReconApi.md#get_file_content_for_export_jobs_v1) | **GET** /recon/entities/export-files/v1 | Download the file associated with a job ID.
*ReconApi* | [**get_notifications_detailed_translated_v1**](./ReconApi.md#get_notifications_detailed_translated_v1) | **GET** /recon/entities/notifications-detailed-translated/v1 | Get detailed notifications based on their IDs. These include the raw intelligence content that generated the match.This endpoint will return translated notification content. The only target language available is English. A single notification can be translated per request
*ReconApi* | [**get_notifications_detailed_v1**](./ReconApi.md#get_notifications_detailed_v1) | **GET** /recon/entities/notifications-detailed/v1 | Get detailed notifications based on their IDs. These include the raw intelligence content that generated the match.
*ReconApi* | [**get_notifications_exposed_data_records_v1**](./ReconApi.md#get_notifications_exposed_data_records_v1) | **GET** /recon/entities/notifications-exposed-data-records/v1 | Get notifications exposed data records based on their IDs. IDs can be retrieved using the GET /queries/notifications-exposed-data-records/v1 endpoint. The associate notification can be fetched using the /entities/notifications/v* endpoints
*ReconApi* | [**get_notifications_translated_v1**](./ReconApi.md#get_notifications_translated_v1) | **GET** /recon/entities/notifications-translated/v1 | Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint. This endpoint will return translated notification content. The only target language available is English.
*ReconApi* | [**get_notifications_v1**](./ReconApi.md#get_notifications_v1) | **GET** /recon/entities/notifications/v1 | Get notifications based on their IDs. IDs can be retrieved using the GET /queries/notifications/v1 endpoint.
*ReconApi* | [**get_rules_v1**](./ReconApi.md#get_rules_v1) | **GET** /recon/entities/rules/v1 | Get monitoring rules based on their IDs. IDs can be retrieved using the GET /queries/rules/v1 endpoint.
*ReconApi* | [**preview_rule_v1**](./ReconApi.md#preview_rule_v1) | **POST** /recon/aggregates/rules-preview/GET/v1 | Preview rules notification count and distribution. This will return aggregations on: channel, count, site.
*ReconApi* | [**query_actions_v1**](./ReconApi.md#query_actions_v1) | **GET** /recon/queries/actions/v1 | Query actions based on provided criteria. Use the IDs from this response to get the action entities on GET /entities/actions/v1.
*ReconApi* | [**query_notifications_exposed_data_records_v1**](./ReconApi.md#query_notifications_exposed_data_records_v1) | **GET** /recon/queries/notifications-exposed-data-records/v1 | Query notifications exposed data records based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications-exposed-data-records/v1
*ReconApi* | [**query_notifications_v1**](./ReconApi.md#query_notifications_v1) | **GET** /recon/queries/notifications/v1 | Query notifications based on provided criteria. Use the IDs from this response to get the notification +entities on GET /entities/notifications/v1, GET /entities/notifications-detailed/v1, +GET /entities/notifications-translated/v1 or GET /entities/notifications-detailed-translated/v1.
*ReconApi* | [**query_rules_v1**](./ReconApi.md#query_rules_v1) | **GET** /recon/queries/rules/v1 | Query monitoring rules based on provided criteria. Use the IDs from this response to fetch the rules on /entities/rules/v1.
*ReconApi* | [**update_action_v1**](./ReconApi.md#update_action_v1) | **PATCH** /recon/entities/actions/v1 | Update an action for a monitoring rule.
*ReconApi* | [**update_notifications_v1**](./ReconApi.md#update_notifications_v1) | **PATCH** /recon/entities/notifications/v1 | Update notification status or assignee. Accepts bulk requests
*ReconApi* | [**update_rules_v1**](./ReconApi.md#update_rules_v1) | **PATCH** /recon/entities/rules/v1 | Update monitoring rules.
*RegistrationApi* | [**register_cspm_snapshot_account**](./RegistrationApi.md#register_cspm_snapshot_account) | **POST** /snapshots/entities/accounts/v1 | Register customer cloud account for snapshot scanning
*ReportExecutionsApi* | [**report_executions_download_period_get**](./ReportExecutionsApi.md#report_executions_download_period_get) | **GET** /reports/entities/report-executions-download/v1 | Get report entity download
*ReportExecutionsApi* | [**report_executions_period_get**](./ReportExecutionsApi.md#report_executions_period_get) | **GET** /reports/entities/report-executions/v1 | Retrieve report details for the provided report IDs.
*ReportExecutionsApi* | [**report_executions_period_query**](./ReportExecutionsApi.md#report_executions_period_query) | **GET** /reports/queries/report-executions/v1 | Find all report execution IDs matching the query with filter
*ReportExecutionsApi* | [**report_executions_period_retry**](./ReportExecutionsApi.md#report_executions_period_retry) | **POST** /reports/entities/report-executions-retry/v1 | This endpoint will be used to retry report executions
*ResponsePoliciesApi* | [**create_rt_response_policies**](./ResponsePoliciesApi.md#create_rt_response_policies) | **POST** /policy/entities/response/v1 | Create Response Policies by specifying details about the policy to create
*ResponsePoliciesApi* | [**delete_rt_response_policies**](./ResponsePoliciesApi.md#delete_rt_response_policies) | **DELETE** /policy/entities/response/v1 | Delete a set of Response Policies by specifying their IDs
*ResponsePoliciesApi* | [**get_rt_response_policies**](./ResponsePoliciesApi.md#get_rt_response_policies) | **GET** /policy/entities/response/v1 | Retrieve a set of Response Policies by specifying their IDs
*ResponsePoliciesApi* | [**perform_rt_response_policies_action**](./ResponsePoliciesApi.md#perform_rt_response_policies_action) | **POST** /policy/entities/response-actions/v1 | Perform the specified action on the Response Policies specified in the request
*ResponsePoliciesApi* | [**query_combined_rt_response_policies**](./ResponsePoliciesApi.md#query_combined_rt_response_policies) | **GET** /policy/combined/response/v1 | Search for Response Policies in your environment by providing an FQL filter and paging details. Returns a set of Response Policies which match the filter criteria
*ResponsePoliciesApi* | [**query_combined_rt_response_policy_members**](./ResponsePoliciesApi.md#query_combined_rt_response_policy_members) | **GET** /policy/combined/response-members/v1 | Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*ResponsePoliciesApi* | [**query_rt_response_policies**](./ResponsePoliciesApi.md#query_rt_response_policies) | **GET** /policy/queries/response/v1 | Search for Response Policies in your environment by providing an FQL filter with sort and/or paging details. This returns a set of Response Policy IDs that match the given criteria.
*ResponsePoliciesApi* | [**query_rt_response_policy_members**](./ResponsePoliciesApi.md#query_rt_response_policy_members) | **GET** /policy/queries/response-members/v1 | Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*ResponsePoliciesApi* | [**set_rt_response_policies_precedence**](./ResponsePoliciesApi.md#set_rt_response_policies_precedence) | **POST** /policy/entities/response-precedence/v1 | Sets the precedence of Response Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
*ResponsePoliciesApi* | [**update_rt_response_policies**](./ResponsePoliciesApi.md#update_rt_response_policies) | **PATCH** /policy/entities/response/v1 | Update Response Policies by specifying the ID of the policy and details to update
*SampleUploadsApi* | [**archive_delete_v1**](./SampleUploadsApi.md#archive_delete_v1) | **DELETE** /archives/entities/archives/v1 | Delete an archive that was uploaded previously
*SampleUploadsApi* | [**archive_get_v1**](./SampleUploadsApi.md#archive_get_v1) | **GET** /archives/entities/archives/v1 | Retrieves the archives upload operation statuses. Status `done` means that archive was processed successfully. Status `error` means that archive was not processed successfully.
*SampleUploadsApi* | [**archive_list_v1**](./SampleUploadsApi.md#archive_list_v1) | **GET** /archives/entities/archive-files/v1 | Retrieves the archives files in chunks.
*SampleUploadsApi* | [**archive_upload_v1**](./SampleUploadsApi.md#archive_upload_v1) | **POST** /archives/entities/archives/v1 | Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis. This method is deprecated in favor of `/archives/entities/archives/v2`
*SampleUploadsApi* | [**archive_upload_v2**](./SampleUploadsApi.md#archive_upload_v2) | **POST** /archives/entities/archives/v2 | Uploads an archive and extracts files list from it. Operation is asynchronous use `/archives/entities/archives/v1` to check the status. After uploading, use `/archives/entities/extractions/v1` to copy the file to internal storage making it available for content analysis.
*SampleUploadsApi* | [**delete_sample_v3**](./SampleUploadsApi.md#delete_sample_v3) | **DELETE** /samples/entities/samples/v3 | Removes a sample, including file, meta and submissions from the collection
*SampleUploadsApi* | [**extraction_create_v1**](./SampleUploadsApi.md#extraction_create_v1) | **POST** /archives/entities/extractions/v1 | Extracts files from an uploaded archive and copies them to internal storage making it available for content analysis.
*SampleUploadsApi* | [**extraction_get_v1**](./SampleUploadsApi.md#extraction_get_v1) | **GET** /archives/entities/extractions/v1 | Retrieves the files extraction operation statuses. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.
*SampleUploadsApi* | [**extraction_list_v1**](./SampleUploadsApi.md#extraction_list_v1) | **GET** /archives/entities/extraction-files/v1 | Retrieves the files extractions in chunks. Status `done` means that all files were processed successfully. Status `error` means that at least one of the file could not be processed.
*SampleUploadsApi* | [**get_sample_v3**](./SampleUploadsApi.md#get_sample_v3) | **GET** /samples/entities/samples/v3 | Retrieves the file associated with the given ID (SHA256)
*SampleUploadsApi* | [**upload_sample_v3**](./SampleUploadsApi.md#upload_sample_v3) | **POST** /samples/entities/samples/v3 | Upload a file for further cloud analysis. After uploading, call the specific analysis API endpoint.
*ScheduledReportsApi* | [**scheduled_reports_period_get**](./ScheduledReportsApi.md#scheduled_reports_period_get) | **GET** /reports/entities/scheduled-reports/v1 | Retrieve scheduled reports for the provided report IDs.
*ScheduledReportsApi* | [**scheduled_reports_period_launch**](./ScheduledReportsApi.md#scheduled_reports_period_launch) | **POST** /reports/entities/scheduled-reports/execution/v1 | Launch scheduled reports executions for the provided report IDs.
*ScheduledReportsApi* | [**scheduled_reports_period_query**](./ScheduledReportsApi.md#scheduled_reports_period_query) | **GET** /reports/queries/scheduled-reports/v1 | Find all report IDs matching the query with filter
*SensorDownloadApi* | [**download_sensor_installer_by_id**](./SensorDownloadApi.md#download_sensor_installer_by_id) | **GET** /sensors/entities/download-installer/v1 | Download sensor installer by SHA256 ID
*SensorDownloadApi* | [**get_combined_sensor_installers_by_query**](./SensorDownloadApi.md#get_combined_sensor_installers_by_query) | **GET** /sensors/combined/installers/v1 | Get sensor installer details by provided query
*SensorDownloadApi* | [**get_sensor_installers_by_query**](./SensorDownloadApi.md#get_sensor_installers_by_query) | **GET** /sensors/queries/installers/v1 | Get sensor installer IDs by provided query
*SensorDownloadApi* | [**get_sensor_installers_ccidby_query**](./SensorDownloadApi.md#get_sensor_installers_ccidby_query) | **GET** /sensors/queries/installers/ccid/v1 | Get CCID to use with sensor installers
*SensorDownloadApi* | [**get_sensor_installers_entities**](./SensorDownloadApi.md#get_sensor_installers_entities) | **GET** /sensors/entities/installers/v1 | Get sensor installer details by provided SHA256 IDs
*SensorUpdatePoliciesApi* | [**create_sensor_update_policies**](./SensorUpdatePoliciesApi.md#create_sensor_update_policies) | **POST** /policy/entities/sensor-update/v1 | Create Sensor Update Policies by specifying details about the policy to create
*SensorUpdatePoliciesApi* | [**create_sensor_update_policies_v2**](./SensorUpdatePoliciesApi.md#create_sensor_update_policies_v2) | **POST** /policy/entities/sensor-update/v2 | Create Sensor Update Policies by specifying details about the policy to create with additional support for uninstall protection
*SensorUpdatePoliciesApi* | [**delete_sensor_update_policies**](./SensorUpdatePoliciesApi.md#delete_sensor_update_policies) | **DELETE** /policy/entities/sensor-update/v1 | Delete a set of Sensor Update Policies by specifying their IDs
*SensorUpdatePoliciesApi* | [**get_sensor_update_policies**](./SensorUpdatePoliciesApi.md#get_sensor_update_policies) | **GET** /policy/entities/sensor-update/v1 | Retrieve a set of Sensor Update Policies by specifying their IDs
*SensorUpdatePoliciesApi* | [**get_sensor_update_policies_v2**](./SensorUpdatePoliciesApi.md#get_sensor_update_policies_v2) | **GET** /policy/entities/sensor-update/v2 | Retrieve a set of Sensor Update Policies with additional support for uninstall protection by specifying their IDs
*SensorUpdatePoliciesApi* | [**perform_sensor_update_policies_action**](./SensorUpdatePoliciesApi.md#perform_sensor_update_policies_action) | **POST** /policy/entities/sensor-update-actions/v1 | Perform the specified action on the Sensor Update Policies specified in the request
*SensorUpdatePoliciesApi* | [**query_combined_sensor_update_builds**](./SensorUpdatePoliciesApi.md#query_combined_sensor_update_builds) | **GET** /policy/combined/sensor-update-builds/v1 | Retrieve available builds for use with Sensor Update Policies
*SensorUpdatePoliciesApi* | [**query_combined_sensor_update_kernels**](./SensorUpdatePoliciesApi.md#query_combined_sensor_update_kernels) | **GET** /policy/combined/sensor-update-kernels/v1 | Retrieve kernel compatibility info for Sensor Update Builds
*SensorUpdatePoliciesApi* | [**query_combined_sensor_update_policies**](./SensorUpdatePoliciesApi.md#query_combined_sensor_update_policies) | **GET** /policy/combined/sensor-update/v1 | Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria
*SensorUpdatePoliciesApi* | [**query_combined_sensor_update_policies_v2**](./SensorUpdatePoliciesApi.md#query_combined_sensor_update_policies_v2) | **GET** /policy/combined/sensor-update/v2 | Search for Sensor Update Policies with additional support for uninstall protection in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria
*SensorUpdatePoliciesApi* | [**query_combined_sensor_update_policy_members**](./SensorUpdatePoliciesApi.md#query_combined_sensor_update_policy_members) | **GET** /policy/combined/sensor-update-members/v1 | Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
*SensorUpdatePoliciesApi* | [**query_sensor_update_kernels_distinct**](./SensorUpdatePoliciesApi.md#query_sensor_update_kernels_distinct) | **GET** /policy/queries/sensor-update-kernels/{distinct-field}/v1 | Retrieve kernel compatibility info for Sensor Update Builds
*SensorUpdatePoliciesApi* | [**query_sensor_update_policies**](./SensorUpdatePoliciesApi.md#query_sensor_update_policies) | **GET** /policy/queries/sensor-update/v1 | Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policy IDs which match the filter criteria
*SensorUpdatePoliciesApi* | [**query_sensor_update_policy_members**](./SensorUpdatePoliciesApi.md#query_sensor_update_policy_members) | **GET** /policy/queries/sensor-update-members/v1 | Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
*SensorUpdatePoliciesApi* | [**reveal_uninstall_token**](./SensorUpdatePoliciesApi.md#reveal_uninstall_token) | **POST** /policy/combined/reveal-uninstall-token/v1 | Reveals an uninstall token for a specific device. To retrieve the bulk maintenance token pass the value 'MAINTENANCE' as the value for 'device_id'
*SensorUpdatePoliciesApi* | [**set_sensor_update_policies_precedence**](./SensorUpdatePoliciesApi.md#set_sensor_update_policies_precedence) | **POST** /policy/entities/sensor-update-precedence/v1 | Sets the precedence of Sensor Update Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
*SensorUpdatePoliciesApi* | [**update_sensor_update_policies**](./SensorUpdatePoliciesApi.md#update_sensor_update_policies) | **PATCH** /policy/entities/sensor-update/v1 | Update Sensor Update Policies by specifying the ID of the policy and details to update
*SensorUpdatePoliciesApi* | [**update_sensor_update_policies_v2**](./SensorUpdatePoliciesApi.md#update_sensor_update_policies_v2) | **PATCH** /policy/entities/sensor-update/v2 | Update Sensor Update Policies by specifying the ID of the policy and details to update with additional support for uninstall protection
*SensorVisibilityExclusionsApi* | [**create_sv_exclusions_v1**](./SensorVisibilityExclusionsApi.md#create_sv_exclusions_v1) | **POST** /policy/entities/sv-exclusions/v1 | Create the sensor visibility exclusions
*SensorVisibilityExclusionsApi* | [**delete_sensor_visibility_exclusions_v1**](./SensorVisibilityExclusionsApi.md#delete_sensor_visibility_exclusions_v1) | **DELETE** /policy/entities/sv-exclusions/v1 | Delete the sensor visibility exclusions by id
*SensorVisibilityExclusionsApi* | [**get_sensor_visibility_exclusions_v1**](./SensorVisibilityExclusionsApi.md#get_sensor_visibility_exclusions_v1) | **GET** /policy/entities/sv-exclusions/v1 | Get a set of Sensor Visibility Exclusions by specifying their IDs
*SensorVisibilityExclusionsApi* | [**query_sensor_visibility_exclusions_v1**](./SensorVisibilityExclusionsApi.md#query_sensor_visibility_exclusions_v1) | **GET** /policy/queries/sv-exclusions/v1 | Search for sensor visibility exclusions.
*SensorVisibilityExclusionsApi* | [**update_sensor_visibility_exclusions_v1**](./SensorVisibilityExclusionsApi.md#update_sensor_visibility_exclusions_v1) | **PATCH** /policy/entities/sv-exclusions/v1 | Update the sensor visibility exclusions
*SpotlightEvaluationLogicApi* | [**combined_query_evaluation_logic**](./SpotlightEvaluationLogicApi.md#combined_query_evaluation_logic) | **GET** /spotlight/combined/evaluation-logic/v1 | Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic entities which match the filter criteria.
*SpotlightEvaluationLogicApi* | [**get_evaluation_logic**](./SpotlightEvaluationLogicApi.md#get_evaluation_logic) | **GET** /spotlight/entities/evaluation-logic/v1 | Get details on evaluation logic items by providing one or more IDs.
*SpotlightEvaluationLogicApi* | [**query_evaluation_logic**](./SpotlightEvaluationLogicApi.md#query_evaluation_logic) | **GET** /spotlight/queries/evaluation-logic/v1 | Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic IDs which match the filter criteria.
*SpotlightVulnerabilitiesApi* | [**combined_query_vulnerabilities**](./SpotlightVulnerabilitiesApi.md#combined_query_vulnerabilities) | **GET** /spotlight/combined/vulnerabilities/v1 | Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability entities which match the filter criteria
*SpotlightVulnerabilitiesApi* | [**get_remediations_v2**](./SpotlightVulnerabilitiesApi.md#get_remediations_v2) | **GET** /spotlight/entities/remediations/v2 | Get details on remediation by providing one or more IDs
*SpotlightVulnerabilitiesApi* | [**get_vulnerabilities**](./SpotlightVulnerabilitiesApi.md#get_vulnerabilities) | **GET** /spotlight/entities/vulnerabilities/v2 | Get details on vulnerabilities by providing one or more IDs
*SpotlightVulnerabilitiesApi* | [**query_vulnerabilities**](./SpotlightVulnerabilitiesApi.md#query_vulnerabilities) | **GET** /spotlight/queries/vulnerabilities/v1 | Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability IDs which match the filter criteria
*TailoredIntelligenceApi* | [**get_events_body**](./TailoredIntelligenceApi.md#get_events_body) | **GET** /ti/events/entities/events-full-body/v2 | Get event body for the provided event ID
*TailoredIntelligenceApi* | [**get_events_entities**](./TailoredIntelligenceApi.md#get_events_entities) | **POST** /ti/events/entities/events/GET/v2 | Get events entities for specified ids.
*TailoredIntelligenceApi* | [**get_rules_entities**](./TailoredIntelligenceApi.md#get_rules_entities) | **POST** /ti/rules/entities/rules/GET/v2 | Get rules entities for specified ids.
*TailoredIntelligenceApi* | [**query_events**](./TailoredIntelligenceApi.md#query_events) | **GET** /ti/events/queries/events/v2 | Get events ids that match the provided filter criteria.
*TailoredIntelligenceApi* | [**query_rules**](./TailoredIntelligenceApi.md#query_rules) | **GET** /ti/rules/queries/rules/v2 | Get rules ids that match the provided filter criteria.
*UserManagementApi* | [**combined_user_roles_v1**](./UserManagementApi.md#combined_user_roles_v1) | **GET** /user-management/combined/user-roles/v1 | Get User Grant(s). This endpoint lists both direct as well as flight control grants between a User and a Customer.
*UserManagementApi* | [**create_user**](./UserManagementApi.md#create_user) | **POST** /users/entities/users/v1 | Deprecated : Please use POST /user-management/entities/users/v1. Create a new user. After creating a user, assign one or more roles with POST /user-roles/entities/user-roles/v1
*UserManagementApi* | [**create_user_v1**](./UserManagementApi.md#create_user_v1) | **POST** /user-management/entities/users/v1 | Create a new user. After creating a user, assign one or more roles with POST '/user-management/entities/user-role-actions/v1'
*UserManagementApi* | [**delete_user**](./UserManagementApi.md#delete_user) | **DELETE** /users/entities/users/v1 | Deprecated : Please use DELETE /user-management/entities/users/v1. Delete a user permanently
*UserManagementApi* | [**delete_user_v1**](./UserManagementApi.md#delete_user_v1) | **DELETE** /user-management/entities/users/v1 | Delete a user permanently.
*UserManagementApi* | [**entities_roles_v1**](./UserManagementApi.md#entities_roles_v1) | **GET** /user-management/entities/roles/v1 | Get info about a role
*UserManagementApi* | [**get_available_role_ids**](./UserManagementApi.md#get_available_role_ids) | **GET** /user-roles/queries/user-role-ids-by-cid/v1 | Deprecated : Please use GET /user-management/queries/roles/v1. Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.
*UserManagementApi* | [**get_roles**](./UserManagementApi.md#get_roles) | **GET** /user-roles/entities/user-roles/v1 | Deprecated : Please use GET /user-management/entities/roles/v1. Get info about a role
*UserManagementApi* | [**get_user_role_ids**](./UserManagementApi.md#get_user_role_ids) | **GET** /user-roles/queries/user-role-ids-by-user-uuid/v1 | Deprecated : Please use GET /user-management/combined/user-roles/v1. Show role IDs of roles assigned to a user. For more information on each role, provide the role ID to `/customer/entities/roles/v1`.
*UserManagementApi* | [**grant_user_role_ids**](./UserManagementApi.md#grant_user_role_ids) | **POST** /user-roles/entities/user-roles/v1 | Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Assign one or more roles to a user
*UserManagementApi* | [**queries_roles_v1**](./UserManagementApi.md#queries_roles_v1) | **GET** /user-management/queries/roles/v1 | Show role IDs for all roles available in your customer account. For more information on each role, provide the role ID to `/user-management/entities/roles/v1`.
*UserManagementApi* | [**query_user_v1**](./UserManagementApi.md#query_user_v1) | **GET** /user-management/queries/users/v1 | List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/user-management/entities/users/GET/v1`.
*UserManagementApi* | [**retrieve_emails_by_cid**](./UserManagementApi.md#retrieve_emails_by_cid) | **GET** /users/queries/emails-by-cid/v1 | Deprecated : Please use POST /user-management/entities/users/GET/v1. List the usernames (usually an email address) for all users in your customer account
*UserManagementApi* | [**retrieve_user**](./UserManagementApi.md#retrieve_user) | **GET** /users/entities/users/v1 | Deprecated : Please use POST /user-management/entities/users/GET/v1. Get info about a user
*UserManagementApi* | [**retrieve_user_uuid**](./UserManagementApi.md#retrieve_user_uuid) | **GET** /users/queries/user-uuids-by-email/v1 | Deprecated : Please use GET /user-management/queries/users/v1. Get a user's ID by providing a username (usually an email address)
*UserManagementApi* | [**retrieve_user_uuids_by_cid**](./UserManagementApi.md#retrieve_user_uuids_by_cid) | **GET** /users/queries/user-uuids-by-cid/v1 | Deprecated : Please use GET /user-management/queries/users/v1. List user IDs for all users in your customer account. For more information on each user, provide the user ID to `/users/entities/user/v1`.
*UserManagementApi* | [**retrieve_users_getv1**](./UserManagementApi.md#retrieve_users_getv1) | **POST** /user-management/entities/users/GET/v1 | Get info about users including their name, UID and CID by providing user UUIDs
*UserManagementApi* | [**revoke_user_role_ids**](./UserManagementApi.md#revoke_user_role_ids) | **DELETE** /user-roles/entities/user-roles/v1 | Deprecated : Please use POST /user-management/entities/user-role-actions/v1. Revoke one or more roles from a user
*UserManagementApi* | [**update_user**](./UserManagementApi.md#update_user) | **PATCH** /users/entities/users/v1 | Deprecated : Please use PATCH /user-management/entities/users/v1. Modify an existing user's first or last name
*UserManagementApi* | [**update_user_v1**](./UserManagementApi.md#update_user_v1) | **PATCH** /user-management/entities/users/v1 | Modify an existing user's first or last name.
*UserManagementApi* | [**user_action_v1**](./UserManagementApi.md#user_action_v1) | **POST** /user-management/entities/user-actions/v1 | Apply actions to one or more User. Available action names: reset_2fa, reset_password. User UUIDs can be provided in `ids` param as part of request payload.
*UserManagementApi* | [**user_roles_action_v1**](./UserManagementApi.md#user_roles_action_v1) | **POST** /user-management/entities/user-role-actions/v1 | Grant or Revoke one or more role(s) to a user against a CID. User UUID, CID and Role ID(s) can be provided in request payload. Available Action(s) : grant, revoke
*ZeroTrustAssessmentApi* | [**get_assessment_v1**](./ZeroTrustAssessmentApi.md#get_assessment_v1) | **GET** /zero-trust-assessment/entities/assessments/v1 | Get Zero Trust Assessment data for one or more hosts by providing agent IDs (AID) and a customer ID (CID).
*ZeroTrustAssessmentApi* | [**get_assessments_by_score_v1**](./ZeroTrustAssessmentApi.md#get_assessments_by_score_v1) | **GET** /zero-trust-assessment/queries/assessments/v1 | Get Zero Trust Assessment data for one or more hosts by providing a customer ID (CID) and a range of scores.
*ZeroTrustAssessmentApi* | [**get_audit_v1**](./ZeroTrustAssessmentApi.md#get_audit_v1) | **GET** /zero-trust-assessment/entities/audit/v1 | Get the Zero Trust Assessment audit report for one customer ID (CID).

## Documentation For Models

- [ApiPeriodActionRespV1](./ApiPeriodActionRespV1.md)
- [ApiPeriodActionV1](./ApiPeriodActionV1.md)
- [ApiPeriodAggregatesResponse](./ApiPeriodAggregatesResponse.md)
- [ApiPeriodAuditEventDetailsResourceV1](./ApiPeriodAuditEventDetailsResourceV1.md)
- [ApiPeriodAuditEventDetailsResponseV1](./ApiPeriodAuditEventDetailsResponseV1.md)
- [ApiPeriodBulkUpdateReqV1](./ApiPeriodBulkUpdateReqV1.md)
- [ApiPeriodCredPayload](./ApiPeriodCredPayload.md)
- [ApiPeriodCustomerSettingsPatchRequestV1](./ApiPeriodCustomerSettingsPatchRequestV1.md)
- [ApiPeriodCustomerSettingsResourceV1](./ApiPeriodCustomerSettingsResourceV1.md)
- [ApiPeriodCustomerSettingsResponseV1](./ApiPeriodCustomerSettingsResponseV1.md)
- [ApiPeriodExposedDataFileDetailsV1](./ApiPeriodExposedDataFileDetailsV1.md)
- [ApiPeriodExposedDataRecordFinancialV1](./ApiPeriodExposedDataRecordFinancialV1.md)
- [ApiPeriodExposedDataRecordLocationV1](./ApiPeriodExposedDataRecordLocationV1.md)
- [ApiPeriodExposedDataRecordSocialV1](./ApiPeriodExposedDataRecordSocialV1.md)
- [ApiPeriodImageLookupRequest](./ApiPeriodImageLookupRequest.md)
- [ApiPeriodIndicatorCreateReqV1](./ApiPeriodIndicatorCreateReqV1.md)
- [ApiPeriodIndicatorCreateReqsV1](./ApiPeriodIndicatorCreateReqsV1.md)
- [ApiPeriodIndicatorQueryRespV1](./ApiPeriodIndicatorQueryRespV1.md)
- [ApiPeriodIndicatorRespV1](./ApiPeriodIndicatorRespV1.md)
- [ApiPeriodIndicatorUpdateReqV1](./ApiPeriodIndicatorUpdateReqV1.md)
- [ApiPeriodIndicatorUpdateReqsV1](./ApiPeriodIndicatorUpdateReqsV1.md)
- [ApiPeriodIndicatorV1](./ApiPeriodIndicatorV1.md)
- [ApiPeriodIndicatorsQueryMeta](./ApiPeriodIndicatorsQueryMeta.md)
- [ApiPeriodIndicatorsQueryPaging](./ApiPeriodIndicatorsQueryPaging.md)
- [ApiPeriodIndicatorsReportRequest](./ApiPeriodIndicatorsReportRequest.md)
- [ApiPeriodIndicatorsSortedSearch](./ApiPeriodIndicatorsSortedSearch.md)
- [ApiPeriodMessageCenterActivityResponse](./ApiPeriodMessageCenterActivityResponse.md)
- [ApiPeriodMessageCenterAttachmentUploadResponse](./ApiPeriodMessageCenterAttachmentUploadResponse.md)
- [ApiPeriodMessageCenterCasesResponse](./ApiPeriodMessageCenterCasesResponse.md)
- [ApiPeriodMetadataReqV1](./ApiPeriodMetadataReqV1.md)
- [ApiPeriodMetadataV1](./ApiPeriodMetadataV1.md)
- [ApiPeriodNotificationExposedDataRecordEntitiesResponseV1](./ApiPeriodNotificationExposedDataRecordEntitiesResponseV1.md)
- [ApiPeriodNotificationExposedDataRecordV1](./ApiPeriodNotificationExposedDataRecordV1.md)
- [ApiPeriodPatternV1](./ApiPeriodPatternV1.md)
- [ApiPeriodPatternsResponse](./ApiPeriodPatternsResponse.md)
- [ApiPeriodPlatformsResponse](./ApiPeriodPlatformsResponse.md)
- [ApiPeriodPostEnrollmentDetails](./ApiPeriodPostEnrollmentDetails.md)
- [ApiPeriodPostEnrollmentDetailsResponse](./ApiPeriodPostEnrollmentDetailsResponse.md)
- [ApiPeriodRuleCreateV1](./ApiPeriodRuleCreateV1.md)
- [ApiPeriodRuleDetailsV1](./ApiPeriodRuleDetailsV1.md)
- [ApiPeriodRuleGroupCreateRequestV1](./ApiPeriodRuleGroupCreateRequestV1.md)
- [ApiPeriodRuleGroupModifyRequestV1](./ApiPeriodRuleGroupModifyRequestV1.md)
- [ApiPeriodRuleGroupV1](./ApiPeriodRuleGroupV1.md)
- [ApiPeriodRuleGroupsResponse](./ApiPeriodRuleGroupsResponse.md)
- [ApiPeriodRuleTypeV1](./ApiPeriodRuleTypeV1.md)
- [ApiPeriodRuleTypesResponse](./ApiPeriodRuleTypesResponse.md)
- [ApiPeriodRuleUpdateV1](./ApiPeriodRuleUpdateV1.md)
- [ApiPeriodRuleUpdatesRequestV1](./ApiPeriodRuleUpdatesRequestV1.md)
- [ApiPeriodRuleV1](./ApiPeriodRuleV1.md)
- [ApiPeriodRulesGetRequestV1](./ApiPeriodRulesGetRequestV1.md)
- [ApiPeriodRulesResponse](./ApiPeriodRulesResponse.md)
- [ApiPeriodSensorDetailsResponseSwagger](./ApiPeriodSensorDetailsResponseSwagger.md)
- [ApiPeriodTokenCreateRequestV1](./ApiPeriodTokenCreateRequestV1.md)
- [ApiPeriodTokenDetailsResourceV1](./ApiPeriodTokenDetailsResourceV1.md)
- [ApiPeriodTokenDetailsResponseV1](./ApiPeriodTokenDetailsResponseV1.md)
- [ApiPeriodTokenPatchRequestV1](./ApiPeriodTokenPatchRequestV1.md)
- [ApiPeriodUserMetadataResponse](./ApiPeriodUserMetadataResponse.md)
- [ApiPeriodUserRoleIdsResponse](./ApiPeriodUserRoleIdsResponse.md)
- [ApiPeriodUserRoleResponse](./ApiPeriodUserRoleResponse.md)
- [ApiPeriodValidationRequestFieldV1](./ApiPeriodValidationRequestFieldV1.md)
- [ApiPeriodValidationRequestV1](./ApiPeriodValidationRequestV1.md)
- [ApiPeriodValidationResponseV1](./ApiPeriodValidationResponseV1.md)
- [BasePeriodPolicyMembersRespV1](./BasePeriodPolicyMembersRespV1.md)
- [BasePeriodSetPolicyPrecedenceReqV1](./BasePeriodSetPolicyPrecedenceReqV1.md)
- [BinservapiPeriodMsaPutFileResponse](./BinservapiPeriodMsaPutFileResponse.md)
- [ChangesPeriodAcl](./ChangesPeriodAcl.md)
- [ChangesPeriodAclChange](./ChangesPeriodAclChange.md)
- [ChangesPeriodAfter](./ChangesPeriodAfter.md)
- [ChangesPeriodAttribute](./ChangesPeriodAttribute.md)
- [ChangesPeriodBasic](./ChangesPeriodBasic.md)
- [ChangesPeriodBefore](./ChangesPeriodBefore.md)
- [ChangesPeriodChange](./ChangesPeriodChange.md)
- [ChangesPeriodDacl](./ChangesPeriodDacl.md)
- [ChangesPeriodDaclEntity](./ChangesPeriodDaclEntity.md)
- [ChangesPeriodDiff](./ChangesPeriodDiff.md)
- [ChangesPeriodDiffHash](./ChangesPeriodDiffHash.md)
- [ChangesPeriodDiffType](./ChangesPeriodDiffType.md)
- [ChangesPeriodGetChangesResponse](./ChangesPeriodGetChangesResponse.md)
- [ChangesPeriodGroup](./ChangesPeriodGroup.md)
- [ChangesPeriodHighVolumeQueryMeta](./ChangesPeriodHighVolumeQueryMeta.md)
- [ChangesPeriodHighVolumeQueryPaging](./ChangesPeriodHighVolumeQueryPaging.md)
- [ChangesPeriodHighVolumeQueryResponse](./ChangesPeriodHighVolumeQueryResponse.md)
- [ChangesPeriodHost](./ChangesPeriodHost.md)
- [ChangesPeriodHostGroup](./ChangesPeriodHostGroup.md)
- [ChangesPeriodOwner](./ChangesPeriodOwner.md)
- [ChangesPeriodPermissions](./ChangesPeriodPermissions.md)
- [ChangesPeriodPermissionsLin](./ChangesPeriodPermissionsLin.md)
- [ChangesPeriodPolicy](./ChangesPeriodPolicy.md)
- [ChangesPeriodPolicyRule](./ChangesPeriodPolicyRule.md)
- [ChangesPeriodPolicyRuleGroup](./ChangesPeriodPolicyRuleGroup.md)
- [ChangesPeriodPrevalence](./ChangesPeriodPrevalence.md)
- [ChangesPeriodTag](./ChangesPeriodTag.md)
- [ClassificationPeriodCriteria](./ClassificationPeriodCriteria.md)
- [ClassificationPeriodLabel](./ClassificationPeriodLabel.md)
- [ClientPeriodArchiveCreateResponseV1](./ClientPeriodArchiveCreateResponseV1.md)
- [ClientPeriodArchiveListFilesResponseV1](./ClientPeriodArchiveListFilesResponseV1.md)
- [ClientPeriodArchiveWithFilesV1](./ClientPeriodArchiveWithFilesV1.md)
- [ClientPeriodExtractionCreateRequestV1](./ClientPeriodExtractionCreateRequestV1.md)
- [ClientPeriodExtractionCreateResponseV1](./ClientPeriodExtractionCreateResponseV1.md)
- [ClientPeriodExtractionFileParametersV1](./ClientPeriodExtractionFileParametersV1.md)
- [ClientPeriodExtractionFileResultV1](./ClientPeriodExtractionFileResultV1.md)
- [ClientPeriodExtractionListFilesResponseV1](./ClientPeriodExtractionListFilesResponseV1.md)
- [ClientPeriodExtractionWithFilesV1](./ClientPeriodExtractionWithFilesV1.md)
- [ClientPeriodQuerySamplesRequest](./ClientPeriodQuerySamplesRequest.md)
- [ClientPeriodSampleMetadataResponseV2](./ClientPeriodSampleMetadataResponseV2.md)
- [ClientPeriodSampleMetadataV2](./ClientPeriodSampleMetadataV2.md)
- [CommonPeriodCidAuditResult](./CommonPeriodCidAuditResult.md)
- [CommonPeriodEntitiesResponse](./CommonPeriodEntitiesResponse.md)
- [CommonPeriodOsAudit](./CommonPeriodOsAudit.md)
- [CorePeriodEntitiesResponse](./CorePeriodEntitiesResponse.md)
- [DetectionPeriodAggregateIndicator](./DetectionPeriodAggregateIndicator.md)
- [DetectionPeriodAggregateThreatIntel](./DetectionPeriodAggregateThreatIntel.md)
- [DetectionPeriodEnrichment](./DetectionPeriodEnrichment.md)
- [DetectionPeriodEnrichmentHost](./DetectionPeriodEnrichmentHost.md)
- [DetectionPeriodHostNetwork](./DetectionPeriodHostNetwork.md)
- [DetectionPeriodInventoryEnrichment](./DetectionPeriodInventoryEnrichment.md)
- [DetectsPeriodAlert](./DetectsPeriodAlert.md)
- [DetectsPeriodBehavior](./DetectsPeriodBehavior.md)
- [DetectsPeriodDeviceDetailIndexed](./DetectsPeriodDeviceDetailIndexed.md)
- [DetectsPeriodHostInfo](./DetectsPeriodHostInfo.md)
- [DetectsPeriodParentDetails](./DetectsPeriodParentDetails.md)
- [DetectsPeriodQuarantinedFile](./DetectsPeriodQuarantinedFile.md)
- [DetectsapiPeriodPatchEntitiesAlertsV2Request](./DetectsapiPeriodPatchEntitiesAlertsV2Request.md)
- [DetectsapiPeriodPostEntitiesAlertsV1Request](./DetectsapiPeriodPostEntitiesAlertsV1Request.md)
- [DetectsapiPeriodPostEntitiesAlertsV1Response](./DetectsapiPeriodPostEntitiesAlertsV1Response.md)
- [DeviceControlPeriodCreatePoliciesV1](./DeviceControlPeriodCreatePoliciesV1.md)
- [DeviceControlPeriodCreatePolicyReqV1](./DeviceControlPeriodCreatePolicyReqV1.md)
- [DeviceControlPeriodExceptionReqV1](./DeviceControlPeriodExceptionReqV1.md)
- [DeviceControlPeriodExceptionRespV1](./DeviceControlPeriodExceptionRespV1.md)
- [DeviceControlPeriodPolicyV1](./DeviceControlPeriodPolicyV1.md)
- [DeviceControlPeriodReqUpdateDefaultDcPolicyV1](./DeviceControlPeriodReqUpdateDefaultDcPolicyV1.md)
- [DeviceControlPeriodRespV1](./DeviceControlPeriodRespV1.md)
- [DeviceControlPeriodRespV2](./DeviceControlPeriodRespV2.md)
- [DeviceControlPeriodSettingsReqV1](./DeviceControlPeriodSettingsReqV1.md)
- [DeviceControlPeriodSettingsRespV1](./DeviceControlPeriodSettingsRespV1.md)
- [DeviceControlPeriodUpdatePoliciesReqV1](./DeviceControlPeriodUpdatePoliciesReqV1.md)
- [DeviceControlPeriodUpdatePolicyReqV1](./DeviceControlPeriodUpdatePolicyReqV1.md)
- [DeviceControlPeriodUsbClassExceptionsReqV1](./DeviceControlPeriodUsbClassExceptionsReqV1.md)
- [DeviceControlPeriodUsbClassExceptionsResponse](./DeviceControlPeriodUsbClassExceptionsResponse.md)
- [DeviceControlPeriodUsbCustomNotification](./DeviceControlPeriodUsbCustomNotification.md)
- [DeviceControlPeriodUsbCustomNotifications](./DeviceControlPeriodUsbCustomNotifications.md)
- [DevicePeriodDevice](./DevicePeriodDevice.md)
- [DevicePeriodDeviceMeta](./DevicePeriodDeviceMeta.md)
- [DevicePeriodDevicePolicy](./DevicePeriodDevicePolicy.md)
- [DevicePeriodManagedApp](./DevicePeriodManagedApp.md)
- [DevicePeriodManagedApps](./DevicePeriodManagedApps.md)
- [DevicePeriodMappedDevicePolicies](./DevicePeriodMappedDevicePolicies.md)
- [DeviceapiPeriodDeviceDetailsResponseSwagger](./DeviceapiPeriodDeviceDetailsResponseSwagger.md)
- [DeviceapiPeriodDevicePaging](./DeviceapiPeriodDevicePaging.md)
- [DeviceapiPeriodDeviceResponse](./DeviceapiPeriodDeviceResponse.md)
- [DeviceapiPeriodDeviceSwagger](./DeviceapiPeriodDeviceSwagger.md)
- [DeviceapiPeriodGroupMeta](./DeviceapiPeriodGroupMeta.md)
- [DeviceapiPeriodGroupResponseV1](./DeviceapiPeriodGroupResponseV1.md)
- [DeviceapiPeriodGroupsResponseV1](./DeviceapiPeriodGroupsResponseV1.md)
- [DeviceapiPeriodLoginDetailV1](./DeviceapiPeriodLoginDetailV1.md)
- [DeviceapiPeriodLoginHistoryResponseV1](./DeviceapiPeriodLoginHistoryResponseV1.md)
- [DeviceapiPeriodLoginInfoV1](./DeviceapiPeriodLoginInfoV1.md)
- [DeviceapiPeriodNetworkAddressHistoryResponseV1](./DeviceapiPeriodNetworkAddressHistoryResponseV1.md)
- [DeviceapiPeriodNetworkAddressHistoryV1](./DeviceapiPeriodNetworkAddressHistoryV1.md)
- [DeviceapiPeriodNetworkAddressV1](./DeviceapiPeriodNetworkAddressV1.md)
- [DeviceapiPeriodRequestMeta](./DeviceapiPeriodRequestMeta.md)
- [DeviceapiPeriodUpdateDeviceDetailsResponseV1](./DeviceapiPeriodUpdateDeviceDetailsResponseV1.md)
- [DeviceapiPeriodUpdateDeviceTagsRequestV1](./DeviceapiPeriodUpdateDeviceTagsRequestV1.md)
- [DeviceapiPeriodUpdateDeviceTagsSwaggerV1](./DeviceapiPeriodUpdateDeviceTagsSwaggerV1.md)
- [DomainPeriodAccessTokenResponseV1](./DomainPeriodAccessTokenResponseV1.md)
- [DomainPeriodAccountAccessResult](./DomainPeriodAccountAccessResult.md)
- [DomainPeriodActionEntitiesResponseV1](./DomainPeriodActionEntitiesResponseV1.md)
- [DomainPeriodActionParameter](./DomainPeriodActionParameter.md)
- [DomainPeriodActionUserRolesRequest](./DomainPeriodActionUserRolesRequest.md)
- [DomainPeriodActionV1](./DomainPeriodActionV1.md)
- [DomainPeriodActivityCreationRequest](./DomainPeriodActivityCreationRequest.md)
- [DomainPeriodActorDocument](./DomainPeriodActorDocument.md)
- [DomainPeriodActorsResponse](./DomainPeriodActorsResponse.md)
- [DomainPeriodAggregatesResponse](./DomainPeriodAggregatesResponse.md)
- [DomainPeriodApiCombinedFindingsResponseV1](./DomainPeriodApiCombinedFindingsResponseV1.md)
- [DomainPeriodApiDetectionDocument](./DomainPeriodApiDetectionDocument.md)
- [DomainPeriodApiError](./DomainPeriodApiError.md)
- [DomainPeriodApiEvaluationLogicComparisonsV1](./DomainPeriodApiEvaluationLogicComparisonsV1.md)
- [DomainPeriodApiEvaluationLogicEntitiesResponseV1](./DomainPeriodApiEvaluationLogicEntitiesResponseV1.md)
- [DomainPeriodApiEvaluationLogicEntityComparisonV1](./DomainPeriodApiEvaluationLogicEntityComparisonV1.md)
- [DomainPeriodApiEvaluationLogicItemV1](./DomainPeriodApiEvaluationLogicItemV1.md)
- [DomainPeriodApiEvaluationLogicStateComparisonV1](./DomainPeriodApiEvaluationLogicStateComparisonV1.md)
- [DomainPeriodApiEvaluationLogicV1](./DomainPeriodApiEvaluationLogicV1.md)
- [DomainPeriodApiFindingFacetV1](./DomainPeriodApiFindingFacetV1.md)
- [DomainPeriodApiFindingRuleV1](./DomainPeriodApiFindingRuleV1.md)
- [DomainPeriodApiFindingWithRuleV1](./DomainPeriodApiFindingWithRuleV1.md)
- [DomainPeriodApiHostGroup](./DomainPeriodApiHostGroup.md)
- [DomainPeriodApiHostInfoFacetV1](./DomainPeriodApiHostInfoFacetV1.md)
- [DomainPeriodApiMitreAttackTacticV1](./DomainPeriodApiMitreAttackTacticV1.md)
- [DomainPeriodApiMitreMitigationV1](./DomainPeriodApiMitreMitigationV1.md)
- [DomainPeriodApiMitreTechniqueV1](./DomainPeriodApiMitreTechniqueV1.md)
- [DomainPeriodApiQueryMetaV1](./DomainPeriodApiQueryMetaV1.md)
- [DomainPeriodApiQueryPagingV1](./DomainPeriodApiQueryPagingV1.md)
- [DomainPeriodApiRemediationIds](./DomainPeriodApiRemediationIds.md)
- [DomainPeriodApiRemediationV2](./DomainPeriodApiRemediationV2.md)
- [DomainPeriodApiVulnerabilityAppV2](./DomainPeriodApiVulnerabilityAppV2.md)
- [DomainPeriodApiVulnerabilityCveDetailsFacetV2](./DomainPeriodApiVulnerabilityCveDetailsFacetV2.md)
- [DomainPeriodApiVulnerabilityCvecisaInfo](./DomainPeriodApiVulnerabilityCvecisaInfo.md)
- [DomainPeriodApiVulnerabilityDataProviderV1](./DomainPeriodApiVulnerabilityDataProviderV1.md)
- [DomainPeriodApiVulnerabilityExtendedAppV2](./DomainPeriodApiVulnerabilityExtendedAppV2.md)
- [DomainPeriodApiVulnerabilityHostFacetV2](./DomainPeriodApiVulnerabilityHostFacetV2.md)
- [DomainPeriodApiVulnerabilityRemediationFacetV2](./DomainPeriodApiVulnerabilityRemediationFacetV2.md)
- [DomainPeriodApiVulnerabilitySuppressionInfoV2](./DomainPeriodApiVulnerabilitySuppressionInfoV2.md)
- [DomainPeriodApiVulnerabilityV2](./DomainPeriodApiVulnerabilityV2.md)
- [DomainPeriodAssessment](./DomainPeriodAssessment.md)
- [DomainPeriodAssessmentItems](./DomainPeriodAssessmentItems.md)
- [DomainPeriodAssessmentsByScoreResponse](./DomainPeriodAssessmentsByScoreResponse.md)
- [DomainPeriodAssessmentsResponse](./DomainPeriodAssessmentsResponse.md)
- [DomainPeriodAuditResponse](./DomainPeriodAuditResponse.md)
- [DomainPeriodAwsAccountInput](./DomainPeriodAwsAccountInput.md)
- [DomainPeriodAwsAccountV2](./DomainPeriodAwsAccountV2.md)
- [DomainPeriodAwsBatchClusterRegion](./DomainPeriodAwsBatchClusterRegion.md)
- [DomainPeriodAwsConfiguration](./DomainPeriodAwsConfiguration.md)
- [DomainPeriodAwsd4CAccountV1](./DomainPeriodAwsd4CAccountV1.md)
- [DomainPeriodAwsd4COrganizationAccountV1](./DomainPeriodAwsd4COrganizationAccountV1.md)
- [DomainPeriodAzureClientServicePrincipalV1](./DomainPeriodAzureClientServicePrincipalV1.md)
- [DomainPeriodAzureResourcePermission](./DomainPeriodAzureResourcePermission.md)
- [DomainPeriodAzureRoleAssignment](./DomainPeriodAzureRoleAssignment.md)
- [DomainPeriodBaseApiVulnerabilityV2](./DomainPeriodBaseApiVulnerabilityV2.md)
- [DomainPeriodBatchExecuteCommandRequest](./DomainPeriodBatchExecuteCommandRequest.md)
- [DomainPeriodBatchGetCmdStatusResponse](./DomainPeriodBatchGetCmdStatusResponse.md)
- [DomainPeriodBatchGetCommandRequest](./DomainPeriodBatchGetCommandRequest.md)
- [DomainPeriodBatchGetCommandResponse](./DomainPeriodBatchGetCommandResponse.md)
- [DomainPeriodBatchInitSessionRequest](./DomainPeriodBatchInitSessionRequest.md)
- [DomainPeriodBatchInitSessionResponse](./DomainPeriodBatchInitSessionResponse.md)
- [DomainPeriodBatchRefreshSessionRequest](./DomainPeriodBatchRefreshSessionRequest.md)
- [DomainPeriodBatchRefreshSessionResponse](./DomainPeriodBatchRefreshSessionResponse.md)
- [DomainPeriodBehavior](./DomainPeriodBehavior.md)
- [DomainPeriodBenchmark](./DomainPeriodBenchmark.md)
- [DomainPeriodBotnetConfigSource](./DomainPeriodBotnetConfigSource.md)
- [DomainPeriodBotnetInject](./DomainPeriodBotnetInject.md)
- [DomainPeriodBreachDetailsV1](./DomainPeriodBreachDetailsV1.md)
- [DomainPeriodBreachedItemV1](./DomainPeriodBreachedItemV1.md)
- [DomainPeriodCaseCreationRequest](./DomainPeriodCaseCreationRequest.md)
- [DomainPeriodCaseCreationRequestV2](./DomainPeriodCaseCreationRequestV2.md)
- [DomainPeriodChildLink](./DomainPeriodChildLink.md)
- [DomainPeriodChildrenResponseV1](./DomainPeriodChildrenResponseV1.md)
- [DomainPeriodCidGroup](./DomainPeriodCidGroup.md)
- [DomainPeriodCidGroupMembers](./DomainPeriodCidGroupMembers.md)
- [DomainPeriodCidGroupMembersRequestV1](./DomainPeriodCidGroupMembersRequestV1.md)
- [DomainPeriodCidGroupMembersResponseV1](./DomainPeriodCidGroupMembersResponseV1.md)
- [DomainPeriodCidGroupsRequestV1](./DomainPeriodCidGroupsRequestV1.md)
- [DomainPeriodCidGroupsResponseV1](./DomainPeriodCidGroupsResponseV1.md)
- [DomainPeriodCidPolicyAssignments](./DomainPeriodCidPolicyAssignments.md)
- [DomainPeriodCloudAccounts](./DomainPeriodCloudAccounts.md)
- [DomainPeriodCloudScope](./DomainPeriodCloudScope.md)
- [DomainPeriodCommandExecuteRequest](./DomainPeriodCommandExecuteRequest.md)
- [DomainPeriodCommandExecuteResponse](./DomainPeriodCommandExecuteResponse.md)
- [DomainPeriodCommandExecuteResponseWrapper](./DomainPeriodCommandExecuteResponseWrapper.md)
- [DomainPeriodCondition](./DomainPeriodCondition.md)
- [DomainPeriodCreateActionRequest](./DomainPeriodCreateActionRequest.md)
- [DomainPeriodCreateUserRequest](./DomainPeriodCreateUserRequest.md)
- [DomainPeriodCredentials](./DomainPeriodCredentials.md)
- [DomainPeriodCsixLabel](./DomainPeriodCsixLabel.md)
- [DomainPeriodCsixRelation](./DomainPeriodCsixRelation.md)
- [DomainPeriodDdosAttackSource](./DomainPeriodDdosAttackSource.md)
- [DomainPeriodDdosTargetDetails](./DomainPeriodDdosTargetDetails.md)
- [DomainPeriodDetailedNotificationV1](./DomainPeriodDetailedNotificationV1.md)
- [DomainPeriodDetectsEntitiesPatchRequest](./DomainPeriodDetectsEntitiesPatchRequest.md)
- [DomainPeriodDevice](./DomainPeriodDevice.md)
- [DomainPeriodDiscoverApiAccount](./DomainPeriodDiscoverApiAccount.md)
- [DomainPeriodDiscoverApiAccountEntitiesResponse](./DomainPeriodDiscoverApiAccountEntitiesResponse.md)
- [DomainPeriodDiscoverApiActiveDiscoveryHost](./DomainPeriodDiscoverApiActiveDiscoveryHost.md)
- [DomainPeriodDiscoverApiActiveDiscoveryNetwork](./DomainPeriodDiscoverApiActiveDiscoveryNetwork.md)
- [DomainPeriodDiscoverApiApplication](./DomainPeriodDiscoverApiApplication.md)
- [DomainPeriodDiscoverApiApplicationEntitiesResponse](./DomainPeriodDiscoverApiApplicationEntitiesResponse.md)
- [DomainPeriodDiscoverApiApplicationHost](./DomainPeriodDiscoverApiApplicationHost.md)
- [DomainPeriodDiscoverApiBiosHashesData](./DomainPeriodDiscoverApiBiosHashesData.md)
- [DomainPeriodDiscoverApiDeviceSlot](./DomainPeriodDiscoverApiDeviceSlot.md)
- [DomainPeriodDiscoverApiDiskSize](./DomainPeriodDiscoverApiDiskSize.md)
- [DomainPeriodDiscoverApiFieldMetadata](./DomainPeriodDiscoverApiFieldMetadata.md)
- [DomainPeriodDiscoverApiHost](./DomainPeriodDiscoverApiHost.md)
- [DomainPeriodDiscoverApiHostEntitiesResponse](./DomainPeriodDiscoverApiHostEntitiesResponse.md)
- [DomainPeriodDiscoverApiHostTriage](./DomainPeriodDiscoverApiHostTriage.md)
- [DomainPeriodDiscoverApiLogin](./DomainPeriodDiscoverApiLogin.md)
- [DomainPeriodDiscoverApiLoginEntitiesResponse](./DomainPeriodDiscoverApiLoginEntitiesResponse.md)
- [DomainPeriodDiscoverApiMountStorageInfo](./DomainPeriodDiscoverApiMountStorageInfo.md)
- [DomainPeriodDiscoverApiNetworkInterface](./DomainPeriodDiscoverApiNetworkInterface.md)
- [DomainPeriodDiscoverApiioTHost](./DomainPeriodDiscoverApiioTHost.md)
- [DomainPeriodDiscoverApiioTHostEntitiesResponse](./DomainPeriodDiscoverApiioTHostEntitiesResponse.md)
- [DomainPeriodDiscoverApiosSecurity](./DomainPeriodDiscoverApiosSecurity.md)
- [DomainPeriodDiscoverParams](./DomainPeriodDiscoverParams.md)
- [DomainPeriodDisposition](./DomainPeriodDisposition.md)
- [DomainPeriodECrimeKillChain](./DomainPeriodECrimeKillChain.md)
- [DomainPeriodEntitiesPatchRequest](./DomainPeriodEntitiesPatchRequest.md)
- [DomainPeriodEntity](./DomainPeriodEntity.md)
- [DomainPeriodEntityActionRequest](./DomainPeriodEntityActionRequest.md)
- [DomainPeriodEnvironmentScore](./DomainPeriodEnvironmentScore.md)
- [DomainPeriodErrorsOnly](./DomainPeriodErrorsOnly.md)
- [DomainPeriodEvent](./DomainPeriodEvent.md)
- [DomainPeriodEventEntitiesResponse](./DomainPeriodEventEntitiesResponse.md)
- [DomainPeriodEventHistogram](./DomainPeriodEventHistogram.md)
- [DomainPeriodExecutionMetadataSummary](./DomainPeriodExecutionMetadataSummary.md)
- [DomainPeriodExecutionMetadataV1](./DomainPeriodExecutionMetadataV1.md)
- [DomainPeriodExportJobEntitiesResponseV1](./DomainPeriodExportJobEntitiesResponseV1.md)
- [DomainPeriodExportJobIdResponseV1](./DomainPeriodExportJobIdResponseV1.md)
- [DomainPeriodExportJobMetadataV1](./DomainPeriodExportJobMetadataV1.md)
- [DomainPeriodExportJobV1](./DomainPeriodExportJobV1.md)
- [DomainPeriodExposedDataRecordFinancialV1](./DomainPeriodExposedDataRecordFinancialV1.md)
- [DomainPeriodExposedDataRecordLocationV1](./DomainPeriodExposedDataRecordLocationV1.md)
- [DomainPeriodExposedDataRecordSocialV1](./DomainPeriodExposedDataRecordSocialV1.md)
- [DomainPeriodExternalApiRegistry](./DomainPeriodExternalApiRegistry.md)
- [DomainPeriodExternalCredentialResponse](./DomainPeriodExternalCredentialResponse.md)
- [DomainPeriodExternalQueryResponse](./DomainPeriodExternalQueryResponse.md)
- [DomainPeriodExternalRegistryListResponse](./DomainPeriodExternalRegistryListResponse.md)
- [DomainPeriodExternalRegistryResponse](./DomainPeriodExternalRegistryResponse.md)
- [DomainPeriodField](./DomainPeriodField.md)
- [DomainPeriodFieldValidation](./DomainPeriodFieldValidation.md)
- [DomainPeriodFieldValue](./DomainPeriodFieldValue.md)
- [DomainPeriodFile](./DomainPeriodFile.md)
- [DomainPeriodFileCount](./DomainPeriodFileCount.md)
- [DomainPeriodFileDetailsV1](./DomainPeriodFileDetailsV1.md)
- [DomainPeriodFileV2](./DomainPeriodFileV2.md)
- [DomainPeriodGcpAccountV1](./DomainPeriodGcpAccountV1.md)
- [DomainPeriodImage](./DomainPeriodImage.md)
- [DomainPeriodIncident](./DomainPeriodIncident.md)
- [DomainPeriodInitRequest](./DomainPeriodInitRequest.md)
- [DomainPeriodInitResponse](./DomainPeriodInitResponse.md)
- [DomainPeriodInitResponseWrapper](./DomainPeriodInitResponseWrapper.md)
- [DomainPeriodKeyValuePair](./DomainPeriodKeyValuePair.md)
- [DomainPeriodKillChain](./DomainPeriodKillChain.md)
- [DomainPeriodLastScheduledExecution](./DomainPeriodLastScheduledExecution.md)
- [DomainPeriodLastUnscheduledExecution](./DomainPeriodLastUnscheduledExecution.md)
- [DomainPeriodLaunchExportJobRequestV1](./DomainPeriodLaunchExportJobRequestV1.md)
- [DomainPeriodLaunchExportJobResponseV1](./DomainPeriodLaunchExportJobResponseV1.md)
- [DomainPeriodListFilesResponseWrapper](./DomainPeriodListFilesResponseWrapper.md)
- [DomainPeriodListFilesV2ResponseWrapper](./DomainPeriodListFilesV2ResponseWrapper.md)
- [DomainPeriodListSessionsResponseMsa](./DomainPeriodListSessionsResponseMsa.md)
- [DomainPeriodMaliciousFile](./DomainPeriodMaliciousFile.md)
- [DomainPeriodMatchedBreachSummaryV1](./DomainPeriodMatchedBreachSummaryV1.md)
- [DomainPeriodMatchedRule](./DomainPeriodMatchedRule.md)
- [DomainPeriodMetaInfo](./DomainPeriodMetaInfo.md)
- [DomainPeriodMsaDetectSummariesResponse](./DomainPeriodMsaDetectSummariesResponse.md)
- [DomainPeriodMsaEnvironmentScoreResponse](./DomainPeriodMsaEnvironmentScoreResponse.md)
- [DomainPeriodMsaExternalBehaviorResponse](./DomainPeriodMsaExternalBehaviorResponse.md)
- [DomainPeriodMsaExternalIncidentResponse](./DomainPeriodMsaExternalIncidentResponse.md)
- [DomainPeriodMsaIncidentPerformActionResponse](./DomainPeriodMsaIncidentPerformActionResponse.md)
- [DomainPeriodMsaIncidentQueryResponse](./DomainPeriodMsaIncidentQueryResponse.md)
- [DomainPeriodMsaQfResponse](./DomainPeriodMsaQfResponse.md)
- [DomainPeriodMsspRoleRequestV1](./DomainPeriodMsspRoleRequestV1.md)
- [DomainPeriodMsspRoleResponseV1](./DomainPeriodMsspRoleResponseV1.md)
- [DomainPeriodMsspRoles](./DomainPeriodMsspRoles.md)
- [DomainPeriodMultiCommandExecuteResponse](./DomainPeriodMultiCommandExecuteResponse.md)
- [DomainPeriodMultiCommandExecuteResponseWrapper](./DomainPeriodMultiCommandExecuteResponseWrapper.md)
- [DomainPeriodMultiPulseSensorResponse](./DomainPeriodMultiPulseSensorResponse.md)
- [DomainPeriodMultiStatusSensorResponse](./DomainPeriodMultiStatusSensorResponse.md)
- [DomainPeriodNewsDocument](./DomainPeriodNewsDocument.md)
- [DomainPeriodNewsResponse](./DomainPeriodNewsResponse.md)
- [DomainPeriodNotificationConfig](./DomainPeriodNotificationConfig.md)
- [DomainPeriodNotificationDetailsResponseV1](./DomainPeriodNotificationDetailsResponseV1.md)
- [DomainPeriodNotificationDetailsV1](./DomainPeriodNotificationDetailsV1.md)
- [DomainPeriodNotificationEntitiesResponseV1](./DomainPeriodNotificationEntitiesResponseV1.md)
- [DomainPeriodNotificationIdResponse](./DomainPeriodNotificationIdResponse.md)
- [DomainPeriodNotificationV1](./DomainPeriodNotificationV1.md)
- [DomainPeriodNotifications](./DomainPeriodNotifications.md)
- [DomainPeriodPastebinTextSource](./DomainPeriodPastebinTextSource.md)
- [DomainPeriodPermission](./DomainPeriodPermission.md)
- [DomainPeriodPlatform](./DomainPeriodPlatform.md)
- [DomainPeriodPolicyInfo](./DomainPeriodPolicyInfo.md)
- [DomainPeriodPolicySettingByAccountAndRegion](./DomainPeriodPolicySettingByAccountAndRegion.md)
- [DomainPeriodPublicIndicatorV3](./DomainPeriodPublicIndicatorV3.md)
- [DomainPeriodPublicIndicatorsV3Response](./DomainPeriodPublicIndicatorsV3Response.md)
- [DomainPeriodQueriesPatchRequest](./DomainPeriodQueriesPatchRequest.md)
- [DomainPeriodQueryMitreAttacksResponse](./DomainPeriodQueryMitreAttacksResponse.md)
- [DomainPeriodQueryResponse](./DomainPeriodQueryResponse.md)
- [DomainPeriodQueuedSessionCommand](./DomainPeriodQueuedSessionCommand.md)
- [DomainPeriodQueuedSessionJob](./DomainPeriodQueuedSessionJob.md)
- [DomainPeriodQueuedSessionResponseWrapper](./DomainPeriodQueuedSessionResponseWrapper.md)
- [DomainPeriodQuota](./DomainPeriodQuota.md)
- [DomainPeriodReconApiError](./DomainPeriodReconApiError.md)
- [DomainPeriodReconApiErrorDetail](./DomainPeriodReconApiErrorDetail.md)
- [DomainPeriodRegisterActionsRequest](./DomainPeriodRegisterActionsRequest.md)
- [DomainPeriodRegistryCredentialsResponse](./DomainPeriodRegistryCredentialsResponse.md)
- [DomainPeriodReportExecutionLaunchRequestV1](./DomainPeriodReportExecutionLaunchRequestV1.md)
- [DomainPeriodReportExecutionRetryRequestV1](./DomainPeriodReportExecutionRetryRequestV1.md)
- [DomainPeriodReportExecutionSummaryV1](./DomainPeriodReportExecutionSummaryV1.md)
- [DomainPeriodReportExecutionV1](./DomainPeriodReportExecutionV1.md)
- [DomainPeriodReportExecutionsResponseV1](./DomainPeriodReportExecutionsResponseV1.md)
- [DomainPeriodReportMetadata](./DomainPeriodReportMetadata.md)
- [DomainPeriodReportParams](./DomainPeriodReportParams.md)
- [DomainPeriodResultMetadata](./DomainPeriodResultMetadata.md)
- [DomainPeriodRole](./DomainPeriodRole.md)
- [DomainPeriodRoleIds](./DomainPeriodRoleIds.md)
- [DomainPeriodRule](./DomainPeriodRule.md)
- [DomainPeriodRuleEntitiesResponse](./DomainPeriodRuleEntitiesResponse.md)
- [DomainPeriodRuleMetaInfo](./DomainPeriodRuleMetaInfo.md)
- [DomainPeriodRulePreviewRequest](./DomainPeriodRulePreviewRequest.md)
- [DomainPeriodRuleQueryResponseV1](./DomainPeriodRuleQueryResponseV1.md)
- [DomainPeriodRuleQuota](./DomainPeriodRuleQuota.md)
- [DomainPeriodRulesEntitiesResponseV1](./DomainPeriodRulesEntitiesResponseV1.md)
- [DomainPeriodRulesResponse](./DomainPeriodRulesResponse.md)
- [DomainPeriodScan](./DomainPeriodScan.md)
- [DomainPeriodScanHostMetadata](./DomainPeriodScanHostMetadata.md)
- [DomainPeriodScanMetadata](./DomainPeriodScanMetadata.md)
- [DomainPeriodScanProfile](./DomainPeriodScanProfile.md)
- [DomainPeriodScanProfileMetadata](./DomainPeriodScanProfileMetadata.md)
- [DomainPeriodScanScheduleDataV1](./DomainPeriodScanScheduleDataV1.md)
- [DomainPeriodSchedule](./DomainPeriodSchedule.md)
- [DomainPeriodScheduledReportV1](./DomainPeriodScheduledReportV1.md)
- [DomainPeriodScheduledReportsResultV1](./DomainPeriodScheduledReportsResultV1.md)
- [DomainPeriodScriptHelp](./DomainPeriodScriptHelp.md)
- [DomainPeriodSearchAfterMeta](./DomainPeriodSearchAfterMeta.md)
- [DomainPeriodSearchAfterPaging](./DomainPeriodSearchAfterPaging.md)
- [DomainPeriodSensorInstallerV1](./DomainPeriodSensorInstallerV1.md)
- [DomainPeriodSensorInstallersV1](./DomainPeriodSensorInstallersV1.md)
- [DomainPeriodSession](./DomainPeriodSession.md)
- [DomainPeriodSessionResponseWrapper](./DomainPeriodSessionResponseWrapper.md)
- [DomainPeriodSignalProperties](./DomainPeriodSignalProperties.md)
- [DomainPeriodSignalProperty](./DomainPeriodSignalProperty.md)
- [DomainPeriodSimpleActor](./DomainPeriodSimpleActor.md)
- [DomainPeriodSpapiCombinedVulnerabilitiesResponse](./DomainPeriodSpapiCombinedVulnerabilitiesResponse.md)
- [DomainPeriodSpapiEvaluationLogicCombinedResponseV1](./DomainPeriodSpapiEvaluationLogicCombinedResponseV1.md)
- [DomainPeriodSpapiEvaluationLogicEntitiesResponseV1](./DomainPeriodSpapiEvaluationLogicEntitiesResponseV1.md)
- [DomainPeriodSpapiQueryMeta](./DomainPeriodSpapiQueryMeta.md)
- [DomainPeriodSpapiQueryPaging](./DomainPeriodSpapiQueryPaging.md)
- [DomainPeriodSpapiQueryResponse](./DomainPeriodSpapiQueryResponse.md)
- [DomainPeriodSpapiRemediationEntitiesResponseV2](./DomainPeriodSpapiRemediationEntitiesResponseV2.md)
- [DomainPeriodSpapiVulnerabilitiesEntitiesResponseV2](./DomainPeriodSpapiVulnerabilitiesEntitiesResponseV2.md)
- [DomainPeriodStatusResponse](./DomainPeriodStatusResponse.md)
- [DomainPeriodStatusResponseWrapper](./DomainPeriodStatusResponseWrapper.md)
- [DomainPeriodTweetSource](./DomainPeriodTweetSource.md)
- [DomainPeriodUpdateActionRequest](./DomainPeriodUpdateActionRequest.md)
- [DomainPeriodUpdateNotificationRequestV1](./DomainPeriodUpdateNotificationRequestV1.md)
- [DomainPeriodUpdateResults](./DomainPeriodUpdateResults.md)
- [DomainPeriodUpdateRuleRequestV1](./DomainPeriodUpdateRuleRequestV1.md)
- [DomainPeriodUpdateUserFields](./DomainPeriodUpdateUserFields.md)
- [DomainPeriodUpdateUserRequest](./DomainPeriodUpdateUserRequest.md)
- [DomainPeriodUser](./DomainPeriodUser.md)
- [DomainPeriodUserAction](./DomainPeriodUserAction.md)
- [DomainPeriodUserActionRequest](./DomainPeriodUserActionRequest.md)
- [DomainPeriodUserCreateRequest](./DomainPeriodUserCreateRequest.md)
- [DomainPeriodUserGrants](./DomainPeriodUserGrants.md)
- [DomainPeriodUserGroup](./DomainPeriodUserGroup.md)
- [DomainPeriodUserGroupMembers](./DomainPeriodUserGroupMembers.md)
- [DomainPeriodUserGroupMembersRequestV1](./DomainPeriodUserGroupMembersRequestV1.md)
- [DomainPeriodUserGroupMembersResponseV1](./DomainPeriodUserGroupMembersResponseV1.md)
- [DomainPeriodUserGroupsRequestV1](./DomainPeriodUserGroupsRequestV1.md)
- [DomainPeriodUserGroupsResponseV1](./DomainPeriodUserGroupsResponseV1.md)
- [DomainPeriodUserMetadata](./DomainPeriodUserMetadata.md)
- [DomainPeriodUserRole](./DomainPeriodUserRole.md)
- [DomainPeriodValueItem](./DomainPeriodValueItem.md)
- [DomainPeriodVulnerability](./DomainPeriodVulnerability.md)
- [DomainPeriodVulnerabilityActor](./DomainPeriodVulnerabilityActor.md)
- [DomainPeriodVulnerabilityAffectedProduct](./DomainPeriodVulnerabilityAffectedProduct.md)
- [DomainPeriodVulnerabilityRelatedThreat](./DomainPeriodVulnerabilityRelatedThreat.md)
- [DomainPeriodVulnerabilityReport](./DomainPeriodVulnerabilityReport.md)
- [DomainPeriodVulnerabilityResponse](./DomainPeriodVulnerabilityResponse.md)
- [DomainPeriodXdrData](./DomainPeriodXdrData.md)
- [DomainPeriodXdrParams](./DomainPeriodXdrParams.md)
- [DomainPeriodZeroTrustSimpleAssessment](./DomainPeriodZeroTrustSimpleAssessment.md)
- [EmpowerapiPeriodMsaPfResponseV1](./EmpowerapiPeriodMsaPfResponseV1.md)
- [EmpowerapiPeriodMsaPfResponseV2](./EmpowerapiPeriodMsaPfResponseV2.md)
- [EmpowerapiPeriodRemoteCommandPutFileV1](./EmpowerapiPeriodRemoteCommandPutFileV1.md)
- [EmpowerapiPeriodRemoteCommandPutFileV2](./EmpowerapiPeriodRemoteCommandPutFileV2.md)
- [EntitiesPeriodOdsCancelScanRequest](./EntitiesPeriodOdsCancelScanRequest.md)
- [EntitiesPeriodOdsScanHostResponse](./EntitiesPeriodOdsScanHostResponse.md)
- [EntitiesPeriodOdsScanMaliciousFileResponse](./EntitiesPeriodOdsScanMaliciousFileResponse.md)
- [EntitiesPeriodOdsScanRequest](./EntitiesPeriodOdsScanRequest.md)
- [EntitiesPeriodOdsScanResponse](./EntitiesPeriodOdsScanResponse.md)
- [EntitiesPeriodOdsScheduleScanRequest](./EntitiesPeriodOdsScheduleScanRequest.md)
- [EntitiesPeriodOdsScheduleScanResponse](./EntitiesPeriodOdsScheduleScanResponse.md)
- [ExclusionsPeriodCreateReqV1](./ExclusionsPeriodCreateReqV1.md)
- [ExclusionsPeriodExclusionV1](./ExclusionsPeriodExclusionV1.md)
- [ExclusionsPeriodRespV1](./ExclusionsPeriodRespV1.md)
- [FalconxPeriodActor](./FalconxPeriodActor.md)
- [FalconxPeriodActorSummary](./FalconxPeriodActorSummary.md)
- [FalconxPeriodAmsiCall](./FalconxPeriodAmsiCall.md)
- [FalconxPeriodAssociatedRuntime](./FalconxPeriodAssociatedRuntime.md)
- [FalconxPeriodC2](./FalconxPeriodC2.md)
- [FalconxPeriodCertificate](./FalconxPeriodCertificate.md)
- [FalconxPeriodContactedHost](./FalconxPeriodContactedHost.md)
- [FalconxPeriodDnsRequest](./FalconxPeriodDnsRequest.md)
- [FalconxPeriodEntity](./FalconxPeriodEntity.md)
- [FalconxPeriodErrorsOnly](./FalconxPeriodErrorsOnly.md)
- [FalconxPeriodExtractedFile](./FalconxPeriodExtractedFile.md)
- [FalconxPeriodExtractedInterestingString](./FalconxPeriodExtractedInterestingString.md)
- [FalconxPeriodFileAccess](./FalconxPeriodFileAccess.md)
- [FalconxPeriodFileDataDirectory](./FalconxPeriodFileDataDirectory.md)
- [FalconxPeriodFileImport](./FalconxPeriodFileImport.md)
- [FalconxPeriodFileMetadata](./FalconxPeriodFileMetadata.md)
- [FalconxPeriodFileResource](./FalconxPeriodFileResource.md)
- [FalconxPeriodFileSection](./FalconxPeriodFileSection.md)
- [FalconxPeriodHandle](./FalconxPeriodHandle.md)
- [FalconxPeriodHttpRequest](./FalconxPeriodHttpRequest.md)
- [FalconxPeriodIncident](./FalconxPeriodIncident.md)
- [FalconxPeriodIntelReportV1](./FalconxPeriodIntelReportV1.md)
- [FalconxPeriodIntelSummaryReportV1](./FalconxPeriodIntelSummaryReportV1.md)
- [FalconxPeriodIntelXReportV1](./FalconxPeriodIntelXReportV1.md)
- [FalconxPeriodMalqueryErrorV1](./FalconxPeriodMalqueryErrorV1.md)
- [FalconxPeriodMalqueryReportV1](./FalconxPeriodMalqueryReportV1.md)
- [FalconxPeriodMalqueryResource](./FalconxPeriodMalqueryResource.md)
- [FalconxPeriodMalwareConfig](./FalconxPeriodMalwareConfig.md)
- [FalconxPeriodMatchedSignature](./FalconxPeriodMatchedSignature.md)
- [FalconxPeriodMemoryDumpData](./FalconxPeriodMemoryDumpData.md)
- [FalconxPeriodMemoryForensic](./FalconxPeriodMemoryForensic.md)
- [FalconxPeriodMetaInfo](./FalconxPeriodMetaInfo.md)
- [FalconxPeriodMitreAttack](./FalconxPeriodMitreAttack.md)
- [FalconxPeriodMitreAttackParent](./FalconxPeriodMitreAttackParent.md)
- [FalconxPeriodModule](./FalconxPeriodModule.md)
- [FalconxPeriodParameter](./FalconxPeriodParameter.md)
- [FalconxPeriodProcess](./FalconxPeriodProcess.md)
- [FalconxPeriodProcessFlag](./FalconxPeriodProcessFlag.md)
- [FalconxPeriodQueryResponse](./FalconxPeriodQueryResponse.md)
- [FalconxPeriodQuota](./FalconxPeriodQuota.md)
- [FalconxPeriodRegistry](./FalconxPeriodRegistry.md)
- [FalconxPeriodRelatedIndicator](./FalconxPeriodRelatedIndicator.md)
- [FalconxPeriodReportV1](./FalconxPeriodReportV1.md)
- [FalconxPeriodReportV1Response](./FalconxPeriodReportV1Response.md)
- [FalconxPeriodSandboxParametersV1](./FalconxPeriodSandboxParametersV1.md)
- [FalconxPeriodSandboxReportV1](./FalconxPeriodSandboxReportV1.md)
- [FalconxPeriodSandboxSummaryReportV1](./FalconxPeriodSandboxSummaryReportV1.md)
- [FalconxPeriodScriptCall](./FalconxPeriodScriptCall.md)
- [FalconxPeriodSignature](./FalconxPeriodSignature.md)
- [FalconxPeriodStream](./FalconxPeriodStream.md)
- [FalconxPeriodSubmissionParametersV1](./FalconxPeriodSubmissionParametersV1.md)
- [FalconxPeriodSubmissionV1](./FalconxPeriodSubmissionV1.md)
- [FalconxPeriodSubmissionV1Response](./FalconxPeriodSubmissionV1Response.md)
- [FalconxPeriodSummaryReportV1](./FalconxPeriodSummaryReportV1.md)
- [FalconxPeriodSummaryReportV1Response](./FalconxPeriodSummaryReportV1Response.md)
- [FalconxPeriodSuricataAlert](./FalconxPeriodSuricataAlert.md)
- [FalconxPeriodThreatGraphIndicatorV1](./FalconxPeriodThreatGraphIndicatorV1.md)
- [FalconxPeriodThreatGraphReportV1](./FalconxPeriodThreatGraphReportV1.md)
- [FalconxPeriodUrlData](./FalconxPeriodUrlData.md)
- [FalconxPeriodVersionInfo](./FalconxPeriodVersionInfo.md)
- [FirewallPeriodCreateFirewallPoliciesReqV1](./FirewallPeriodCreateFirewallPoliciesReqV1.md)
- [FirewallPeriodCreateFirewallPolicyReqV1](./FirewallPeriodCreateFirewallPolicyReqV1.md)
- [FirewallPeriodPolicyV1](./FirewallPeriodPolicyV1.md)
- [FirewallPeriodRespV1](./FirewallPeriodRespV1.md)
- [FirewallPeriodUpdateFirewallPoliciesReqV1](./FirewallPeriodUpdateFirewallPoliciesReqV1.md)
- [FirewallPeriodUpdateFirewallPolicyReqV1](./FirewallPeriodUpdateFirewallPolicyReqV1.md)
- [FlightcontrolapiPeriodGetRolesResponse](./FlightcontrolapiPeriodGetRolesResponse.md)
- [FlightcontrolapiPeriodUserGrantResponse](./FlightcontrolapiPeriodUserGrantResponse.md)
- [FlightcontrolapiPeriodUserResponse](./FlightcontrolapiPeriodUserResponse.md)
- [FwmgrPeriodApiPeriodAggregatesResponse](./FwmgrPeriodApiPeriodAggregatesResponse.md)
- [FwmgrPeriodApiPeriodEventsResponse](./FwmgrPeriodApiPeriodEventsResponse.md)
- [FwmgrPeriodApiPeriodFilepathTestRequest](./FwmgrPeriodApiPeriodFilepathTestRequest.md)
- [FwmgrPeriodApiPeriodFilepathTestResult](./FwmgrPeriodApiPeriodFilepathTestResult.md)
- [FwmgrPeriodApiPeriodFirewallFieldsResponse](./FwmgrPeriodApiPeriodFirewallFieldsResponse.md)
- [FwmgrPeriodApiPeriodFirewallFieldsV1](./FwmgrPeriodApiPeriodFirewallFieldsV1.md)
- [FwmgrPeriodApiPeriodJsonDiff](./FwmgrPeriodApiPeriodJsonDiff.md)
- [FwmgrPeriodApiPeriodMetaInfo](./FwmgrPeriodApiPeriodMetaInfo.md)
- [FwmgrPeriodApiPeriodNetworkLocationCreateRequestV1](./FwmgrPeriodApiPeriodNetworkLocationCreateRequestV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1](./FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationModifyPrecedenceRequestV1](./FwmgrPeriodApiPeriodNetworkLocationModifyPrecedenceRequestV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1](./FwmgrPeriodApiPeriodNetworkLocationModifyRequestV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationSummariesResponse](./FwmgrPeriodApiPeriodNetworkLocationSummariesResponse.md)
- [FwmgrPeriodApiPeriodNetworkLocationSummaryV1](./FwmgrPeriodApiPeriodNetworkLocationSummaryV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationsMetadataV1](./FwmgrPeriodApiPeriodNetworkLocationsMetadataV1.md)
- [FwmgrPeriodApiPeriodNetworkLocationsResponse](./FwmgrPeriodApiPeriodNetworkLocationsResponse.md)
- [FwmgrPeriodApiPeriodNetworkLocationsV1](./FwmgrPeriodApiPeriodNetworkLocationsV1.md)
- [FwmgrPeriodApiPeriodPlatformsResponse](./FwmgrPeriodApiPeriodPlatformsResponse.md)
- [FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1](./FwmgrPeriodApiPeriodPolicyContainerUpsertRequestV1.md)
- [FwmgrPeriodApiPeriodPolicyContainersResponse](./FwmgrPeriodApiPeriodPolicyContainersResponse.md)
- [FwmgrPeriodApiPeriodQueryPaging](./FwmgrPeriodApiPeriodQueryPaging.md)
- [FwmgrPeriodApiPeriodQueryResponse](./FwmgrPeriodApiPeriodQueryResponse.md)
- [FwmgrPeriodApiPeriodRuleCreateRequestV1](./FwmgrPeriodApiPeriodRuleCreateRequestV1.md)
- [FwmgrPeriodApiPeriodRuleGroupCreateRequestV1](./FwmgrPeriodApiPeriodRuleGroupCreateRequestV1.md)
- [FwmgrPeriodApiPeriodRuleGroupModifyRequestV1](./FwmgrPeriodApiPeriodRuleGroupModifyRequestV1.md)
- [FwmgrPeriodApiPeriodRuleGroupV1](./FwmgrPeriodApiPeriodRuleGroupV1.md)
- [FwmgrPeriodApiPeriodRuleGroupsResponse](./FwmgrPeriodApiPeriodRuleGroupsResponse.md)
- [FwmgrPeriodApiPeriodRulesResponse](./FwmgrPeriodApiPeriodRulesResponse.md)
- [FwmgrPeriodApiPeriodValidateFilepathResponse](./FwmgrPeriodApiPeriodValidateFilepathResponse.md)
- [FwmgrPeriodApiPeriodWorkaroundUiFieldValue](./FwmgrPeriodApiPeriodWorkaroundUiFieldValue.md)
- [FwmgrPeriodDomainPeriodAddressRange](./FwmgrPeriodDomainPeriodAddressRange.md)
- [FwmgrPeriodDomainPeriodConnectionType](./FwmgrPeriodDomainPeriodConnectionType.md)
- [FwmgrPeriodDomainPeriodDnsResolutionTargets](./FwmgrPeriodDomainPeriodDnsResolutionTargets.md)
- [FwmgrPeriodDomainPeriodDnsResolutionTargetsWithPolling](./FwmgrPeriodDomainPeriodDnsResolutionTargetsWithPolling.md)
- [FwmgrPeriodDomainPeriodDnsTarget](./FwmgrPeriodDomainPeriodDnsTarget.md)
- [FwmgrPeriodDomainPeriodField](./FwmgrPeriodDomainPeriodField.md)
- [FwmgrPeriodDomainPeriodHttpsHosts](./FwmgrPeriodDomainPeriodHttpsHosts.md)
- [FwmgrPeriodDomainPeriodHttpsHostsWithPolling](./FwmgrPeriodDomainPeriodHttpsHostsWithPolling.md)
- [FwmgrPeriodDomainPeriodIcmp](./FwmgrPeriodDomainPeriodIcmp.md)
- [FwmgrPeriodDomainPeriodIcmpTargets](./FwmgrPeriodDomainPeriodIcmpTargets.md)
- [FwmgrPeriodDomainPeriodIcmpTargetsWithPolling](./FwmgrPeriodDomainPeriodIcmpTargetsWithPolling.md)
- [FwmgrPeriodDomainPeriodMonitoring](./FwmgrPeriodDomainPeriodMonitoring.md)
- [FwmgrPeriodDomainPeriodPlatform](./FwmgrPeriodDomainPeriodPlatform.md)
- [FwmgrPeriodDomainPeriodPortRange](./FwmgrPeriodDomainPeriodPortRange.md)
- [FwmgrPeriodDomainPeriodValueItem](./FwmgrPeriodDomainPeriodValueItem.md)
- [FwmgrPeriodDomainPeriodWirelessType](./FwmgrPeriodDomainPeriodWirelessType.md)
- [FwmgrPeriodFirewallPeriodAddressRange](./FwmgrPeriodFirewallPeriodAddressRange.md)
- [FwmgrPeriodFirewallPeriodFieldValue](./FwmgrPeriodFirewallPeriodFieldValue.md)
- [FwmgrPeriodFirewallPeriodFlags](./FwmgrPeriodFirewallPeriodFlags.md)
- [FwmgrPeriodFirewallPeriodIcmp](./FwmgrPeriodFirewallPeriodIcmp.md)
- [FwmgrPeriodFirewallPeriodMatchEventResponse](./FwmgrPeriodFirewallPeriodMatchEventResponse.md)
- [FwmgrPeriodFirewallPeriodMonitoring](./FwmgrPeriodFirewallPeriodMonitoring.md)
- [FwmgrPeriodFirewallPeriodPolicyContainerV1](./FwmgrPeriodFirewallPeriodPolicyContainerV1.md)
- [FwmgrPeriodFirewallPeriodPortRange](./FwmgrPeriodFirewallPeriodPortRange.md)
- [FwmgrPeriodFirewallPeriodRuleGroupSummaryV1](./FwmgrPeriodFirewallPeriodRuleGroupSummaryV1.md)
- [FwmgrPeriodFirewallPeriodRuleV1](./FwmgrPeriodFirewallPeriodRuleV1.md)
- [FwmgrPeriodMsaPeriodAggregateQueryRequest](./FwmgrPeriodMsaPeriodAggregateQueryRequest.md)
- [FwmgrPeriodMsaPeriodAggregationResult](./FwmgrPeriodMsaPeriodAggregationResult.md)
- [FwmgrPeriodMsaPeriodAggregationResultItem](./FwmgrPeriodMsaPeriodAggregationResultItem.md)
- [FwmgrPeriodMsaPeriodDateRangeSpec](./FwmgrPeriodMsaPeriodDateRangeSpec.md)
- [FwmgrPeriodMsaPeriodRangeSpec](./FwmgrPeriodMsaPeriodRangeSpec.md)
- [FwmgrPeriodMsaspecPeriodError](./FwmgrPeriodMsaspecPeriodError.md)
- [FwmgrPeriodMsaspecPeriodMetaInfo](./FwmgrPeriodMsaspecPeriodMetaInfo.md)
- [FwmgrPeriodMsaspecPeriodPaging](./FwmgrPeriodMsaspecPeriodPaging.md)
- [FwmgrPeriodMsaspecPeriodQueryResponse](./FwmgrPeriodMsaspecPeriodQueryResponse.md)
- [FwmgrPeriodMsaspecPeriodResponseFields](./FwmgrPeriodMsaspecPeriodResponseFields.md)
- [FwmgrPeriodMsaspecPeriodWrites](./FwmgrPeriodMsaspecPeriodWrites.md)
- [HostGroupsPeriodCreateGroupReqV1](./HostGroupsPeriodCreateGroupReqV1.md)
- [HostGroupsPeriodCreateGroupsReqV1](./HostGroupsPeriodCreateGroupsReqV1.md)
- [HostGroupsPeriodHostGroupV1](./HostGroupsPeriodHostGroupV1.md)
- [HostGroupsPeriodMembersRespV1](./HostGroupsPeriodMembersRespV1.md)
- [HostGroupsPeriodRespV1](./HostGroupsPeriodRespV1.md)
- [HostGroupsPeriodUpdateGroupReqV1](./HostGroupsPeriodUpdateGroupReqV1.md)
- [HostGroupsPeriodUpdateGroupsReqV1](./HostGroupsPeriodUpdateGroupsReqV1.md)
- [ImagesPeriodExtCombinedImagesResponse](./ImagesPeriodExtCombinedImagesResponse.md)
- [InternalPeriodSensorStatus](./InternalPeriodSensorStatus.md)
- [IoaExclusionsPeriodIoaExclusionCreateReqV1](./IoaExclusionsPeriodIoaExclusionCreateReqV1.md)
- [IoaExclusionsPeriodIoaExclusionRespV1](./IoaExclusionsPeriodIoaExclusionRespV1.md)
- [IoaExclusionsPeriodIoaExclusionUpdateReqV1](./IoaExclusionsPeriodIoaExclusionUpdateReqV1.md)
- [IoaExclusionsPeriodIoaExclusionsRespV1](./IoaExclusionsPeriodIoaExclusionsRespV1.md)
- [IoaPeriodCloudAccountId](./IoaPeriodCloudAccountId.md)
- [IoaPeriodEnrichments](./IoaPeriodEnrichments.md)
- [IoaPeriodEventAggregate](./IoaPeriodEventAggregate.md)
- [IoaRuleGroupsPeriodRuleGroupV1](./IoaRuleGroupsPeriodRuleGroupV1.md)
- [IocapiPeriodIocDevicesCount](./IocapiPeriodIocDevicesCount.md)
- [IocapiPeriodMsaReplyDevicesRanOn](./IocapiPeriodMsaReplyDevicesRanOn.md)
- [IocapiPeriodMsaReplyIocDevicesCount](./IocapiPeriodMsaReplyIocDevicesCount.md)
- [IocapiPeriodMsaReplyProcessesRanOn](./IocapiPeriodMsaReplyProcessesRanOn.md)
- [IocapiPeriodPaginationMeta](./IocapiPeriodPaginationMeta.md)
- [IocapiPeriodResponseMeta](./IocapiPeriodResponseMeta.md)
- [K8sregPeriodAccountPermissionsStatus](./K8sregPeriodAccountPermissionsStatus.md)
- [K8sregPeriodApiKey](./K8sregPeriodApiKey.md)
- [K8sregPeriodAwsAccountResp](./K8sregPeriodAwsAccountResp.md)
- [K8sregPeriodAwsClusterItemResp](./K8sregPeriodAwsClusterItemResp.md)
- [K8sregPeriodAzureAcctClusterItemResp](./K8sregPeriodAzureAcctClusterItemResp.md)
- [K8sregPeriodAzureBashScript](./K8sregPeriodAzureBashScript.md)
- [K8sregPeriodAzureSubEntity](./K8sregPeriodAzureSubEntity.md)
- [K8sregPeriodAzureSubscriptionResp](./K8sregPeriodAzureSubscriptionResp.md)
- [K8sregPeriodAzureTenantConfig](./K8sregPeriodAzureTenantConfig.md)
- [K8sregPeriodAzureTenantInfo](./K8sregPeriodAzureTenantInfo.md)
- [K8sregPeriodClusterResp](./K8sregPeriodClusterResp.md)
- [K8sregPeriodCreateAwsAccReq](./K8sregPeriodCreateAwsAccReq.md)
- [K8sregPeriodCreateAwsAccReqPeriodResources](./K8sregPeriodCreateAwsAccReqPeriodResources.md)
- [K8sregPeriodCreateAwsAccResp](./K8sregPeriodCreateAwsAccResp.md)
- [K8sregPeriodCreateAzureSubReq](./K8sregPeriodCreateAzureSubReq.md)
- [K8sregPeriodGetAwsAccountsResp](./K8sregPeriodGetAwsAccountsResp.md)
- [K8sregPeriodGetAzureBashScriptResp](./K8sregPeriodGetAzureBashScriptResp.md)
- [K8sregPeriodGetAzureSubscriptionsResp](./K8sregPeriodGetAzureSubscriptionsResp.md)
- [K8sregPeriodGetAzureTenantConfigResp](./K8sregPeriodGetAzureTenantConfigResp.md)
- [K8sregPeriodGetAzureTenantInfoResp](./K8sregPeriodGetAzureTenantInfoResp.md)
- [K8sregPeriodGetClustersResp](./K8sregPeriodGetClustersResp.md)
- [K8sregPeriodGetLocationsResp](./K8sregPeriodGetLocationsResp.md)
- [K8sregPeriodGetScriptsResp](./K8sregPeriodGetScriptsResp.md)
- [K8sregPeriodListClusterCloudItemResp](./K8sregPeriodListClusterCloudItemResp.md)
- [K8sregPeriodListClusterCloudResp](./K8sregPeriodListClusterCloudResp.md)
- [K8sregPeriodLocationResp](./K8sregPeriodLocationResp.md)
- [K8sregPeriodRegenApiKeyResp](./K8sregPeriodRegenApiKeyResp.md)
- [K8sregPeriodVersionResp](./K8sregPeriodVersionResp.md)
- [MainPeriodAvailableStreamV2](./MainPeriodAvailableStreamV2.md)
- [MainPeriodDiscoveryResponseV2](./MainPeriodDiscoveryResponseV2.md)
- [MainPeriodSessionToken](./MainPeriodSessionToken.md)
- [MalqueryPeriodExternalExactSearchParametersV1](./MalqueryPeriodExternalExactSearchParametersV1.md)
- [MalqueryPeriodExternalHuntOptions](./MalqueryPeriodExternalHuntOptions.md)
- [MalqueryPeriodExternalHuntParametersV1](./MalqueryPeriodExternalHuntParametersV1.md)
- [MalqueryPeriodExternalQueryResponse](./MalqueryPeriodExternalQueryResponse.md)
- [MalqueryPeriodExternalResource](./MalqueryPeriodExternalResource.md)
- [MalqueryPeriodFuzzyOptions](./MalqueryPeriodFuzzyOptions.md)
- [MalqueryPeriodFuzzySearchMetaInfo](./MalqueryPeriodFuzzySearchMetaInfo.md)
- [MalqueryPeriodFuzzySearchParametersV1](./MalqueryPeriodFuzzySearchParametersV1.md)
- [MalqueryPeriodFuzzySearchResponse](./MalqueryPeriodFuzzySearchResponse.md)
- [MalqueryPeriodMultiDownloadRequestV1](./MalqueryPeriodMultiDownloadRequestV1.md)
- [MalqueryPeriodQueryError](./MalqueryPeriodQueryError.md)
- [MalqueryPeriodQueryMetaInfo](./MalqueryPeriodQueryMetaInfo.md)
- [MalqueryPeriodRateLimitsMeta](./MalqueryPeriodRateLimitsMeta.md)
- [MalqueryPeriodRateLimitsResponse](./MalqueryPeriodRateLimitsResponse.md)
- [MalqueryPeriodRequestMetaInfo](./MalqueryPeriodRequestMetaInfo.md)
- [MalqueryPeriodRequestResponse](./MalqueryPeriodRequestResponse.md)
- [MalqueryPeriodSampleMetadata](./MalqueryPeriodSampleMetadata.md)
- [MalqueryPeriodSampleMetadataResponse](./MalqueryPeriodSampleMetadataResponse.md)
- [MalqueryPeriodSamplesMetadataMetaInfo](./MalqueryPeriodSamplesMetadataMetaInfo.md)
- [MalqueryPeriodSearchParameter](./MalqueryPeriodSearchParameter.md)
- [MalqueryPeriodStats](./MalqueryPeriodStats.md)
- [MalqueryPeriodUserRequestCount](./MalqueryPeriodUserRequestCount.md)
- [MessagesPeriodActivity](./MessagesPeriodActivity.md)
- [MessagesPeriodAlert](./MessagesPeriodAlert.md)
- [MessagesPeriodAttachment](./MessagesPeriodAttachment.md)
- [MessagesPeriodAuthor](./MessagesPeriodAuthor.md)
- [MessagesPeriodCase](./MessagesPeriodCase.md)
- [MessagesPeriodDetection](./MessagesPeriodDetection.md)
- [MessagesPeriodIncident](./MessagesPeriodIncident.md)
- [MlscannerapiPeriodMetaInfo](./MlscannerapiPeriodMetaInfo.md)
- [MlscannerapiPeriodQueryResponse](./MlscannerapiPeriodQueryResponse.md)
- [MlscannerapiPeriodQuota](./MlscannerapiPeriodQuota.md)
- [MlscannerapiPeriodSamplesScanParameters](./MlscannerapiPeriodSamplesScanParameters.md)
- [MlscannerapiPeriodSamplesScanResult](./MlscannerapiPeriodSamplesScanResult.md)
- [MlscannerapiPeriodScanV1Response](./MlscannerapiPeriodScanV1Response.md)
- [MlscannerapiPeriodScannedSample](./MlscannerapiPeriodScannedSample.md)
- [ModelPeriodArgument](./ModelPeriodArgument.md)
- [ModelPeriodFile](./ModelPeriodFile.md)
- [ModelPeriodSessionLog](./ModelPeriodSessionLog.md)
- [ModelsPeriodAccessHealthDetails](./ModelsPeriodAccessHealthDetails.md)
- [ModelsPeriodAccountEntitiesInput](./ModelsPeriodAccountEntitiesInput.md)
- [ModelsPeriodAccountStatusResponse](./ModelsPeriodAccountStatusResponse.md)
- [ModelsPeriodApplicationLibrary](./ModelsPeriodApplicationLibrary.md)
- [ModelsPeriodApplicationPackageInfoType](./ModelsPeriodApplicationPackageInfoType.md)
- [ModelsPeriodAwsAccountAccessHealth](./ModelsPeriodAwsAccountAccessHealth.md)
- [ModelsPeriodAwsAccountRequestV1](./ModelsPeriodAwsAccountRequestV1.md)
- [ModelsPeriodAwsAccountV1](./ModelsPeriodAwsAccountV1.md)
- [ModelsPeriodAwsAccountsV1](./ModelsPeriodAwsAccountsV1.md)
- [ModelsPeriodAwsCustomerSettingsRequestV1](./ModelsPeriodAwsCustomerSettingsRequestV1.md)
- [ModelsPeriodBaseResponseV1](./ModelsPeriodBaseResponseV1.md)
- [ModelsPeriodCreateAwsAccountsV1](./ModelsPeriodCreateAwsAccountsV1.md)
- [ModelsPeriodCredentials](./ModelsPeriodCredentials.md)
- [ModelsPeriodCustomerConfigurationsV1](./ModelsPeriodCustomerConfigurationsV1.md)
- [ModelsPeriodExtApiImageCombined](./ModelsPeriodExtApiImageCombined.md)
- [ModelsPeriodJobMetaData](./ModelsPeriodJobMetaData.md)
- [ModelsPeriodModifyAwsCustomerSettingsV1](./ModelsPeriodModifyAwsCustomerSettingsV1.md)
- [ModelsPeriodPackageInfoType](./ModelsPeriodPackageInfoType.md)
- [ModelsPeriodRegistryCredentialsResponse](./ModelsPeriodRegistryCredentialsResponse.md)
- [ModelsPeriodScanResults](./ModelsPeriodScanResults.md)
- [ModelsPeriodSnapshotAccountStatus](./ModelsPeriodSnapshotAccountStatus.md)
- [ModelsPeriodSnapshotInventoryApplication](./ModelsPeriodSnapshotInventoryApplication.md)
- [ModelsPeriodSnapshotInventoryPayload](./ModelsPeriodSnapshotInventoryPayload.md)
- [ModelsPeriodUpdateAwsAccountsV1](./ModelsPeriodUpdateAwsAccountsV1.md)
- [ModelsPeriodVerifyAccessResponseV1](./ModelsPeriodVerifyAccessResponseV1.md)
- [MsaPeriodAffectedEntity](./MsaPeriodAffectedEntity.md)
- [MsaPeriodAggregateQueryRequest](./MsaPeriodAggregateQueryRequest.md)
- [MsaPeriodAggregatesResponse](./MsaPeriodAggregatesResponse.md)
- [MsaPeriodAggregationResult](./MsaPeriodAggregationResult.md)
- [MsaPeriodAggregationResultItem](./MsaPeriodAggregationResultItem.md)
- [MsaPeriodApiError](./MsaPeriodApiError.md)
- [MsaPeriodBaseEntitiesResponse](./MsaPeriodBaseEntitiesResponse.md)
- [MsaPeriodDateRangeSpec](./MsaPeriodDateRangeSpec.md)
- [MsaPeriodEntitiesResponse](./MsaPeriodEntitiesResponse.md)
- [MsaPeriodEntityActionRequest](./MsaPeriodEntityActionRequest.md)
- [MsaPeriodEntityActionRequestV2](./MsaPeriodEntityActionRequestV2.md)
- [MsaPeriodErrorsOnly](./MsaPeriodErrorsOnly.md)
- [MsaPeriodFacet](./MsaPeriodFacet.md)
- [MsaPeriodFacetsResponse](./MsaPeriodFacetsResponse.md)
- [MsaPeriodIdsRequest](./MsaPeriodIdsRequest.md)
- [MsaPeriodMetaInfo](./MsaPeriodMetaInfo.md)
- [MsaPeriodPaging](./MsaPeriodPaging.md)
- [MsaPeriodQueryResponse](./MsaPeriodQueryResponse.md)
- [MsaPeriodRangeSpec](./MsaPeriodRangeSpec.md)
- [MsaPeriodReplyAffectedEntities](./MsaPeriodReplyAffectedEntities.md)
- [MsaPeriodReplyMetaOnly](./MsaPeriodReplyMetaOnly.md)
- [MsaPeriodResources](./MsaPeriodResources.md)
- [MsaspecPeriodActionParameter](./MsaspecPeriodActionParameter.md)
- [MsaspecPeriodError](./MsaspecPeriodError.md)
- [MsaspecPeriodIdsRequest](./MsaspecPeriodIdsRequest.md)
- [MsaspecPeriodMetaInfo](./MsaspecPeriodMetaInfo.md)
- [MsaspecPeriodPaging](./MsaspecPeriodPaging.md)
- [MsaspecPeriodQueryResponse](./MsaspecPeriodQueryResponse.md)
- [MsaspecPeriodResponseFields](./MsaspecPeriodResponseFields.md)
- [MsaspecPeriodWrites](./MsaspecPeriodWrites.md)
- [Oauth2AccessTokenRequest](./Oauth2AccessTokenRequest.md)
- [PatterndispositionPeriodPatternDisposition](./PatterndispositionPeriodPatternDisposition.md)
- [PeriodResource](./PeriodResource.md)
- [PeriodResources](./PeriodResources.md)
- [PolicyPeriodSensorUpdateSchedule](./PolicyPeriodSensorUpdateSchedule.md)
- [PolicyPeriodSensorUpdateScheduler](./PolicyPeriodSensorUpdateScheduler.md)
- [PreventionPeriodCategoryRespV1](./PreventionPeriodCategoryRespV1.md)
- [PreventionPeriodCreatePoliciesReqV1](./PreventionPeriodCreatePoliciesReqV1.md)
- [PreventionPeriodCreatePolicyReqV1](./PreventionPeriodCreatePolicyReqV1.md)
- [PreventionPeriodPolicyV1](./PreventionPeriodPolicyV1.md)
- [PreventionPeriodRespV1](./PreventionPeriodRespV1.md)
- [PreventionPeriodSettingReqV1](./PreventionPeriodSettingReqV1.md)
- [PreventionPeriodSettingRespV1](./PreventionPeriodSettingRespV1.md)
- [PreventionPeriodUpdatePoliciesReqV1](./PreventionPeriodUpdatePoliciesReqV1.md)
- [PreventionPeriodUpdatePolicyReqV1](./PreventionPeriodUpdatePolicyReqV1.md)
- [ProcessesapiPeriodMsaProcessDetailResponse](./ProcessesapiPeriodMsaProcessDetailResponse.md)
- [ProcessesapiPeriodProcessDetail](./ProcessesapiPeriodProcessDetail.md)
- [QuarantinePeriodQuarantinedFile](./QuarantinePeriodQuarantinedFile.md)
- [QuarantinePeriodQuarantinedFilePath](./QuarantinePeriodQuarantinedFilePath.md)
- [ReconmsaPeriodApiError](./ReconmsaPeriodApiError.md)
- [ReconmsaPeriodApiErrorDetail](./ReconmsaPeriodApiErrorDetail.md)
- [RegistrationPeriodAwsAccountConsoleUrl](./RegistrationPeriodAwsAccountConsoleUrl.md)
- [RegistrationPeriodAwsAccountCreateRequestD4CExtV2](./RegistrationPeriodAwsAccountCreateRequestD4CExtV2.md)
- [RegistrationPeriodAwsAccountCreateRequestExtV2](./RegistrationPeriodAwsAccountCreateRequestExtV2.md)
- [RegistrationPeriodAwsAccountD4CExtV2](./RegistrationPeriodAwsAccountD4CExtV2.md)
- [RegistrationPeriodAwsAccountExtV2](./RegistrationPeriodAwsAccountExtV2.md)
- [RegistrationPeriodAwsAccountPatch](./RegistrationPeriodAwsAccountPatch.md)
- [RegistrationPeriodAwsAccountPatchRequest](./RegistrationPeriodAwsAccountPatchRequest.md)
- [RegistrationPeriodAwsAccountResponseV2](./RegistrationPeriodAwsAccountResponseV2.md)
- [RegistrationPeriodAwsAccountScript](./RegistrationPeriodAwsAccountScript.md)
- [RegistrationPeriodAwsProvisionGetAccountScriptResponseV2](./RegistrationPeriodAwsProvisionGetAccountScriptResponseV2.md)
- [RegistrationPeriodAzureAccountCreateRequestExternalV1](./RegistrationPeriodAzureAccountCreateRequestExternalV1.md)
- [RegistrationPeriodAzureAccountExternalV1](./RegistrationPeriodAzureAccountExternalV1.md)
- [RegistrationPeriodAzureAccountResponseV1](./RegistrationPeriodAzureAccountResponseV1.md)
- [RegistrationPeriodAzureAccountV1Ext](./RegistrationPeriodAzureAccountV1Ext.md)
- [RegistrationPeriodAzureDownloadCertificateResponseV1](./RegistrationPeriodAzureDownloadCertificateResponseV1.md)
- [RegistrationPeriodAzureKeyV1](./RegistrationPeriodAzureKeyV1.md)
- [RegistrationPeriodAzureProvisionGetUserScriptResponseV1](./RegistrationPeriodAzureProvisionGetUserScriptResponseV1.md)
- [RegistrationPeriodAzureTenantConfigurationResponseV1](./RegistrationPeriodAzureTenantConfigurationResponseV1.md)
- [RegistrationPeriodAzureTenantDefaultSubscriptionIdResponseV1](./RegistrationPeriodAzureTenantDefaultSubscriptionIdResponseV1.md)
- [RegistrationPeriodAzureTenantIdsResponseV1](./RegistrationPeriodAzureTenantIdsResponseV1.md)
- [RegistrationPeriodAzureTenantIdsResponseV1PeriodResources](./RegistrationPeriodAzureTenantIdsResponseV1PeriodResources.md)
- [RegistrationPeriodAzureUserScript](./RegistrationPeriodAzureUserScript.md)
- [RegistrationPeriodExternalIoaEventResponse](./RegistrationPeriodExternalIoaEventResponse.md)
- [RegistrationPeriodExternalIoaResources](./RegistrationPeriodExternalIoaResources.md)
- [RegistrationPeriodExternalIomEventResponse](./RegistrationPeriodExternalIomEventResponse.md)
- [RegistrationPeriodExternalIomEventResponseV2](./RegistrationPeriodExternalIomEventResponseV2.md)
- [RegistrationPeriodGcpAccountCreateRequestExtV1](./RegistrationPeriodGcpAccountCreateRequestExtV1.md)
- [RegistrationPeriodGcpAccountExtV1](./RegistrationPeriodGcpAccountExtV1.md)
- [RegistrationPeriodGcpAccountResponseV1](./RegistrationPeriodGcpAccountResponseV1.md)
- [RegistrationPeriodGcpProvisionGetUserScriptResponseV1](./RegistrationPeriodGcpProvisionGetUserScriptResponseV1.md)
- [RegistrationPeriodGcpUserScript](./RegistrationPeriodGcpUserScript.md)
- [RegistrationPeriodIoaEvent](./RegistrationPeriodIoaEvent.md)
- [RegistrationPeriodIomEvent](./RegistrationPeriodIomEvent.md)
- [RegistrationPeriodIomEventIdsResponseV2](./RegistrationPeriodIomEventIdsResponseV2.md)
- [RegistrationPeriodIomEventV2](./RegistrationPeriodIomEventV2.md)
- [RegistrationPeriodIomResources](./RegistrationPeriodIomResources.md)
- [RegistrationPeriodMsaMetaInfoExtension](./RegistrationPeriodMsaMetaInfoExtension.md)
- [RegistrationPeriodMsaPagingExtension](./RegistrationPeriodMsaPagingExtension.md)
- [RegistrationPeriodMsaSpecMetaInfoExtension](./RegistrationPeriodMsaSpecMetaInfoExtension.md)
- [RegistrationPeriodPolicyExtV1](./RegistrationPeriodPolicyExtV1.md)
- [RegistrationPeriodPolicyRequestExtV1](./RegistrationPeriodPolicyRequestExtV1.md)
- [RegistrationPeriodPolicyResponseV1](./RegistrationPeriodPolicyResponseV1.md)
- [RegistrationPeriodPolicySettingsResponseV1](./RegistrationPeriodPolicySettingsResponseV1.md)
- [RegistrationPeriodScanScheduleResponseV1](./RegistrationPeriodScanScheduleResponseV1.md)
- [RegistrationPeriodScanScheduleUpdateRequestV1](./RegistrationPeriodScanScheduleUpdateRequestV1.md)
- [RegistrationPeriodStaticScriptsResponse](./RegistrationPeriodStaticScriptsResponse.md)
- [RegistryassessmentPeriodExternalCredPayload](./RegistryassessmentPeriodExternalCredPayload.md)
- [RegistryassessmentPeriodExternalRegistryPatchPayload](./RegistryassessmentPeriodExternalRegistryPatchPayload.md)
- [RegistryassessmentPeriodExternalRegistryPayload](./RegistryassessmentPeriodExternalRegistryPayload.md)
- [RemoteResponsePeriodCreatePoliciesV1](./RemoteResponsePeriodCreatePoliciesV1.md)
- [RemoteResponsePeriodCreatePolicyReqV1](./RemoteResponsePeriodCreatePolicyReqV1.md)
- [RemoteResponsePeriodPolicyV1](./RemoteResponsePeriodPolicyV1.md)
- [RemoteResponsePeriodRespV1](./RemoteResponsePeriodRespV1.md)
- [RemoteResponsePeriodUpdatePoliciesReqV1](./RemoteResponsePeriodUpdatePoliciesReqV1.md)
- [RemoteResponsePeriodUpdatePolicyReqV1](./RemoteResponsePeriodUpdatePolicyReqV1.md)
- [SadomainPeriodCreateRuleRequestV1](./SadomainPeriodCreateRuleRequestV1.md)
- [SadomainPeriodCustomerAssets](./SadomainPeriodCustomerAssets.md)
- [SadomainPeriodNotificationLog](./SadomainPeriodNotificationLog.md)
- [SadomainPeriodRule](./SadomainPeriodRule.md)
- [SadomainPeriodSubmitForBlockingInfo](./SadomainPeriodSubmitForBlockingInfo.md)
- [SadomainPeriodTyposquattingBaseDomain](./SadomainPeriodTyposquattingBaseDomain.md)
- [SadomainPeriodTyposquattingComponent](./SadomainPeriodTyposquattingComponent.md)
- [SadomainPeriodTyposquattingParentDomain](./SadomainPeriodTyposquattingParentDomain.md)
- [SadomainPeriodWhoisRecord](./SadomainPeriodWhoisRecord.md)
- [SadomainPeriodWhoisRegistrant](./SadomainPeriodWhoisRegistrant.md)
- [SadomainPeriodWhoisRegistrar](./SadomainPeriodWhoisRegistrar.md)
- [SchemaPeriodSensorEvent](./SchemaPeriodSensorEvent.md)
- [SchemaPeriodSensorEventField](./SchemaPeriodSensorEventField.md)
- [SchemaPeriodSensorEventResponseV1](./SchemaPeriodSensorEventResponseV1.md)
- [SchemaPeriodSensorField](./SchemaPeriodSensorField.md)
- [SchemaPeriodSensorFieldResponseV1](./SchemaPeriodSensorFieldResponseV1.md)
- [SchemaPeriodSensorFieldValue](./SchemaPeriodSensorFieldValue.md)
- [SensorUpdatePeriodBuildReqV1](./SensorUpdatePeriodBuildReqV1.md)
- [SensorUpdatePeriodBuildRespV1](./SensorUpdatePeriodBuildRespV1.md)
- [SensorUpdatePeriodBuildsRespV1](./SensorUpdatePeriodBuildsRespV1.md)
- [SensorUpdatePeriodCreatePoliciesReqV1](./SensorUpdatePeriodCreatePoliciesReqV1.md)
- [SensorUpdatePeriodCreatePoliciesReqV2](./SensorUpdatePeriodCreatePoliciesReqV2.md)
- [SensorUpdatePeriodCreatePolicyReqV1](./SensorUpdatePeriodCreatePolicyReqV1.md)
- [SensorUpdatePeriodCreatePolicyReqV2](./SensorUpdatePeriodCreatePolicyReqV2.md)
- [SensorUpdatePeriodKernelRespV1](./SensorUpdatePeriodKernelRespV1.md)
- [SensorUpdatePeriodKernelsRespV1](./SensorUpdatePeriodKernelsRespV1.md)
- [SensorUpdatePeriodPolicyV1](./SensorUpdatePeriodPolicyV1.md)
- [SensorUpdatePeriodPolicyV2](./SensorUpdatePeriodPolicyV2.md)
- [SensorUpdatePeriodRespV1](./SensorUpdatePeriodRespV1.md)
- [SensorUpdatePeriodRespV2](./SensorUpdatePeriodRespV2.md)
- [SensorUpdatePeriodSettingsReqV1](./SensorUpdatePeriodSettingsReqV1.md)
- [SensorUpdatePeriodSettingsReqV2](./SensorUpdatePeriodSettingsReqV2.md)
- [SensorUpdatePeriodSettingsRespV1](./SensorUpdatePeriodSettingsRespV1.md)
- [SensorUpdatePeriodSettingsRespV2](./SensorUpdatePeriodSettingsRespV2.md)
- [SensorUpdatePeriodUpdatePoliciesReqV1](./SensorUpdatePeriodUpdatePoliciesReqV1.md)
- [SensorUpdatePeriodUpdatePoliciesReqV2](./SensorUpdatePeriodUpdatePoliciesReqV2.md)
- [SensorUpdatePeriodUpdatePolicyReqV1](./SensorUpdatePeriodUpdatePolicyReqV1.md)
- [SensorUpdatePeriodUpdatePolicyReqV2](./SensorUpdatePeriodUpdatePolicyReqV2.md)
- [StatePeriodOnlineStateRespV1](./StatePeriodOnlineStateRespV1.md)
- [StatePeriodOnlineStateResultV1](./StatePeriodOnlineStateResultV1.md)
- [SvExclusionsPeriodCreateReqV1](./SvExclusionsPeriodCreateReqV1.md)
- [SvExclusionsPeriodRespV1](./SvExclusionsPeriodRespV1.md)
- [SvExclusionsPeriodSvExclusionV1](./SvExclusionsPeriodSvExclusionV1.md)
- [SvExclusionsPeriodUpdateReqV1](./SvExclusionsPeriodUpdateReqV1.md)
- [UninstallTokenPeriodRespV1](./UninstallTokenPeriodRespV1.md)
- [UninstallTokenPeriodRevealUninstallTokenReqV1](./UninstallTokenPeriodRevealUninstallTokenReqV1.md)
- [UninstallTokenPeriodUninstallTokenV1](./UninstallTokenPeriodUninstallTokenV1.md)
- [UploadSampleV2Request](./UploadSampleV2Request.md)
