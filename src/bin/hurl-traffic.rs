///
/// The hurl-traffic command line tool
/// is a cheap immitation of the real hurl CLI.
///
/// It allows you full control of injecting variables
/// so you can e.g. generate data for benchmarking
/// or fuzz testing, without leaving Rust or giving
/// up existing hurl tests.
///
/// Here we just inject the `baseurl` and run some
/// simple tests.
///
use std::collections::HashMap;

use hurl::runner;
use hurl::runner::{RunnerOptionsBuilder, Value};
use hurl::util::logger::LoggerOptionsBuilder;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Read input args
    let baseurl = std::env::args().nth(1).expect(
        "ERROR: First argument should be a valid base URL, all requests are made relative to this URL.",
    );

    let hurlfile = std::env::args()
        .nth(2)
        .unwrap_or("./tests/api.hurl".to_string());

    // Read hurl file
    let content = std::fs::read_to_string(&hurlfile)
        .expect("Second argument should be a valid path to a hurl file.");

    // Set the baseurl variable
    let variables = HashMap::from([("baseurl".to_string(), Value::String(baseurl.to_string()))]);

    // Run it
    println!(
        "----\nRunning Hurl tests '{}' against '{}'",
        &hurlfile, &baseurl
    );
    let runner_opts = RunnerOptionsBuilder::new().follow_location(true).build();
    let logger_opts = LoggerOptionsBuilder::new().build();
    let result = runner::run(&content, &runner_opts, &variables, &logger_opts)?;

    // Report
    println!("time: {} ms", &result.time_in_ms);
    let mut total_assertions = 0;
    for entry in &result.entries {
        total_assertions += &entry.asserts.len();
    }
    println!("n requests: {}", &result.entries.len());
    println!("n asserts: {}", total_assertions);
    println!("success: {}", &result.success);

    if result.success {
        Ok(())
    } else {
        // dump the assertions for review
        // TODO make this more readable
        for entry in &result.entries {
            println!("assertions: {}", &entry.asserts.len());
            for a in &entry.asserts {
                dbg!(a);
            }
        }
        Err("Hurl failed".into())
    }
}
