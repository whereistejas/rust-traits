use crate::traits::{Organism, Runs, Swims};

#[derive(Clone)]
pub struct Mammals {
    pub name: String,
    pub limbs: i32,
    pub swim: bool,
    pub run: bool,
}

impl Organism for Mammals {
    fn is_living(&self) -> bool {
        true
    }
}

impl Swims for Mammals {
    fn can_swim(&self) -> bool {
        true
    }

    fn how_deep(&self) -> i32 {
        0
    }
}

impl Runs for Mammals {
    fn can_run(&self) -> bool {
        true
    }

    fn how_fast(&self) -> i32 {
        100
    }
}
