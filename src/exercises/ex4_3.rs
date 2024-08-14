/*
Example 4.3: Gambler’s Problem A gambler has the opportunity to make bets on the outcomes
of a sequence of coin flips. If the coin comes up heads, he wins as many dollars as he has staked on
that flip; if it is tails, he loses his stake. The game ends when the gambler wins by reaching his goal
of $100, or loses by running out of money. On each flip, the gambler must decide what portion of his
capital to stake, in integer numbers of dollars. This problem can be formulated as an undiscounted,
episodic, finite MDP. The state is the gambler’s capital, s ∈ {1, 2, . . . , 99} and the actions are stakes,
a ∈ {0, 1, . . . , min(s, 100 − s)}. The reward is zero on all transitions except those on which the gambler
reaches his goal, when it is +1. The state-value function then gives the probability of winning from
each state. A policy is a mapping from levels of capital to stakes. The optimal policy maximizes
the probability of reaching the goal. Let ph denote the probability of the coin coming up heads. If
ph is known, then the entire problem is known and it can be solved, for instance, by value iteration.
Figure 4.3 shows the change in the value function over successive sweeps of value iteration, and the final
policy found, for the case of ph = 0.4. This policy is optimal, but not unique. In fact, there is a whole
family of optimal policies, all corresponding to ties for the argmax action selection with respect to the
optimal value function. Can you guess what the entire family looks like?

Exercise 4.9 (programming) Implement value iteration for the gambler’s problem and solve it for
ph = 0.25 and ph = 0.55. In programming, you may find it convenient to introduce two dummy states
corresponding to termination with capital of 0 and 100, giving them values of 0 and 1 respectively.
Show your results graphically, as in Figure 4.3. Are your results stable as θ → 0?
*/

use std::collections::HashMap;

use rand::{thread_rng, Rng};

use crate::bases::{
    mdp::{Action, Agent, Enviorment, Policy, State},
    policy_iteration::value_iteration,
};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GamblerState {
    capital: u8,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GamblerAction {
    stake: u8,
}

pub struct Casino {
    probability_of_win: f32,
}

impl Enviorment<GamblerState, GamblerAction> for Casino {
    fn dynamics(
        &self,
        state: &GamblerState,
        action: &GamblerAction,
    ) -> HashMap<(GamblerState, i32), f32> {
        let mut distribution: HashMap<(GamblerState, i32), f32> = HashMap::new();
        // since i loop over the dict to do the sum and multiply by prob and multiply by zero and
        // sum is like not doing anything
        for next_state in self.get_states() {
            if state.capital == 100 {
                continue;
            }
            if next_state.capital == state.capital - action.stake {
                distribution.insert((next_state, 0), 1.0 - self.probability_of_win);
            } else if next_state.capital == state.capital + action.stake {
                if state.capital + action.stake == 100 {
                    distribution.insert((next_state, 1), self.probability_of_win);
                } else {
                    distribution.insert((next_state, 0), self.probability_of_win);
                }
            } else {
                //No prob, no need to write
            }
        }
        if state.capital + action.stake > 100 {
            distribution.insert((GamblerState { capital: 100 }, 1), self.probability_of_win);
        } else {
        }
        distribution
    }
    fn posible_actions(&self, state: &GamblerState) -> Vec<GamblerAction> {
        let possible_stakes = 0..(state.capital.min(100 - state.capital) + 1);
        let mut posible_actions: Vec<GamblerAction> = Vec::new();
        for stake in possible_stakes {
            posible_actions.push(GamblerAction { stake })
        }
        posible_actions
    }
    fn get_states(&self) -> Vec<GamblerState> {
        let possible_capitals = 0..100;
        let mut states: Vec<GamblerState> = Vec::new();
        for capital in possible_capitals {
            states.push(GamblerState { capital })
        }
        states
    }
}

impl State for GamblerState {
    fn is_terminal(&self) -> bool {
        self.capital == 100
    }
}
impl Action for GamblerAction {}

pub fn solution() -> HashMap<GamblerState, f32> {
    let mut rng = thread_rng();
    let mapping: HashMap<GamblerState, GamblerAction> = HashMap::new();
    let mut _gambler = Agent::<GamblerState, GamblerAction> {
        policy: Policy::Deterministic(mapping),
    };
    let casino = Casino {
        probability_of_win: 0.4,
    };
    let states = casino.get_states();
    let mut values: HashMap<GamblerState, f32> = HashMap::new();
    for state in &states {
        if state.is_terminal() {
            values.insert(state.clone(), 1.0);
        } else {
            values.insert(state.clone(), rng.gen());
        }
    }
    value_iteration(&casino, &states, values, 1.0, 0.1)
}
