use rusty_falcon::apis::configuration;
use rusty_falcon::apis::vulnerabilities_api;
use rusty_falcon::easy::client::FalconHandle;
use rusty_falcon::models;
use std::error;
use std::fmt;

#[tokio::main]
async fn main() {
    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    // Learn about available filters at https://falcon.crowdstrike.com/documentation/98/spotlight-apis#spotlight-api-filter-parameters
    let filter = "status:'open'";
    let order = Some("updated_timestamp.asc");
    let mut after: Option<String> = None;

    print!("[");
    let mut empty = true;
    loop {
        let response = get_vulnerabilities(
            &falcon.cfg,
            order,
            filter,
            after.as_ref().map(String::as_str),
        )
        .await
        .expect("Could not list vulnerabilities");

        if response.resources.is_empty() {
            break;
        }

        // Print received vulnerabilities as a json
        for vulnerability in response.resources {
            let json = serde_json::json!(vulnerability);
            if !empty {
                print!(",");
            }
            print!("{}", json);
            empty = false;
        }

        after = match response.meta.next() {
            None => break,
            Some(pagination_token) => Some(pagination_token.to_owned()),
        };
    }
    print!("]")
}

async fn get_vulnerabilities(
    cfg: &configuration::Configuration,
    sort: Option<&str>,
    filter: &str,
    after: Option<&str>,
) -> Result<models::DomainPeriodSpapiCombinedVulnerabilitiesResponse, Box<dyn error::Error>> {
    let mut response = vulnerabilities_api::combined_query_vulnerabilities(
        cfg,
        filter,
        after,
        Some(5000),
        sort,
        None,
    )
    .await?;
    let errors = match response.errors {
        None => Vec::new(),
        Some(errors) => errors,
    };
    if !errors.is_empty() {
        return Err(ApiError(format!(
            "while listing Spotlight Vulnerabilities: '{:?}'",
            errors
        ))
        .into());
    }
    response.errors = None;
    return Ok(response);
}

pub trait PaginationHelper {
    fn next(&self) -> Option<&str>;
}

impl PaginationHelper for models::DomainPeriodSpapiQueryMeta {
    fn next(&self) -> Option<&str> {
        return match &self.pagination {
            None => None,
            Some(pagination) => Some(&pagination.after),
        };
    }
}

#[derive(Debug, Clone)]
pub struct ApiError(pub String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application Errors: {}", self.to_string())
    }
}

impl error::Error for ApiError {}
