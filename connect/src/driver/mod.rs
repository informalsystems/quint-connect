mod status;
mod step;

pub mod nondet;

pub use status::Status;
pub use step::Step;

pub trait Driver {
    fn step(&mut self, step: &Step) -> Status;
}
