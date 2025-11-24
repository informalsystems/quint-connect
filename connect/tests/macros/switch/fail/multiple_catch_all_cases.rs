use quint_connect::*;

struct TestDriver;

impl Driver for TestDriver {
    fn step(&mut self, step: &Step) -> Status {
        switch!(step {
            _ => todo!(),
            _ => todo!(),
        })
    }
}

fn main() {}
