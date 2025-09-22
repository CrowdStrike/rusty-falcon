use clap::Parser;
use rusty_falcon::apis::sensor_update_policies_api;
use rusty_falcon::easy::client::FalconHandle;
use rusty_falcon::models::SensorUpdateKernelRespV1;
use std::collections::HashSet;
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

    let offset = 0;
    let limit = 100;
    let kernels = sensor_update_policies_api::query_combined_sensor_update_kernels(
        &falcon.cfg,
        None,
        Some(offset),
        Some(limit),
    )
    .await
    .expect("Could not fetch sensor update policy.");

    let mut arch_set = HashSet::new();
    let mut distro_set = HashSet::new();
    if let Some(kernel) = kernels
        .resources
        .as_ref()
        .and_then(|kernels| kernels.first())
    {
        let SensorUpdateKernelRespV1 {
            architecture,
            distro,
            ..
        } = kernel;
        arch_set.insert(architecture);
        distro_set.insert(distro);
    };

    let mut valid_archs = Vec::from_iter(arch_set);
    let mut valid_distros = Vec::from_iter(distro_set);
    valid_archs.sort_by_key(|name| name.to_lowercase());
    valid_distros.sort_by_key(|name| name.to_lowercase());

    let mut distro = String::new();
    if args.distro.is_none() {
        println!(
            "Missing --distro command-line option. Available distributions are: {valid_distros:?}"
        );
        print!("Selected distro: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        distro.push_str(input.trim());
    }

    let mut arch = String::new();
    if args.arch.is_none() {
        println!(
            "Missing --arch command-line option. Available architectures are: {valid_archs:?}"
        );
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

    if let Some(errors) = response.errors.filter(|errors| !errors.is_empty()) {
        eprintln!("Errors occurred while getting Falcon CCID: {:?}", errors);
    }

    let Some(resources) = response.resources else {
        eprintln!("No CCID returned");
        return;
    };

    let releases = resources
        .into_iter()
        .map(|obj| obj.release)
        .collect::<Vec<String>>();

    let json = serde_json::to_string_pretty(&releases).unwrap();
    println!("{json}");
}
