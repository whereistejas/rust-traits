pub struct Living {
    pub breathes: bool,
    pub eats: bool,
}

impl Organism for Living {
    fn is_living(&self) -> bool {
        true
    }
}

pub trait Organism {
    fn is_living(&self) -> bool;
}

pub trait Move {
    fn can_move(&self) -> bool;
}

pub trait Swims {
    fn can_swim(&self) -> bool;
    fn how_deep(&self) -> i32;
}

pub trait Runs {
    fn can_run(&self) -> bool;
    fn how_fast(&self) -> i32;
}

pub trait Flies {
    fn can_fly(&self) -> bool;
    fn how_high(&self) -> i32;
}
