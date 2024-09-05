use core::panic;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::utils::stats::sample_from_hashmap_dist;

pub struct Agent<'a, S, A> {
    pub policy: Policy<'a, S, A>,
}
#[derive(Clone)]
pub enum Policy<'a, S, A> {
    Deterministic(HashMap<S, A>),
    Stochastic(HashMap<&'a S, HashMap<&'a A, f32>>),
}

pub trait State: PartialEq + Eq + Hash + Clone + Debug {}

pub trait Action: PartialEq + Eq + Hash + Clone + Debug {}

pub trait EnviormentModel<S, A>
where
    S: State,
    A: Action,
{
    fn dynamics(&self, state: &S, action: &A) -> HashMap<(S, i32), f32>;
    fn posible_actions(&self, state: &S) -> Vec<A>;
    fn get_states(&self) -> Vec<S>;
    fn response(&self, state: &S, action: &A) -> (S, i32);
    fn is_terminal(&self, state: &S) -> bool;
}

pub trait Enviorment<'a, S, A>
where
    S: State,
    A: Action,
{
    fn response(&self, state: &S, action: &A) -> (S, i32);
    fn is_terminal(&self, state: &S) -> bool;
    fn posible_actions(&self, state: &S) -> Vec<A>;
    fn get_states(&self) -> Vec<S>;
    fn episode(&self, init_state: &S, pol: &Policy<'a, S, A>) -> Vec<(S, A, i32)> {
        match pol {
            Policy::Deterministic(policy) => {
                let mut state = init_state.clone();
                let mut trajectory = Vec::new();
                loop {
                    if self.is_terminal(&state) {
                        break;
                    }
                    let action = &policy[&state];
                    let (next_state, reward) = self.response(&state, &action);
                    trajectory.push((state.clone(), action.clone(), reward));
                    state = next_state;
                }
                return trajectory;
            }
            Policy::Stochastic(policy) => {
                let mut state = init_state.clone();
                let mut trajectory = Vec::new();
                loop {
                    if self.is_terminal(&state) {
                        break;
                    }
                    let choice_distribution = &policy[&state];
                    let action = sample_from_hashmap_dist(choice_distribution);
                    //println!("{:?} , {:?}", state, action);
                    let (next_state, reward) = self.response(&state, &action);
                    trajectory.push((state.clone(), action.clone(), reward));
                    state = next_state;
                }
                return trajectory;
            }
        }
    }
}
