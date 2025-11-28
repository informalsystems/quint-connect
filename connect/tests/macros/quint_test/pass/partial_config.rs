use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_test(spec = "spec.qnt", test = "test1", main = "main", max_samples = 10)]
fn test1() -> impl Driver {
    TestDriver
}

#[quint_test(spec = "spec.qnt", test = "test2", main = "main", seed = "0xabc")]
fn test2() -> impl Driver {
    TestDriver
}

fn main() {}
