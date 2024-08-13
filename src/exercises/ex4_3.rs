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
*/

use std::{collections::HashMap, os::linux::raw::stat};

use crate::bases::mdp::{Action, Agent, Enviorment, Policy, State};

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct GamblerState {
    capital: u8,
}

#[derive(PartialEq, Eq, Clone, Hash)]
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
    ) -> HashMap<(GamblerState, f32), f32> {
        // next_capital = capital + result; result {+stake, -stake}

        let distribution: HashMap<(GamblerState, f32), f32> = HashMap::new();
        for next_state in self.get_states() {
            if next_state.capital == state.capital - action.stake {
                // 1 - prob_win
            } else if next_state.capital == state.capital + action.stake {
                // prob_win
            } else {
                //zero prob
            }
        }
        todo!()
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

impl State for GamblerState {}
impl Action for GamblerAction {}

pub fn solution() {
    let mapping: HashMap<GamblerState, GamblerAction> = HashMap::new();
    let mut gambler = Agent::<GamblerState, GamblerAction> {
        policy: Policy::Deterministic(mapping),
    };
}
