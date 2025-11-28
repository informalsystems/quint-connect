use quint_connect::*;

struct TestDriver;

impl TestDriver {
    fn init(&mut self) {}
    fn handle(&mut self, _arg: u8) {}
    fn handle2(&mut self, _a: Option<u8>) {}
    fn handle3(&mut self, _a: Option<u8>, _b: u8) {}
    fn complex_handler(&mut self) {}
}

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            init,                                          // implicit
            action1(arg) => self.handle(arg),             // required param
            action2(a: u8?) => self.handle2(a),           // optional with type
            action3(a?, b) => self.handle3(a, b),         // mixed optional/required
            action4 => { self.complex_handler() },        // explicit with block
            _ => todo!()                                   // catch-all
        })
    }
}

fn main() {}
