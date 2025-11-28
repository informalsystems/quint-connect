mod state;
mod step;

pub mod nondet;

pub use state::State;
pub use step::Step;

pub type Result<A = ()> = anyhow::Result<A>;
pub type Path = &'static [&'static str];

#[derive(Default)]
pub struct SpecAnnotations {
    pub state_location: Path,
    pub nondet_location: Path,
}

pub trait Driver: Sized {
    type State: State<Self>;

    fn step(&mut self, step: &Step) -> Result;

    fn annotations() -> SpecAnnotations {
        SpecAnnotations::default()
    }
}
