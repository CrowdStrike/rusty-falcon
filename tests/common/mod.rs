use rusty_falcon::apis::configuration;
use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    http::Method,
    matchers::{method, path},
};

/// Helper function to create mock endpoints.
///
/// Creates a new MockServer, mounts the specified mock endpoint, and returns the server.
///
/// # Arguments
/// * `http_method` - The HTTP method (GET, POST, etc.)
/// * `endpoint_path` - The API endpoint path
/// * `status_code` - The HTTP status code to return
/// * `response_body` - The JSON response body
///
/// # Returns
/// A configured MockServer instance
pub async fn setup_mock(
    http_method: Method,
    endpoint_path: &str,
    status_code: u16,
    response_body: Value,
) -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(method(http_method.as_str()))
        .and(path(endpoint_path))
        .respond_with(ResponseTemplate::new(status_code).set_body_json(response_body))
        .mount(&mock_server)
        .await;
    mock_server
}

/// Helper function to create a test configuration pointing to a mock server.
///
/// # Arguments
/// * `mock_server` - Reference to the MockServer
///
/// # Returns
/// A Configuration instance with base_path set to the mock server URI
pub fn create_test_config(mock_server: &MockServer) -> configuration::Configuration {
    let mut config = configuration::Configuration::default();
    config.base_path = mock_server.uri();
    config
}
