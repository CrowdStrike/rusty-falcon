use clap::Parser;

use rusty_falcon::{apis::intel_api::query_intel_indicator_entities, easy::client::FalconHandle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter: String,

    #[arg(short, long)]
    sort: String,

    #[arg(short, long)]
    query: Option<String>,

    #[arg(short = 'd', long)]
    include_deleted: Option<bool>,

    #[arg(short = 'r', long)]
    include_relations: Option<bool>,

    #[arg(short, long, default_value_t = 1000, value_parser = clap::value_parser!(u16).range(1..=10000))]
    limit: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let response = query_intel_indicator_entities(
        &falcon.cfg,
        // Restricted API, can only start with offset `0` or `None`.
        None,
        Some(args.limit.into()),
        Some(args.sort.as_str()),
        Some(args.filter.as_str()),
        args.query.as_deref(),
        args.include_deleted,
        args.include_relations,
    )
    .await
    .expect("Could not fetch CCID");

    if !response.errors.is_empty() {
        eprintln!(
            "Errors occurred while getting Falcon CCID: {:?}",
            response.errors
        );
    }

    if response.resources.is_empty() {
        eprintln!("No CCID returned");
    }

    let json = serde_json::to_string_pretty(&response.resources).unwrap();
    println!("{json}");
}
