use std::collections::HashMap;

use axum_test::{TestServer, TestServerConfig};
use hurl::runner;
use hurl::runner::{RunnerOptionsBuilder, Value};
use hurl::util::logger::LoggerOptionsBuilder;

use axum_hurl_testing::make_app;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_basic() {
    let app: axum::Router = make_app();

    // Run the test server on a random port
    let config = TestServerConfig::builder().http_transport().build();
    let server = TestServer::new_with_config(app, config).unwrap();
    let addr = server.server_address().unwrap();
    let baseurl = addr.as_str().strip_suffix('/').unwrap();

    // Read hurl file
    let content = std::fs::read_to_string("./tests/api/basic.hurl").unwrap();

    // Set the baseurl variable
    let mut variables = HashMap::default();
    variables.insert("baseurl".to_string(), Value::String(baseurl.to_string()));

    // Run it
    let runner_opts = RunnerOptionsBuilder::new().follow_location(true).build();
    let logger_opts = LoggerOptionsBuilder::new().build();
    let result = runner::run(&content, &runner_opts, &variables, &logger_opts).unwrap();

    assert!(result.success);
}
