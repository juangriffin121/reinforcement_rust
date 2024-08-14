use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct Agent<S, A> {
    pub policy: Policy<S, A>,
}

pub enum Policy<S, A> {
    Deterministic(HashMap<S, A>),
    Stochastic(fn(&S) -> HashMap<&A, f32>),
}

pub trait State: PartialEq + Eq + Hash + Clone + Debug {
    fn is_terminal(&self) -> bool;
}

pub trait Action: PartialEq + Eq + Hash + Clone + Debug {}

pub trait Enviorment<S, A>
where
    S: State,
    A: Action,
{
    fn dynamics(&self, state: &S, action: &A) -> HashMap<(S, i32), f32>;
    fn posible_actions(&self, state: &S) -> Vec<A>;
    fn get_states(&self) -> Vec<S>;
}
