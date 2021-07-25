use crate::traits::{Flies, Organism, Swims};

#[derive(Clone)]
pub struct Birds {
    pub name: String,
    pub limbs: i32,
    pub swim: bool,
    pub fly: bool,
}

impl Organism for Birds {
    fn is_living(&self) -> bool {
        true
    }
}

impl Flies for Birds {
    fn can_fly(&self) -> bool {
        true
    }
    fn how_high(&self) -> i32 {
        100
    }
}

impl Swims for Birds {
    fn can_swim(&self) -> bool {
        true
    }

    fn how_deep(&self) -> i32 {
        0
    }
}
