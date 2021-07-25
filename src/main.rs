#![allow(dead_code)]
mod birds;
mod fish;
mod mammals;
mod traits;

use birds::*;
use fish::*;
use mammals::*;
use traits::*;

fn is_an_organism<T: Organism + ?Sized>(organisms: Vec<Box<T>>) -> Vec<bool> {
    let mut result = Vec::new();
    for organism in organisms {
        result.push(organism.is_living());
    }
    result
}

#[derive(Clone)]
struct Undead;
impl Runs for Undead {
    fn can_run(&self) -> bool {
        true
    }

    fn how_fast(&self) -> i32 {
        10000
    }
}
impl Swims for Undead {
    fn can_swim(&self) -> bool {
        true
    }

    fn how_deep(&self) -> i32 {
        10000
    }
}
impl Flies for Undead {
    fn can_fly(&self) -> bool {
        true
    }

    fn how_high(&self) -> i32 {
        10000
    }
}

fn is_a_swimmer<T>(organisms: Vec<Box<T>>) -> Vec<bool>
where
    T: Swims + ?Sized,
{
    let mut result = Vec::new();
    for organism in organisms {
        result.push(organism.can_swim());
    }
    result
}

fn is_a_runner<T>(organisms: Vec<Box<T>>) -> Vec<bool>
where
    T: Runs + Organism + ?Sized,
{
    let mut result = Vec::new();
    for organism in organisms {
        result.push(organism.can_run());
    }
    result
}

fn main() {
    let mammal = Box::new(Mammals {
        name: String::from("swimming dog"),
        limbs: 4,
        swim: true,
        run: false,
    });
    let fish = Box::new(Fish {
        name: String::from("flying fish"),
        limbs: 0,
        swim: true,
        fly: true,
    });
    let bird = Box::new(Birds {
        name: String::from("swimming bird"),
        limbs: 0,
        swim: true,
        fly: true,
    });
    let undead = Box::new(Undead);
    let animals: Vec<Box<dyn Organism>> = vec![mammal.clone(), fish.clone(), bird.clone()];
    let living = is_an_organism(animals);
    println!("All animals are living: {:?}", living);

    let animals: Vec<Box<dyn Swims>> = vec![mammal.clone(), fish.clone(), bird.clone()];
    let living = is_a_swimmer(animals);
    println!("{:?}", living);

    let animals: Vec<Box<dyn Runs>> = vec![mammal, undead.clone()];
    let living = is_a_runner(animals);
    println!("{:?}", living);
}
