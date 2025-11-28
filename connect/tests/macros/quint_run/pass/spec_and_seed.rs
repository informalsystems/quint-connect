use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_run(spec = "spec.qnt", seed = "0x123abc")]
fn test1() -> impl Driver {
    TestDriver
}

fn main() {}
