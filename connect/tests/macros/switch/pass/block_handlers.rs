use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn init(&mut self, _x: i32) {}
    fn handle_positive(&mut self, _arg: u8) {}
    fn handle_negative(&mut self, _arg: u8) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init => {
                let x = 42;
                self.init(x)
            },
            action1(arg) => {
                if arg > 0 {
                    self.handle_positive(arg)
                } else {
                    self.handle_negative(arg)
                }
            }
        })
    }
}

fn main() {}
