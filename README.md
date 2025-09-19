![CrowdStrike Falcon](https://raw.githubusercontent.com/CrowdStrike/falconpy/main/docs/asset/cs-logo.png) [![Twitter URL](https://img.shields.io/twitter/url?label=Follow%20%40CrowdStrike&style=social&url=https%3A%2F%2Ftwitter.com%2FCrowdStrike)](https://twitter.com/CrowdStrike)<br/>

# rusty_falcon

[![Build CI](https://github.com/CrowdStrike/rusty-falcon/actions/workflows/ci.yaml/badge.svg)](https://github.com/CrowdStrike/rusty-falcon/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/rusty_falcon.svg)](https://crates.io/crates/rusty_falcon)
[![Documentation](https://docs.rs/rusty_falcon/badge.svg)](https://docs.rs/rusty_falcon)

Rust-based SDK to CrowdStrike's Falcon APIs

rusty_falcon documentation is available on [docs.rs](https://docs.rs/rusty_falcon/latest/rusty_falcon/).
Users are advised to consult this rusty_falcon documentation together with the comprehensive CrowdStrike
API documentation published on [Developer Center](https://developer.crowdstrike.com/docs/openapi).
The easiest way to learn about the SDK is to consult the set of
[examples](https://github.com/CrowdStrike/rusty-falcon/tree/main/examples) built on top of the SDK.

# Quick Start

To get you started quickly, the easiest and highest-level way to establish API client is to instantiate
`easy::client::FalconHandle`. The most convenient way is to use `easy::client::FalconHandle::from_env`
function that will read the following environment variables to authenticate with falcon cloud:
`FALCON_CLIENT_ID`, `FALCON_CLIENT_SECRET`, and `FALCON_CLOUD`. Unless you already have a CrowdStrike key
pair you can establish a new one in [Falcon Portal](https://falcon.crowdstrike.com/api-clients-and-keys).

```rust
use rusty_falcon::apis::sensor_download_api;
use rusty_falcon::easy::client::FalconHandle;

#[tokio::main]
async fn main() {
    // Fetch credentials from environment variables and establish OAuth2 connection
    let falcon = FalconHandle::from_env()
        .await
        .expect("Could not authenticate with CrowdStrike API");

    // Call one particular API end-point using the authenticated client
    let response = sensor_download_api::get_sensor_installers_ccidby_query(&falcon.cfg)
        .await
        .expect("Could not fetch CCID");

    // Response objects returned from APIs usually follow the same pattern of having
    // 'errors', 'meta', and 'resources' fields. It is recommended to check for possible
    // application errors:
    if !response.errors.is_empty() {
        eprintln!("Errors occurred while getting Falcon CCID: {:?}", response.errors);
    }

    // Print response from the API:
    println!("{:?}", response.resources)
}
```

# Examples

Ready-made examples can be found in [git repo](https://github.com/CrowdStrike/rusty-falcon/tree/main/examples).

There's a [handy script](https://github.com/CrowdStrike/rusty-falcon/tree/main/scripts/run-examples.sh) that can be used to (sequentially) run and test the examples. This script will show the status (pass / fail) of each example.

```sh
# Run all examples
./scripts/run-examples.sh

# Run a single example
./scripts/run-examples.sh falcon_custom_ioas
```

# [WIP] Project Documentation

This project uses `mdbook` to serve its documentation, to run it locally:

```bash
cd docs
mdbook build
mdbook serve
```

# Generating models from OpenApi Specification

This api model is generated from OpenApi specification using [OpenApi Generator](https://openapi-generator.tech/docs/installation/) for Rust language.
> Note: In the OpenApi specification please update version to `rolling``, if required and update version on the list below.
> This will prevent crazy long PRs with updates.

An example command to generate `api` and `model`:

```bash
openapi-generator generate -g rust -i swagger.json -o ./new
```

## OpenApi Specification Version

`2025-09-19T00:00:03Z`

# Getting Help

rusty_falcon is an open source project, not a CrowdStrike product. As such it carries no formal support,
expressed or implied.

If you encounter any issues while using rusty_falcon, you can create an issue on our
[Github repo](https://github.com/CrowdStrike/rusty-falcon) for bugs, enhancements, or other requests.

rusty_falcon project is periodically refreshed to reflect the newest additions to the CrowdStrike API. Users
of the SDK are advised to track the latest releases rather closely to ensure proper function in the unlikely
event of an incompatible change to a CrowdStrike API.
