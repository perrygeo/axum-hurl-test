# Testing Axum applications with Hurl

[Hurl](https://hurl.dev/) is a fanstastic tool for testing HTTP services.
You write the flow of requests and responses in a plain text format,
and the `hurl` command runs them, and tests them, against the live server.
This gives you a high-level domain-specific test language to describe
exactly how your API should work, acting like an **integration test at the HTTP boundary**.

For example, to test the JSON response for a `status`:

```hurl
GET https://example.org/api/tests/4567
HTTP 200
[Asserts]
jsonpath "$.status" == "RUNNING"
```

Hurl scripts can become living, testable documentation of
the public behavior of your API. They work equally well as day-to-day
integration tests and as acceptance critera for product design.

As a CLI that speaks HTTP, Hurl works with any language.
But writing backend software often requires tooling with
deeper integration to the langauge ecosystem so this will come in handy.
For instance, in a Rust application, I'd like to be able to

- run `cargo test` and have the application server spun up automatically.
- run `cargo fuzz` to test the domain of input variables at the API level.
- run `cargo llvm-cov` to evaluate code coverage of the API tests.
- run `cargo flamegraph` to profile for hot spots.
- run `cargo bench` to gather timings and compare with `criterion`.
- run `cargo run --bin my_hurl_runner http://staging.example.com` to send traffic to an arbitrary host.

The Hurl tests can express the HTTP logic for all cases. But do we need rewrite that logic for each
scenario? Thankfully, Hurl is also implemented in Rust and publishes a library so we can use
it in our test suite. So we aim to maximally leverage the Hurl tests - beyond just ad-hoc CLI testing, they
can be used for integration tests, fuzz testing, profiling, coverage, benchmarking, and live qa testing.

> Write Hurl tests once. Run Hurl tests everywhere.

This repository is a demonstration of that principle. Specially, this repo contains an Axum web server,
a set of hurl tests, and the rust "scaffolding" to run the Hurl tests in the scenarios above.

I'm not proposing them as your _only_ form of testing, but Hurl API tests and their variations herein
do provide a lot of "bang for the buck". They test at the public API boundary
which is ultimately the contract that matters.

## Step 1 Basic tests
