pub type Path = &'static [&'static str];

#[derive(Default)]
pub struct SpecAnnotations {
    pub state_location: Path,
    pub nondet_location: Path,
}
