use rusty_falcon::models::DomainExternalAssetService;
use serde_json::{Value, json};

fn record(status_code: Value) -> Value {
    json!({
        "id": "local-test-service",
        "last_seen": "2026-10-28T00:00:00Z",
        "port": 443,
        "protocol": "https",
        "status": "active",
        "status_code": status_code,
        "transport": "tcp"
    })
}

#[test]
fn integer_status_codes_round_trip_without_coercion() {
    for code in [i32::MIN, 0, 200, i32::MAX] {
        let service = serde_json::from_value::<DomainExternalAssetService>(record(json!(code)))
            .expect("integer status_code must deserialize");
        assert_eq!(serde_json::to_value(service).unwrap()["status_code"], code);
    }
}

#[test]
fn null_status_code_round_trips_without_becoming_zero() {
    let service = serde_json::from_value::<DomainExternalAssetService>(record(Value::Null))
        .expect("reported null status_code must deserialize");
    let serialized = serde_json::to_value(service).unwrap();
    assert_eq!(serialized.get("status_code"), Some(&Value::Null));
}

#[test]
fn absent_status_code_is_unknown() {
    let mut input = record(json!(200));
    input.as_object_mut().unwrap().remove("status_code");
    let service = serde_json::from_value::<DomainExternalAssetService>(input)
        .expect("an absent status code must deserialize as unknown");
    assert_eq!(
        serde_json::to_value(service).unwrap()["status_code"],
        Value::Null
    );
}

#[test]
fn invalid_status_codes_remain_errors() {
    for code in [
        json!("200"),
        json!(false),
        json!(1.5),
        json!(2147483648_i64),
        json!({}),
    ] {
        assert!(
            serde_json::from_value::<DomainExternalAssetService>(record(code.clone())).is_err(),
            "invalid status code was accepted: {code}"
        );
    }
}

#[test]
fn null_port_remains_an_error() {
    let mut input = record(json!(200));
    input["port"] = Value::Null;
    assert!(serde_json::from_value::<DomainExternalAssetService>(input).is_err());
}

#[test]
fn existing_integer_constructor_remains_usable() {
    let service = DomainExternalAssetService::new(
        "test-service".into(),
        "2026-10-28T00:00:00Z".into(),
        443,
        "https".into(),
        "active".into(),
        200,
        "tcp".into(),
    );
    assert_eq!(serde_json::to_value(service).unwrap()["status_code"], 200);
}
