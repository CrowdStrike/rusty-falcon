mod common;

#[cfg(test)]
mod host_group_api_tests {
    use rusty_falcon::apis::host_group_api;
    use rusty_falcon::models::host_groups_host_group_v1::GroupType;
    use serde_json::json;
    use wiremock::http::Method;

    use crate::common::{create_test_config, setup_mock};

    #[tokio::test]
    async fn test_query_combined_host_groups_success() {
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
                        "assignment_rule": "some-assignment-rule",
                        "group_type": "dynamic",
                        "created_by": "jdoe@example.com",
                        "created_timestamp": "2023-04-20T17:49:24.601496284Z",
                        "description": "Test web servers in US-EAST region",
                        "id": "sometestid",
                        "modified_by": "admin@example.com",
                        "modified_timestamp": "2024-01-15T14:23:18.412937102Z",
                        "name": "test-web-us-east"
                    }
                ]
            }
        );
        let mock_server = setup_mock(
            Method::GET,
            "/devices/combined/host-groups/v1",
            200,
            response,
        )
        .await;

        let configuration = create_test_config(&mock_server);
        let response =
            host_group_api::query_combined_host_groups(&configuration, None, None, None, None)
                .await
                .expect("API call should succeed");

        assert!(response.errors.is_empty());
        assert_eq!(response.resources.len(), 1);

        let group = &response.resources[0];
        assert_eq!(group.id, "sometestid");
        assert_eq!(group.name, "test-web-us-east");
        assert_eq!(group.description, "Test web servers in US-EAST region");
        assert_eq!(group.created_by, "jdoe@example.com");
        assert_eq!(group.created_timestamp, "2023-04-20T17:49:24.601496284Z");
        assert_eq!(group.modified_by, "admin@example.com");
        assert_eq!(group.modified_timestamp, "2024-01-15T14:23:18.412937102Z");
        assert_eq!(
            group.assignment_rule,
            Some("some-assignment-rule".to_string())
        );
        assert_eq!(group.group_type, Some(GroupType::Dynamic));

        assert_eq!(response.meta.trace_id, "12345".to_string());
    }

    #[tokio::test]
    async fn test_query_combined_host_groups_failure() {
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
            "/devices/combined/host-groups/v1",
            403,
            error_response,
        )
        .await;

        let configuration = create_test_config(&mock_server);

        let result =
            host_group_api::query_combined_host_groups(&configuration, None, None, None, None)
                .await;

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
