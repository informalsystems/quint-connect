use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_test(spec = "spec.qnt", unknown_attr = "value")]
fn test1() -> impl Driver {
    TestDriver
}

fn main() {}
