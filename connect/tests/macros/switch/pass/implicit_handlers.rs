use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn init(&mut self) {}
    fn action1(&mut self) {}
    fn action2(&mut self) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init,
            action1,
            action2
        })
    }
}

fn main() {}
