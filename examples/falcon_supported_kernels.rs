use clap::Parser;
use rusty_falcon::apis::sensor_update_policies_api;
use rusty_falcon::easy::client::FalconHandle;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    distro: Option<String>,

    #[arg(short, long)]
    arch: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    let mut distro = String::new();
    if args.distro.is_none() {
        println!("Missing --distro command-line option. Available distributions are: [amzn1, amzn2, debian9, elrepo7, elrepo8, oracle6, oracle7, oracle8, rhel6, rhel7, rhel8, rhel9, suse11, suse12, suse15, ubuntu14, ubuntu16, ubuntu18, ubuntu20, ubuntu22]");
        print!("Selected distro: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        distro.push_str(input.trim());
    }

    let mut arch = String::new();
    if args.arch.is_none() {
        println!("Missing --arch command-line option. Available architectures are: [aarch64, s390x, x86_64]");
        print!("Selected architecture: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        arch.push_str(input.trim());
    }

    let mut filter = String::new();
    if args.distro.is_some() && args.arch.is_some() {
        filter.push_str(
            format!(
                "distro:'{}'+architecture:'{}'",
                args.distro.as_deref().unwrap_or_default(),
                args.arch.as_deref().unwrap_or_default()
            )
            .as_str(),
        );
    } else {
        filter.push_str(format!("distro:'{distro}'+architecture:'{arch}'").as_str());
    }

    let offset = 0;
    let limit = 100;
    let response = sensor_update_policies_api::query_combined_sensor_update_kernels(
        &falcon.cfg,
        Some(filter.as_str()),
        Some(offset),
        Some(limit),
    )
    .await
    .expect("Could not fetch sensor update policy.");

    if !response.errors.is_empty() {
        eprintln!(
            "Errors occurred while getting Falcon CCID: {:?}",
            response.errors
        );
    }

    if response.resources.is_none() {
        eprintln!("No CCID returned");
        return;
    }

    let releases = response
        .resources
        .expect("Could not find the releases.")
        .into_iter()
        .map(|obj| obj.release)
        .collect::<Vec<String>>();

    let json = serde_json::to_string_pretty(&releases).unwrap();
    println!("{json}");
}
