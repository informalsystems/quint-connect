use crate::driver::nondet::NondetPicks;

pub struct Step {
    pub action_taken: String,
    pub nondet_picks: NondetPicks,
}
