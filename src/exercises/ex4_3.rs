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

use crate::bases::{
    mdp::{Action, Agent, Enviorment, Policy, State},
    policy_iteration::{greedy_policy, policy_iteration, value_iteration},
};
use plotters::prelude::*;
use rand::{thread_rng, Rng};

use rand::seq::SliceRandom;
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GamblerState {
    capital: u8,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GamblerAction {
    stake: u8,
}

pub struct Casino {
    probability_of_win: f32, //ph
}

impl Enviorment<GamblerState, GamblerAction> for Casino {
    fn dynamics(
        &self,
        state: &GamblerState,
        action: &GamblerAction,
    ) -> HashMap<(GamblerState, i32), f32> {
        let mut distribution: HashMap<(GamblerState, i32), f32> = HashMap::new();
        // since i loop over the dict to do the sum and multiply by prob and multiply by zero and
        // sum is like not doing anything theres no need to add the 0 prob to the hashmap
        for next_state in self.get_states() {
            if state.is_terminal() {
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
        let possible_capitals = 0..101;
        let mut states: Vec<GamblerState> = Vec::new();
        for capital in possible_capitals {
            states.push(GamblerState { capital })
        }
        states
    }
}

impl State for GamblerState {
    fn is_terminal(&self) -> bool {
        self.capital == 0 || self.capital == 100
    }
}
impl Action for GamblerAction {}

fn plot_graph(
    values: HashMap<GamblerState, f32>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Extract and sort the data from the HashMap
    let mut data: Vec<(u8, f32)> = values.into_iter().map(|(k, v)| (k.capital, v)).collect();
    data.sort_by_key(|&(attr, _)| attr);

    // Create a chart
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Values", ("sans-serif", 50).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .build_cartesian_2d(
            data.first().unwrap().0 as f64..data.last().unwrap().0 as f64,
            0f64..data.iter().map(|(_, v)| *v as f64).fold(f64::NAN, f64::max),
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.into_iter().map(|(x, y)| (x as f64, y as f64)),
            &RED,
        ))?
        .label("Curve")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn plot_graph_act(
    policy: HashMap<GamblerState, GamblerAction>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Extract and sort the data from the HashMap
    let mut data: Vec<(u8, u8)> = policy
        .into_iter()
        .map(|(k, v)| (k.capital, v.stake))
        .collect();
    data.sort_by_key(|&(attr, _)| attr);

    // Create a chart
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Policy", ("sans-serif", 50).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .build_cartesian_2d(
            data.first().unwrap().0 as f64..data.last().unwrap().0 as f64,
            0f64..data.iter().map(|(_, v)| *v as f64).fold(f64::NAN, f64::max),
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.into_iter().map(|(x, y)| (x as f64, y as f64)),
            &RED,
        ))?
        .label("Curve")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn solution() -> HashMap<GamblerState, f32> {
    let mut rng = thread_rng();

    let casino = Casino {
        probability_of_win: 0.4,
    };
    let states = casino.get_states();

    let mut mapping: HashMap<GamblerState, GamblerAction> = HashMap::new();
    for state in &states {
        let actions = casino.posible_actions(&state);
        if let Some(action) = actions.choose(&mut rng) {
            mapping.insert(state.clone(), action.clone());
        } else {
            println!("The vector is empty!");
        }
    }

    let mut gambler = Agent::<GamblerState, GamblerAction> {
        policy: Policy::Deterministic(mapping),
    };
    let mut values: HashMap<GamblerState, f32> = HashMap::new();
    for state in &states {
        if state.is_terminal() {
            values.insert(state.clone(), 0.0);
        } else {
            values.insert(state.clone(), rng.gen());
        }
    }
    let gamma = 1.0;
    let tolerance = 0.01;
    let values = value_iteration(&casino, &states, Some(values), gamma, tolerance);
    plot_graph(values.clone(), "graph.png").unwrap();
    let policy = greedy_policy(&casino, &states, &values, gamma);
    plot_graph_act(policy, "act_graph.png").unwrap();

    println!("done with val iter");

    let values = policy_iteration(&mut gambler, &casino, None, gamma, tolerance);
    plot_graph(values.clone(), "graph,_PI.png").unwrap();
    let policy = greedy_policy(&casino, &states, &values, gamma);
    plot_graph_act(policy, "act_graph_PI.png").unwrap();
    values
}
