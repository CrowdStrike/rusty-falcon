use clap::Parser;
use rusty_falcon::{
    apis::discover_api::{get_hosts, query_hosts},
    easy::client::FalconHandle,
};
use std::convert::{TryFrom, TryInto};

// We set API limits to a constant value as the `get_hosts` takes max 100 elements.
const LIMIT: i32 = 100;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filter: Option<String>,

    #[arg(short, long)]
    sort: String,

    #[arg(short, long)]
    query: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let mut details = vec![];
    let mut offset = 0usize;

    loop {
        // Use try_into() for safe conversion from usize to i32
        let offset_i32: i32 = offset.try_into().expect("Offset exceeded i32 range");

        let response = query_hosts(
            &falcon.cfg,
            Some(offset_i32),
            Some(LIMIT),
            Some(args.sort.as_str()),
            args.filter.as_deref(),
        )
        .await
        .expect("Could not fetch CCID");

        if response.errors.is_some() {
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

        offset += response.resources.len();

        let batch_details = get_hosts(&falcon.cfg, response.resources)
            .await
            .map(|entities| entities.resources.into_iter().collect::<Vec<_>>())
            .expect("Couldn't fetch hosts details.");
        details.extend(batch_details);

        match response.meta.pagination {
            Some(pagination) => {
                // Use `try_from()` to safely convert `i64` to `usize`
                if let Ok(total_usize) = usize::try_from(pagination.total) {
                    if offset < total_usize {
                        continue;
                    }
                }
                break;
            }
            _ => break,
        };
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&details).expect("Couldn't convert the data to json.")
    );
}
