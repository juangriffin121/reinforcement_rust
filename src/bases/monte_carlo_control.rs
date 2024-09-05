use core::f32;
use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::mdp::{Action, Enviorment, Policy, State};
use plotters::prelude::*;

pub fn first_visit_monte_carlo_control<'a, E, S, A>(
    init_pol: Policy<'a, S, A>,
    init_vals: HashMap<(S, A), f32>,
    init_states: Vec<S>,
    episodes: u32,
    env: E,
    epsilon: f32,
    gamma: f32,
) where
    S: State,
    A: Action,
    E: Enviorment<'a, S, A>,
{
    let min_epsilon = 0.0001;
    let epsilon_decay: f32 = 0.999; // Example decay factor
    let mut current_epsilon = epsilon;
    let mut loss_curve = Vec::new();
    let mut rng = thread_rng();
    let mut pol = init_pol.clone();
    match init_pol {
        Policy::Stochastic(mut map) => {
            let mut returns = HashMap::new();
            let mut vals = init_vals.clone();
            for i in 0..episodes {
                current_epsilon = (current_epsilon * epsilon_decay.powi(i as i32)).max(min_epsilon); // Annealing epsilon
                println!("episode: {:?}", i);
                let init_state = init_states.choose(&mut rng).unwrap();
                let trajectory = env.episode(&init_state, &pol);
                let mut visited = HashSet::new();
                let mut g = 0.0;
                for (state, action, reward) in trajectory.iter().rev() {
                    g = gamma * g + *reward as f32;
                    let pair = (state.clone(), action.clone());
                    if !visited.contains(&pair) {
                        returns.entry(pair.clone()).or_insert(Vec::new()).push(g);
                        let sum: f32 = returns[&pair].iter().sum();
                        let n = returns[&pair].len();
                        visited.insert((state.clone(), action.clone()));
                        let mean = sum / n as f32;
                        *vals.get_mut(&(state.clone(), action.clone())).unwrap() = mean;
                        update_policy(&env, &mut map, &vals, state, current_epsilon)
                    }
                }
                println!("\tloss: {:?}", g);
                pol = Policy::Stochastic(map.clone());
                loss_curve.push(g as f64);
                println!("");
                /*
                for (k, val) in vals.iter() {
                    if *val != -200.0 {
                        println!("{:?}: {:?}", k, val)
                    }
                }
                println!("")
                */
            }
            for state in init_states {
                for action in env.posible_actions(&state) {
                    let v = vals.get(&(state.clone(), action.clone()));
                    println!("({:?} , {:?}): {:?}", state, action, v)
                }
            }
        }
        Policy::Deterministic(map) => {
            unimplemented!() // montecarlo is only used with soft policies
        }
    }
    let root_area = BitMapBackend::new("graph.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Line Graph", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..loss_curve.len(), -500.0..0.0) // Adjust range based on your data
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Plot the data as a line
    chart
        .draw_series(LineSeries::new(
            (0..).zip(loss_curve.iter()).map(|(x, &y)| (x, y)),
            &BLUE,
        ))
        .unwrap();

    // Save the result
    root_area.present().unwrap();
    println!("Graph saved to graph.png");
}

pub fn update_policy<'a, E, S, A>(
    env: &E,
    map: &mut HashMap<&'a S, HashMap<&'a A, f32>>,
    action_values: &HashMap<(S, A), f32>,
    state: &S,
    epsilon: f32,
) where
    S: State,
    A: Action,
    E: Enviorment<'a, S, A>,
{
    /*
    let old_max_action = (*map[state]
        .iter()
        .max_by(|&x, &y| x.1.partial_cmp(y.1).unwrap())
        .unwrap()
        .0)
        .clone();
    */
    let actions = env.posible_actions(state);
    let max_action: A = actions
        .iter()
        .max_by(|&x, &y| {
            action_values[&(state.clone(), x.clone())]
                .partial_cmp(&action_values[&(state.clone(), y.clone())])
                .unwrap()
        })
        .unwrap()
        .clone();
    /*
    if old_max_action != max_action {
        println!("{:?}", state);
        println!("{:?}", old_max_action);
        println!("{:?}", max_action);
        println!("");
    }
    */

    for action in &actions {
        let val = if *action == max_action {
            1.0 - epsilon + epsilon / actions.len() as f32
        } else {
            epsilon / actions.len() as f32
        };
        *map.get_mut(state)
            .unwrap()
            .get_mut(&action.clone())
            .unwrap() = val;
    }
}

pub fn rand_init() {}
