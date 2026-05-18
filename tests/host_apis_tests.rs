mod common;

#[cfg(test)]
mod query_combined_host_groups_tests {
    use rusty_falcon::apis::hosts_api;
    use serde_json::json;
    use wiremock::http::Method;

    use crate::common::{create_test_config, setup_mock};

    #[tokio::test]
    async fn test_get_device_details_v2_success() {
        let response = json!(
            {
                "meta": {
                    "query_time": 0.1,
                    "pagination": {
                    "offset": 1,
                    "limit": 1,
                    "total": 1
                    },
                    "trace_id": "12345"
                },
                "errors": null,
                "resources": [
                    {
                        "cid": "test-customer-id-123",
                        "device_id": "12345",
                        "hostname": "TEST-hostname-01"
                    }
                ]
            }
        );
        let mock_server =
            setup_mock(Method::GET, "/devices/entities/devices/v2", 200, response).await;

        let configuration = create_test_config(&mock_server);
        let ids = vec!["123".to_owned()];

        let response = hosts_api::get_device_details_v2(&configuration, ids)
            .await
            .expect("API call should succeed");

        assert!(response.errors.is_empty());
        assert_eq!(response.resources.len(), 1);

        let device1 = &response.resources[0];
        assert_eq!(device1.cid, "test-customer-id-123");
        assert_eq!(device1.device_id, "12345");
        assert_eq!(device1.hostname, Some("TEST-hostname-01".to_string()));

        assert_eq!(response.meta.trace_id, "12345".to_string());
    }

    #[tokio::test]
    async fn test_get_device_details_v2_failure() {
        let error_response = json!(
            {
                "meta": {
                    "query_time": 0.002,
                    "trace_id": "error-trace-12345"
                },
                "errors": [
                    {
                        "code": 403,
                        "message": "access denied, authorization failed"
                    }
                ]
            }
        );
        let mock_server = setup_mock(
            Method::GET,
            "/devices/entities/devices/v2",
            403,
            error_response,
        )
        .await;

        let configuration = create_test_config(&mock_server);
        let ids = vec!["123".to_owned()];
        let result = hosts_api::get_device_details_v2(&configuration, ids).await;

        assert!(result.is_err(), "Expected an error response");

        let error = result.unwrap_err();
        match error {
            rusty_falcon::apis::Error::ResponseError(response_content) => {
                assert_eq!(response_content.status, 403);
                assert!(response_content.content.contains("access denied"));
            }
            _ => panic!("Expected ResponseError, got: {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_query_devices_by_filter_scroll_success() {
        let response = json!(
            {
                "meta": {
                    "query_time": 0.1,
                    "pagination": {
                        "offset": "abcd1234",
                        "limit": 100,
                        "total": 3
                    },
                    "trace_id": "scroll-trace-12345"
                },
                "errors": [],
                "resources": [
                    "device-id-12345",
                    "device-id-67890",
                    "device-id-abcde"
                ]
            }
        );
        let mock_server = setup_mock(
            Method::GET,
            "/devices/queries/devices-scroll/v1",
            200,
            response,
        )
        .await;

        let configuration = create_test_config(&mock_server);

        let response =
            hosts_api::query_devices_by_filter_scroll(&configuration, None, None, None, None)
                .await
                .expect("API call should succeed");

        assert!(response.errors.is_empty());
        assert_eq!(response.resources.len(), 3);

        assert_eq!(response.resources[0], "device-id-12345");
        assert_eq!(response.resources[1], "device-id-67890");
        assert_eq!(response.resources[2], "device-id-abcde");
        assert_eq!(response.meta.trace_id, "scroll-trace-12345".to_string());
    }

    #[tokio::test]
    async fn test_query_devices_by_filter_scroll_failure() {
        let error_response = json!(
            {
                "meta": {
                    "query_time": 0.002,
                    "trace_id": "error-trace-scroll-99999"
                },
                "errors": [
                    {
                        "code": 403,
                        "message": "access denied, authorization failed"
                    }
                ]
            }
        );
        let mock_server = setup_mock(
            Method::GET,
            "/devices/queries/devices-scroll/v1",
            403,
            error_response,
        )
        .await;

        let configuration = create_test_config(&mock_server);

        let result =
            hosts_api::query_devices_by_filter_scroll(&configuration, None, None, None, None).await;

        assert!(result.is_err(), "Expected an error response");

        let error = result.unwrap_err();
        match error {
            rusty_falcon::apis::Error::ResponseError(response_content) => {
                assert_eq!(response_content.status, 403);
                assert!(response_content.content.contains("access denied"));
            }
            _ => panic!("Expected ResponseError, got: {:?}", error),
        }
    }
}
