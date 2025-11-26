mod state;
mod step;

pub mod nondet;
pub use state::State;
pub use step::Step;

pub type Result = anyhow::Result<()>;

pub trait Driver: Sized {
    type State: State<Self>;

    fn step(&mut self, step: &Step) -> Result;
}
