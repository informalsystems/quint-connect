mod nondet;
mod state;
mod step;

pub use state::State;
pub use step::Step;

pub type Result<A = ()> = anyhow::Result<A>;
pub type Path = &'static [&'static str];

#[derive(Default)]
pub struct Config {
    pub state_path: Path,
    pub nondet_path: Path,
}

pub trait Driver: Sized {
    type State: State<Self>;

    fn step(&mut self, step: &Step) -> Result;

    fn config() -> Config {
        Config::default()
    }
}
