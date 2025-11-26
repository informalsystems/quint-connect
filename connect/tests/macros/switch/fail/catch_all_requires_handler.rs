use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    type State = ();

    fn step(&mut self, step: &Step) -> Status {
        switch!(step { _ })
    }
}

fn main() {}
