use std::collections::HashMap;

use bases::{
    mdp::{Enviorment, Policy},
    monte_carlo_control::first_visit_monte_carlo_control,
};
use exercises::{
    ex4_3::solution,
    ex5_10::{get_race_track, solution5_10, CarAction, CarState},
};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[allow(unused)]
mod bases;
#[allow(unused)]
mod exercises;
#[allow(unused)]
mod utils;
#[allow(unused)]
fn main() {
    /*
    ex 4.3
    let values = solution();
    println!("{values:?}")
    */

    solution5_10();
}
