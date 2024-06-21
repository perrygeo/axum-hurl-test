# Testing Axum applications with Hurl

[Hurl](https://hurl.dev/) is a CLI tool for testing HTTP services.

[Axum](https://docs.rs/axum/latest/axum/) is an asynchronous Rust web server.

Together, they are more than the sum of their parts.

## Hurl in a nutshell

You write the flow of HTTP requests and responses in a plain text format.
The `hurl` command runs and tests them against a live server. 
This gives you a domain-specific language to describe
exactly how your API should work, an **integration test at the HTTP boundary**.

For example, to test that you're getting a `text/html` response with a 200 status,
create a file `hello.hurl`

```hurl
GET https://example.org/
HTTP 200
[Asserts]
header "Content-Type" contains "text/html"
```

then run the tests...

```bash
$ hurl --test hello.hurl
hello.hurl: Running [1/1]
hello.hurl: Success (1 request(s) in 25 ms)
--------------------------------------------------------------------------------
Executed files:  1
Succeeded files: 1 (100.0%)
Failed files:    0 (0.0%)
Duration:        26 ms
```

As a CLI, Hurl works with _any_ language that can stand up an HTTP server.
But hurl is implemented in Rust, and this write-up is a brief demonstration of 
the additional benefits Rust developers can gain from adopting hurl.


## API tests

> Write Hurl tests once. Run Hurl tests everywhere.

Hurl tests work at the HTTP level and are decoupled from the 
messy technology churn of programming languages and frameworks.
Web services often live longer than the trendy technology that created them.
When the tests that define your core business logic get tied to those systems, 
you're trapped! You've got no way to move to a different solution
without breaking things, thus confidence wavers.

By using hurl to capture the _actual_
behavior of your API, you're free to change implementation details.
In other words, hurl helps you rewrite it in Rust :-)
Or in any other language/technique/hosting service for that matter.

But language is of particular importance here, because Hurl is implemented in Rust.
By integrating at the API level, using the hurl crates to get at timing information,
variable injection, etc. we have a solid integration testing toolkit.

The aim is to maximally leverage the Hurl tests, reusing them
beyond just ad-hoc CLI testing. They can be used for integration tests, fuzz testing,
profiling, coverage, benchmarking, and live QA testing.

## Examples

Spin up the test server then hurl traffic at it.

```shell
$ cargo test --test api
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running tests/api.rs (target/debug/deps/api-a7d87431c0d4249a)

running 1 test
test test_api ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Evaluate code coverage of the hurl tests.

```
$ cargo llvm-cov --ignore-filename-regex bin --test api
   Compiling axum-hurl-testing v0.1.0 (/home/user/projects/axum-hurl-test)
    Finished test [unoptimized + debuginfo] target(s) in 4.24s
     Running tests/api.rs (target/llvm-cov-target/debug/deps/api-a7d87431c0d4249a)

running 1 test
test test_api ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                                            Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/home/user/projects/axum-hurl-test/src/lib.rs             5                 2    60.00%           5                 2    60.00%          13                 4    69.23%           0                 0         -
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                     5                 2    60.00%           5                 2    60.00%          13                 4    69.23%           0                 0         -
```

To send the same traffic to an arbitrary host.

```
$ cargo run --bin hurl-traffic http://localhost:3000
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/hurl-traffic 'http://localhost:3000'`
----
Running Hurl tests './tests/api.hurl' against 'http://localhost:3000'
time: 13 ms
n requests: 2
n asserts: 6
success: true
```

This repo contains an Axum web server, a hurl test file, and the
Rust scaffolding to run the Hurl tests in these scenarios.

I'm not proposing them as your _only_
form of testing. But hurl-based API tests (traffic tests, integration tests, whatever you want to label them)
provide a lot of bang for the buck, in the Rust context particularly.
