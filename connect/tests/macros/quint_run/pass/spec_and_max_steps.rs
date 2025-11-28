use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_run(spec = "spec.qnt", max_steps = 50)]
fn test1() -> impl Driver {
    TestDriver
}

fn main() {}
