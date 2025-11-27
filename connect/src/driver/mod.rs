mod state;
mod step;

pub mod nondet;
pub use state::State;
pub use step::Step;

use crate::itf::value::Record;

pub type Result<A = ()> = anyhow::Result<A>;

pub trait Driver: Sized {
    type State: State<Self>;

    fn step(&mut self, step: &Step) -> Result;

    fn prepare(&mut self, state: Record) -> Result<Step> {
        Step::from_mbt_state(state)
    }
}
