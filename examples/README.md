# rusty_falcon code samples

This directory contains useful code patterns for using rusty_falcon and communicating with CrowdStrike Falcon API.

Some examples available here are rather similar to the [gofalcon (Golang-based SDK) examples](https://github.com/CrowdStrike/gofalcon/tree/main/examples). Experienced golang users may find it useful to compare rusty_falcon and gofalcon counterparts.

## simple

[simple.rs](simple.rs)

Minimalist example to show case authentication and initialisation of client library. Upon successful authentication the very latest [CrowdStrike Score](https://www.crowdstrike.com/blog/tech-center/crowdscore-efficiency/) is shown.

Example run:
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example simple
```

## falcon_get_cid

[falcon_get_cid.rs](falcon_get_cid.rs)

This is stand-alone tool that can be used to get Customer ID based on the API key pair. The ability to derive CID from API key pair is useful in various occasions like new sensor registration.

Example run:
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_get_cid
```

## falcon_host_details

[falcon_host_details.rs](falcon_host_details.rs)

Stand-alone tool that uses Host API to query all the host details and output JSON to the stdout. This tool can be used together with JSON parsing tools like jq in order to build reports of your liking.

Get full details of all the hosts
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq
```

Get total number of hosts
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq length
```

List hostnames of all hosts
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq 'map(.hostname)'
```

List hosts and agent versions as key-value dictionary
```
FALCON_CLIENT_ID="abc" FALCON_CLIENT_SECRET="XYZ" FALCON_CLOUD="us-2" \
    cargo run  --example falcon_host_details | jq -r ' map( { (.hostname) : .agent_version } ) | add'
```
