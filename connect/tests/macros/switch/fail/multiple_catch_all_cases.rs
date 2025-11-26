use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            _ => todo!(),
            _ => todo!(),
        })
    }
}

fn main() {}
