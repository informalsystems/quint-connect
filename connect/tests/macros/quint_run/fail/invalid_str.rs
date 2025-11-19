use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {}

#[quint_run(spec = "spec.qnt", main = 666)]
fn test1() -> impl Driver {
    TestDriver
}

fn main() {}
