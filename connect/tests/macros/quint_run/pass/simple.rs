use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    fn step(&mut self, _step: &Step) -> Status {
        todo!()
    }
}

#[quint_run(spec = "spec.qnt")]
fn test1() -> impl Driver {
    TestDriver
}

#[quint_run(
    spec = "spec.qnt",
    main = "main",
    init = "init",
    step = "step",
    max_samples = 32,
    max_steps = 64,
    seed = "0x42"
)]
fn test2() -> impl Driver {
    TestDriver
}

fn main() {}
