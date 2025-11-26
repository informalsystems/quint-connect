use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn init(&mut self) {}
    fn action(&mut self, _arg: u8) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Status {
        switch!(step {
            init,
            action1(arg) => self.action(arg),
            action2(a: u8?, b: u8) => self.action(a.unwrap_or(b)),
            _ => todo!()
        })
    }
}

fn main() {}
