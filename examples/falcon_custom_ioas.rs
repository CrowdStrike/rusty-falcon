use clap::Parser;

use rusty_falcon::{
    apis::custom_ioa_api::{get_rule_groups_mixin0, query_rule_groups_mixin0},
    easy::client::FalconHandle,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter: Option<String>,

    #[arg(short, long)]
    sort: Option<String>,

    #[arg(short, long)]
    query: Option<String>,

    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u16).range(1..=500))]
    limit: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let mut details = vec![];
    let mut offset: i64 = 0;
    loop {
        let response = query_rule_groups_mixin0(
            &falcon.cfg,
            args.sort.as_deref(),
            args.filter.as_deref(),
            args.query.as_deref(),
            Some(offset.to_string().as_str()),
            Some(args.limit.into()),
        )
        .await
        .expect("Could not fetch CCID");

        if !response.errors.is_empty() {
            eprintln!(
                "Errors occurred while getting Falcon CCID: {:?}",
                response.errors
            );
            std::process::exit(1);
        }

        if response.resources.is_empty() {
            eprintln!("No CCID returned");
            break;
        }

        let details_response = get_rule_groups_mixin0(&falcon.cfg, response.resources)
            .await
            .map(|detail| detail.resources.into_iter().collect::<Vec<_>>());
        details.extend(details_response);

        offset = match response.meta.pagination {
            Some(pagination) if i64::from(pagination.offset) < pagination.total => {
                i64::from(pagination.offset)
            }
            _ => break,
        };
    }

    println!("{}", serde_json::to_string_pretty(&details).unwrap());
}
