use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, _step: &Step) -> Result {
        todo!()
    }
}

#[quint_run(spec = "spec.qnt", main = "main", max_samples = 10)]
fn test1() -> impl Driver {
    TestDriver
}

#[quint_run(spec = "spec.qnt", init = "init", step = "step", seed = "0xabc")]
fn test2() -> impl Driver {
    TestDriver
}

fn main() {}
