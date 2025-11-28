use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_test(spec = "spec.qnt", test = "test1Test")]
fn test1() -> impl Driver {
    TestDriver
}

#[quint_test(
    spec = "spec.qnt",
    main = "main",
    test = "test2Test",
    max_samples = 32,
    seed = "0x42"
)]
fn test2() -> impl Driver {
    TestDriver
}

fn main() {}
