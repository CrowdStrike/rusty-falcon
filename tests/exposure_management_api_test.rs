use rusty_falcon::apis::{configuration::Configuration, exposure_management_api};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

async fn assert_nullable_service_response(asset_type: &str) {
    // Synthetic records exercise the real HTTP/deserialization path without a tenant.
    let services = [None, Some(0), Some(200)].map(|status_code| {
        json!({
            "id": "test-service",
            "last_seen": "2026-10-28T00:00:00Z",
            "port": 443,
            "protocol": "https",
            "status": "active",
            "status_code": status_code,
            "transport": "tcp"
        })
    });
    let mut asset = json!({
        "applicable_actions": [],
        "asset_type": asset_type,
        "cid": "test-customer",
        "confidence": 100,
        "data_providers": [],
        "entity_type": "external",
        "id": "test-asset",
        "status": "active"
    });
    asset[asset_type] = if asset_type == "ip" {
        json!({"ip_address": "192.0.2.1", "services": services})
    } else {
        json!({"fqdn": "example.test", "type": "root", "services": services})
    };
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/fem/entities/external-assets/v1"))
        .and(query_param("ids", "test-asset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "meta": {"query_time": 0.01, "trace_id": "test-trace"},
            "resources": [asset]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let configuration = Configuration {
        base_path: server.uri(),
        client: reqwest::Client::builder().no_proxy().build().unwrap(),
        ..Configuration::default()
    };
    let response =
        exposure_management_api::get_external_assets(&configuration, vec!["test-asset".into()])
            .await
            .expect("a null service status must not discard the asset response");
    assert_eq!(response.resources.len(), 1);
    let asset = &response.resources[0];
    assert_eq!(asset.id, "test-asset");
    assert_eq!(asset.asset_type, asset_type);
    let services = if asset_type == "ip" {
        asset.ip.as_ref().unwrap().services.as_ref().unwrap()
    } else {
        asset
            .dns_domain
            .as_ref()
            .unwrap()
            .services
            .as_ref()
            .unwrap()
    };
    let serialized = serde_json::to_value(services).unwrap();
    let codes: Vec<_> = serialized
        .as_array()
        .unwrap()
        .iter()
        .map(|service| service["status_code"].clone())
        .collect();
    assert_eq!(codes, vec![json!(null), json!(0), json!(200)]);
}

#[tokio::test]
async fn external_ip_services_accept_null_status_code() {
    assert_nullable_service_response("ip").await;
}

#[tokio::test]
async fn dns_domain_services_accept_null_status_code() {
    assert_nullable_service_response("dns_domain").await;
}
