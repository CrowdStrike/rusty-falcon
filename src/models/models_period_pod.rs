/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelsPeriodPod {
    #[serde(rename = "agents")]
    pub agents: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "allow_privilege_escalation")]
    pub allow_privilege_escalation: bool,
    #[serde(rename = "annotations_list")]
    pub annotations_list: Vec<String>,
    #[serde(rename = "app_name")]
    pub app_name: String,
    #[serde(rename = "automount_service_token")]
    pub automount_service_token: bool,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cloud_account_id")]
    pub cloud_account_id: String,
    #[serde(rename = "cloud_name")]
    pub cloud_name: String,
    #[serde(rename = "cloud_region")]
    pub cloud_region: String,
    #[serde(rename = "cloud_service")]
    pub cloud_service: String,
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    #[serde(rename = "cluster_name")]
    pub cluster_name: String,
    #[serde(rename = "container_count")]
    pub container_count: i32,
    #[serde(rename = "containers")]
    pub containers: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "first_seen")]
    pub first_seen: String,
    #[serde(rename = "host_ipc")]
    pub host_ipc: bool,
    #[serde(rename = "host_network")]
    pub host_network: bool,
    #[serde(rename = "host_pid")]
    pub host_pid: bool,
    #[serde(rename = "image_pull_secrets")]
    pub image_pull_secrets: Vec<String>,
    #[serde(rename = "ipv4")]
    pub ipv4: String,
    #[serde(rename = "ipv6")]
    pub ipv6: String,
    #[serde(rename = "kac_agent_id")]
    pub kac_agent_id: String,
    #[serde(rename = "labels")]
    pub labels: std::collections::HashMap<String, String>,
    #[serde(rename = "labels_list")]
    pub labels_list: Vec<String>,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "node_ipv4")]
    pub node_ipv4: String,
    #[serde(rename = "node_name")]
    pub node_name: String,
    #[serde(rename = "node_selector")]
    pub node_selector: String,
    #[serde(rename = "node_uid")]
    pub node_uid: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "owner_type")]
    pub owner_type: String,
    #[serde(rename = "pod_external_id")]
    pub pod_external_id: String,
    #[serde(rename = "pod_id")]
    pub pod_id: String,
    #[serde(rename = "pod_name")]
    pub pod_name: String,
    #[serde(rename = "ports")]
    pub ports: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "privileged")]
    pub privileged: bool,
    #[serde(rename = "resource_status")]
    pub resource_status: String,
    #[serde(rename = "root_write_access")]
    pub root_write_access: bool,
    #[serde(rename = "run_as_root_group")]
    pub run_as_root_group: bool,
    #[serde(rename = "run_as_root_user")]
    pub run_as_root_user: bool,
    #[serde(rename = "scheduler_name")]
    pub scheduler_name: String,
    #[serde(rename = "service_account_name")]
    pub service_account_name: String,
    #[serde(rename = "share_process_namespace")]
    pub share_process_namespace: bool,
    #[serde(rename = "volume_mounts")]
    pub volume_mounts: String,
}

impl ModelsPeriodPod {
    pub fn new(
        agents: Vec<std::collections::HashMap<String, String>>,
        allow_privilege_escalation: bool,
        annotations_list: Vec<String>,
        app_name: String,
        automount_service_token: bool,
        cid: String,
        cloud_account_id: String,
        cloud_name: String,
        cloud_region: String,
        cloud_service: String,
        cluster_id: String,
        cluster_name: String,
        container_count: i32,
        containers: Vec<std::collections::HashMap<String, String>>,
        created_at: String,
        first_seen: String,
        host_ipc: bool,
        host_network: bool,
        host_pid: bool,
        image_pull_secrets: Vec<String>,
        ipv4: String,
        ipv6: String,
        kac_agent_id: String,
        labels: std::collections::HashMap<String, String>,
        labels_list: Vec<String>,
        last_seen: String,
        namespace: String,
        node_ipv4: String,
        node_name: String,
        node_selector: String,
        node_uid: String,
        owner_id: String,
        owner_type: String,
        pod_external_id: String,
        pod_id: String,
        pod_name: String,
        ports: Vec<std::collections::HashMap<String, String>>,
        privileged: bool,
        resource_status: String,
        root_write_access: bool,
        run_as_root_group: bool,
        run_as_root_user: bool,
        scheduler_name: String,
        service_account_name: String,
        share_process_namespace: bool,
        volume_mounts: String,
    ) -> ModelsPeriodPod {
        ModelsPeriodPod {
            agents,
            allow_privilege_escalation,
            annotations_list,
            app_name,
            automount_service_token,
            cid,
            cloud_account_id,
            cloud_name,
            cloud_region,
            cloud_service,
            cluster_id,
            cluster_name,
            container_count,
            containers,
            created_at,
            deleted_at: None,
            first_seen,
            host_ipc,
            host_network,
            host_pid,
            image_pull_secrets,
            ipv4,
            ipv6,
            kac_agent_id,
            labels,
            labels_list,
            last_seen,
            namespace,
            node_ipv4,
            node_name,
            node_selector,
            node_uid,
            owner_id,
            owner_type,
            pod_external_id,
            pod_id,
            pod_name,
            ports,
            privileged,
            resource_status,
            root_write_access,
            run_as_root_group,
            run_as_root_user,
            scheduler_name,
            service_account_name,
            share_process_namespace,
            volume_mounts,
        }
    }
}
