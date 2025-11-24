pub struct NondetPicks;

impl NondetPicks {
    pub fn get(&self, _name: &str) -> Option<NondetPick> {
        todo!()
    }
}

pub struct NondetPick;

impl NondetPick {
    // FIXME: refine this interface
    pub fn try_into<T>(self) -> Result<T, ()> {
        todo!()
    }
}
