# rusty_falcon code samples

This directory contains useful code patterns for using `rusty_falcon` and communicating with `CrowdStrike Falcon API`.

Some examples available here are rather similar to the [gofalcon (Golang-based SDK) examples](https://github.com/CrowdStrike/gofalcon/tree/main/examples). Experienced golang users may find it useful to compare `rusty_falcon` and `gofalcon` counterparts.

## Examples

1. [Simple client usage](#simple)
2. [Get Customer ID](#falcon_get_cid)
3. [Get Host Details](#falcon_host_details)
4. [Spotlight Vulnerabilities](#falcon_spotlight_vulnerabilities)
5. [Intel Indicators](#intel_indicators)
6. [Supported Kernels](#falcon_supported_kernels)
7. [Falcon Custom IOAs](#falcon_custom_ioas)
8. [Discover API Hosts Details](#falcon_discover_hosts)
9. [Falcon ZTA](#falcon_zta)

### simple

[simple.rs](simple.rs)

Minimalist example to show case authentication and initialisation of client library. Upon successful authentication the very latest [CrowdStrike Score](https://www.crowdstrike.com/blog/tech-center/crowdscore-efficiency/) is shown.

Example run:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example simple
```

### falcon_get_cid

[falcon_get_cid.rs](falcon_get_cid.rs)

This is stand-alone tool that can be used to get Customer ID based on the API key pair. The ability to derive CID from API key pair is useful in various occasions like new sensor registration.

Example run:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_get_cid
```

### falcon_host_details

[falcon_host_details.rs](falcon_host_details.rs)

Stand-alone tool that uses Host API to query all the host details and output JSON to the `stdout`. This tool can be used together with JSON parsing tools like jq in order to build reports of your liking.

Get full details of all the hosts

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq
```

Get total number of hosts

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq length
```

List hostnames of all hosts

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq 'map(.hostname)'
```

List hosts and agent versions as key-value dictionary

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq -r ' map( { (.hostname) : .agent_version } ) | add'
```

### falcon_spotlight_vulnerabilities

[falcon_spotlight_vulnerabilities.rs](falcon_spotlight_vulnerabilities.rs)

Stand-alone tool that uses Falcon Spotlight API to query the vulnerabilities affecting your environment and outputs those vulnerabilities in JSON format to the `stdout`. This tool can be used together with JSON parsing tools like `jq` in order to build reports of your liking.

Get all open vulnerabilities affecting your env

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_spotlight_vulnerabilities | jq
```

### intel_indicators

[intel_indicators.rs](intel_indicators.rs)

Stand-alone tool that uses Falcon Intel API to query the indicators affecting your environment in JSON format to the `stdout`. This tool can be used together with JSON parsing tools like `jq` in order to build reports of your liking.

Get indicators example:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
     cargo run --example intel_indicators -- --sort published_date.asc --filter deleted:false -q abc | jq
```

### falcon_supported_kernels

[falcon_supported_kernels.rs](falcon_supported_kernels.rs)

This example shows listing of the supported Linux kernels. The tool outputs short list of recently supported kernels by CrowdStrike Falcon Sensor for Linux on a given distribution.

Supported kernels example:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
     cargo run --example falcon_supported_kernels -- --distro=rhel9 --arch=aarch64
```

### falcon_custom_ioas

[falcon_custom_ioas.rs](falcon_custom_ioas.rs)

This example shows listing of the custom IOAs.
The cli allows to provide parameters to the call to sort or filter the results, more details can be found in the API documentation.

```bash
Options:
  -f, --filter <FILTER>
  -s, --sort <SORT>
  -q, --query <QUERY>
  -l, --limit <LIMIT>    [default: 100]
```

To run the example:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
     cargo run --example falcon_custom_ioas
```

Sorted by `created_on`:

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
cargo run --example falcon_custom_ioas -- --sort created_on
```

Filtered by `enabled`:

```bash
cargo run --example falcon_custom_ioas -- --filter enabled:true
```

### falcon_discover_hosts

[falcon_discover_hosts.rs](falcon_discover_hosts.rs)

This example prints out details for all the hosts on the tenant.

```bash
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
     cargo run --example falcon_discover_hosts -- --sort hostname
```

### falcon_zta

CrowdStrike Falcon ZTA API is available to determine the Falcon ZTA stats for each of the Falcon managed endpoints that can be integrated with existing workflows.

This example showcases use of Falcon ZTA API. To learn more about Falcon ZTA please visit [product announcement](https://www.crowdstrike.com/press-releases/crowdstrike-extends-zero-trust-to-endpoint-devices/). To learn more about the concepts of Zero Trust visit [cybersecurity-101](https://www.crowdstrike.com/cybersecurity-101/zero-trust-security/).

Please refer to [Falcon Zero Trust Assessment APIs](https://falcon.crowdstrike.com/documentation/156/zero-trust-assessment-apis) documentation to learn more about specific fields returned by this API.

Please refer to [Falcon Hosts API documentation](https://falcon.crowdstrike.com/documentation/84/host-and-host-group-management-apis) to learn more about FQL filter parameter, about the meaning of the entity properties, and best practices.

Example usage:

Get ZTA details of each of the hosts

```bash
$ FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD=us-1 \
cargo run --example falcon_zta
```

Get ZTA details for sub-set of hosts specified by FQL (Falcon Query Language). In this case, we query zta details for all the hosts running Linux.

```bash
$ FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD=us-1 \
cargo run --example falcon_zta -- --filter "last_seen:>='2022-01-01'"
```

Get ZTA details for all hosts and transform the data to only show overall score:

```bash
$ FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD=us-1 \
cargo run --example falcon_zta | jq -r 'map( { (.aid) : .assessment.overall } ) | add'
```

Get ZTA details for all the hosts that have been seen in the last 45 days and sort it by ZTA overall score from the worst to the best.

```bash
$ FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD=us-1 \
week_ago=$(date -jf %s $(( $(date +%s) - 86400 * 7 )) +%Y-%m-%d)
cargo run --example falcon_zta -- --filter="last_seen:>='${week_ago}'" | jq -r 'sort_by(.assessment.overall)'
```
