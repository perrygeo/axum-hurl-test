# Testing Axum applications with Hurl

[Hurl](https://hurl.dev/) is a fanstastic tool for testing HTTP services.
You specify the flow of requests and responses in a plain text format,
and the `hurl` command runs and tests them against the live server.
This gives you a domain-specific language to describe
exactly how your API should work, an **integration test at the HTTP boundary**.

For example, to assert that you're getting a JSON response with RUNNING status...

```hurl
GET https://example.org/api/tests/4567
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.status" == "RUNNING"
```

and run the tests...

```bash
$ hurl --test hello.hurl
hello.hurl: Running [1/2]
hello.hurl: Success (6 request(s) in 245 ms)
--------------------------------------------------------------------------------
Executed files:  1
Succeeded files: 1 (100.0%)
Failed files:    0 (0.0%)
Duration:        261 ms
```

## Goal

> Write Hurl tests once. Run Hurl tests everywhere.

As a CLI that speaks HTTP, Hurl works with any language. But writing
web services often requires better integration with the langauge ecosystem.

In a Rust application, I'd like to be able to:

- run `cargo test` and have the application server spun up automatically.
- run `cargo fuzz` to test the domain of input variables _at the API level_.
- run `cargo llvm-cov` to evaluate code coverage of the API tests.
- run `cargo flamegraph` to profile for hot spots.
- run `cargo bench` to gather timings and compare with `criterion`.
- run `cargo run --bin my_hurl_runner http://staging.example.com` to send traffic to an arbitrary host.

The Hurl tests can express the HTTP logic for all cases.
Thankfully, Hurl is also implemented in Rust and publishes a library so we can use it in our test suite.
So we can aim to maximally leverage the Hurl tests, reusing them
beyond just ad-hoc CLI testing. They can be used for integration tests, fuzz testing,
profiling, coverage, benchmarking, and live QA testing.

This repo contains an axum web server, a set of hurl tests, and the
Rust "scaffolding" to run the Hurl tests in these scenarios. In each case,
all the HTTP business logic is defered to the Hurl tests.

I'm not proposing them as your _only_ form of testing, but Hurl API tests integrated
with Rust test tooling does provide a lot of "bang for the buck".
It tests at the public API boundary which is ultimately the contract that matters if you're publishing
a web site.
