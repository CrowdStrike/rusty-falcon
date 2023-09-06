use clap::Parser;

use rusty_falcon::{
    apis::{
        hosts_api::query_devices_by_filter_scroll, zero_trust_assessment_api::get_assessment_v1,
    },
    easy::client::FalconHandle,
};

// We set API limits to a constant value as the `get_hosts` takes max 100 elements.
const LIMIT: i32 = 100;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let mut resources = vec![];
    let mut offset: Option<String> = None;
    loop {
        let hosts = query_devices_by_filter_scroll(
            &falcon.cfg,
            offset.as_deref(),
            Some(LIMIT),
            None,
            args.filter.as_deref(),
        )
        .await
        .expect("Couldn't fetch hosts");

        // TODO: It appears that this API returns `aid`s which have no assessment available as errors, fix it.
        let response = get_assessment_v1(&falcon.cfg, hosts.resources)
            .await
            .expect("Couldn't fetch statistics");

        resources.extend(response.resources);

        offset = match hosts.meta.pagination {
            Some(pagination) if !pagination.offset.is_empty() => Some(pagination.offset),
            _ => break,
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&resources).expect("Couldn't convert the data to json.")
    );
}
