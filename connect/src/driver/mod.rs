mod nondet;
mod status;
mod step;

pub use nondet::{NondetPick, NondetPicks};
pub use status::Status;
pub use step::Step;

pub trait Driver {
    fn step(&mut self, step: &Step) -> Status;
}
