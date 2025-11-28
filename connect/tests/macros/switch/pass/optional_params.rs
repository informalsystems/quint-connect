use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn handle(&mut self, _arg: Option<u8>) {}
    fn handle2(&mut self, _a: Option<u8>, _b: Option<u8>) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            action1(arg?) => self.handle(arg),
            action2(a?, b?) => self.handle2(a, b)
        })
    }
}

fn main() {}
