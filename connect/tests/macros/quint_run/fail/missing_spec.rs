use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {}

#[quint_run]
fn test_simple_run() -> impl Driver {
    TestDriver
}

fn main() {}
