use rusty_falcon::apis::configuration;
use rusty_falcon::apis::hosts_api;
use rusty_falcon::easy::client::FalconHandle;
use rusty_falcon::models;
use std::error;
use std::fmt;

#[tokio::main]
async fn main() {
    let falcon = FalconHandle::from_env().await.expect("Could not authenticate with CrowdStrike API");

    let hosts = get_all_hosts(&falcon.cfg, None, None).await.expect("Could not list devices");
    let all_host_details = serde_json::json!(hosts);
    print!("{}", all_host_details);
}

async fn get_all_hosts(configuration: &configuration::Configuration, sort: Option<&str>, filter: Option<&str>) -> Result<Vec<models::DomainDeviceSwagger>, Box<dyn error::Error>> {
    let mut details = Vec::<models::DomainDeviceSwagger>::new();
    for page in 0.. {
        let response = query_devices_by_filter_page(configuration, sort, filter, page).await?;
        if response.resources.len() == 0 {
            break;
        }
        details.append(&mut get_device_details(configuration, response.resources).await?);
        if last_page(response.meta.pagination) {
            break;
        }
    }
    return Ok(details);
}

async fn get_device_details(configuration: &configuration::Configuration, ids: Vec<String>) -> Result<Vec<models::DomainDeviceSwagger>, Box<dyn error::Error>> {
    let response = hosts_api::get_device_details(configuration, ids).await?;
    if !response.errors.is_empty() {
        return Err(ApiError(format!("while getting Falcon Host Details: '{:?}", response.errors)).into());
    }
    return Ok(response.resources);
}

async fn query_devices_by_filter_page(configuration: &configuration::Configuration, sort: Option<&str>, filter: Option<&str>, page: i32) -> Result<models::MsaQueryResponse, Box<dyn error::Error>> {
    let response = hosts_api::query_devices_by_filter(configuration, Some(page * 500), Some(500), sort, filter).await?;
    if !response.errors.is_empty() {
        return Err(ApiError(format!("while getting Falcon Host IDs: '{:?}'", response.errors)).into());
    }
    return Ok(response);
}

fn last_page(pagination: Option<Box<models::MsaPaging>>) -> bool {
    return match pagination {
        None => false,
        Some(p) => (p.total - i64::from(p.offset)) <= 0,
    };
}

#[derive(Debug, Clone)]
pub struct ApiError(pub String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application Errors: {}", self.to_string())
    }
}

impl error::Error for ApiError {}
