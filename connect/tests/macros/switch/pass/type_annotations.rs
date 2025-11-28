use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn handle(&mut self, _arg: u8) {}
    fn handle2(&mut self, _a: Option<String>, _b: u32) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            action1(arg: u8) => self.handle(arg),
            action2(a: String?, b: u32) => self.handle2(a, b)
        })
    }
}

fn main() {}
