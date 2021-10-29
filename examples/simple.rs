use rusty_falcon::apis::{Error};
use rusty_falcon::apis::configuration::{Configuration};
use rusty_falcon::apis::oauth2_api::{oauth2_access_token, Oauth2AccessTokenError};
use rusty_falcon::apis::incidents_api::{crowd_score};

use std::env;

#[tokio::main]
async fn main() {
    let falcon_client_id = env::var("FALCON_CLIENT_ID")
        .expect("Missing FALCON_CLIENT_ID environment variable. Please provide your OAuth2 API Client ID for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys.");
    let falcon_client_secret = env::var("FALCON_CLIENT_SECRET")
        .expect("Missing FALCON_CLIENT_SECRET environment variable. Please provide your OAuth2 API Client Secret for authentication with CrowdStrike Falcon platform. Establishing and retrieving OAuth2 API credentials can be performed at https://falcon.crowdstrike.com/support/api-clients-and-keys.");

    let configuration = new_client(&falcon_client_id, &falcon_client_secret)
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let crowd_score_response = crowd_score(&configuration, None, None, None, None)
        .await
        .expect("Could not fetch CrowdScore");

    if ! crowd_score_response.errors.is_empty() {
        eprintln!("Errors occured while calculating CrowdScore: {:?}", crowd_score_response.errors);
    }

    if crowd_score_response.resources.is_empty() {
        eprintln!("No CrowdScore returned")
    }

    let score = crowd_score_response.resources.last().unwrap();
    println!("As of {} your CrowdScore is {}.", score.timestamp, score.score)
}

async fn new_client(falcon_client_id: &str, falcon_client_secret: &str) -> Result<Configuration, Error<Oauth2AccessTokenError>> {
    let mut configuration = Configuration {
        ..Default::default()
    };

    let response = oauth2_access_token(&configuration, &falcon_client_id, &falcon_client_secret, None).await?;

    configuration.oauth_access_token = Some(response.access_token);
    return Ok(configuration);
}
