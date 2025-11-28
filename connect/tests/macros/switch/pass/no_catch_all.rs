use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn init(&mut self) {}
    fn handle(&mut self, _arg: u8) {}
    fn handle2(&mut self) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init,
            action1(arg) => self.handle(arg),
            action2 => self.handle2()
        })
    }
}

fn main() {}
