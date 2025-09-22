use rusty_falcon::apis::configuration;
use rusty_falcon::apis::hosts_api;
use rusty_falcon::easy::client::FalconHandle;
use rusty_falcon::models;
use std::error;
use std::fmt;

#[tokio::main]
async fn main() {
    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let hosts = get_all_hosts(&falcon.cfg, None, None)
        .await
        .expect("Could not list devices");
    let all_host_details = serde_json::json!(hosts);
    print!("{all_host_details}");
}

async fn get_all_hosts(
    configuration: &configuration::Configuration,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<Vec<models::DeviceapiDeviceSwagger>, Box<dyn error::Error>> {
    let mut details = Vec::<models::DeviceapiDeviceSwagger>::new();
    let mut offset = String::new();
    loop {
        let response =
            query_devices_by_filter_offset(configuration, sort, filter, offset.as_str()).await?;
        let resources_count = response.resources.len();
        if resources_count == 0 {
            break;
        }
        offset = response.resources[resources_count - 1].to_string();
        details.append(&mut get_device_details(configuration, &response.resources).await?);
        if resources_count <= 5000 {
            break;
        }
    }
    Ok(details)
}

async fn get_device_details(
    configuration: &configuration::Configuration,
    ids: &[String],
) -> Result<Vec<models::DeviceapiDeviceSwagger>, Box<dyn error::Error>> {
    let response = hosts_api::post_device_details_v2(
        configuration,
        models::MsaIdsRequest::new(ids.to_owned()),
    )
    .await?;

    if response.errors.is_some() {
        return Err(ApiError(format!(
            "while getting Falcon Host IDs: '{:?}'",
            response.errors
        ))
        .into());
    }

    Ok(response.resources)
}

async fn query_devices_by_filter_offset(
    configuration: &configuration::Configuration,
    sort: Option<&str>,
    filter: Option<&str>,
    offset: &str,
) -> Result<models::DeviceapiDeviceResponse, Box<dyn error::Error>> {
    let response = hosts_api::query_devices_by_filter_scroll(
        configuration,
        Some(offset),
        Some(5000),
        sort,
        filter,
    )
    .await?;
    if !response.errors.is_empty() {
        return Err(ApiError(format!(
            "while getting Falcon Host IDs: '{:?}'",
            response.errors
        ))
        .into());
    }
    Ok(response)
}

#[derive(Debug, Clone)]
pub struct ApiError(pub String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application Errors: {}", self.0)
    }
}

impl error::Error for ApiError {}
