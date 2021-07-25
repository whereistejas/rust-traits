use crate::traits::{Flies, Organism, Swims};

#[derive(Clone)]
pub struct Fish {
    pub name: String,
    pub limbs: i32,
    pub swim: bool,
    pub fly: bool,
}

impl Organism for Fish {
    fn is_living(&self) -> bool {
        true
    }
}

impl Swims for Fish {
    fn can_swim(&self) -> bool {
        true
    }

    fn how_deep(&self) -> i32 {
        100
    }
}

impl Flies for Fish {
    fn can_fly(&self) -> bool {
        true
    }
    fn how_high(&self) -> i32 {
        0
    }
}
