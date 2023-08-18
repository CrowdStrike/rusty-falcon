use rusty_falcon::apis::sensor_download_api::get_sensor_installers_ccidby_query;
use rusty_falcon::easy::client::FalconHandle;

#[tokio::main]
async fn main() {
    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let response = get_sensor_installers_ccidby_query(&falcon.cfg)
        .await
        .expect("Could not fetch CCID");

    if response.errors.is_some() {
        eprintln!(
            "Errors occurred while getting Falcon CCID: {:?}",
            response.errors
        );
    }
    if response.resources.is_empty() {
        eprintln!("No CCID returned");
    }
    print!("{}", response.resources[0]);
}
