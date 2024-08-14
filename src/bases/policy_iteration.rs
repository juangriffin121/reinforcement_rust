use crate::bases::mdp;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub fn policy_evaluation<E, S, A>(
    agent: &mut mdp::Agent<S, A>,
    enviorment: &E,
    states: &Vec<S>,
    mut values: HashMap<S, f32>,
    gamma: f32,
    tolerance: f32,
) -> HashMap<S, f32>
where
    A: mdp::Action,
    S: mdp::State,
    E: mdp::Enviorment<S, A>,
{
    let mut delta: f32 = 0.0;

    loop {
        for state in states {
            let v = values[state];
            let value = match &agent.policy {
                mdp::Policy::Deterministic(policy) => {
                    let action = &policy[state];
                    let probs = enviorment.dynamics(state, &action);
                    let mut sum = 0.0;
                    for ((next_state, reward), prob) in probs.iter() {
                        sum += prob * (*reward as f32 + gamma * values[next_state])
                    }
                    sum
                }
                mdp::Policy::Stochastic(policy) => {
                    let action_dist = policy(state);
                    let mut sum = 0.0;
                    for action in enviorment.posible_actions(state) {
                        let acton_prob = action_dist[&action];
                        let probs = enviorment.dynamics(state, &action);
                        for ((next_state, reward), prob) in probs.iter() {
                            sum += acton_prob * prob * (*reward as f32 + gamma * values[next_state])
                        }
                    }
                    sum
                }
            };
            delta = delta.max((v - value).abs());
            let val = values.get_mut(state).unwrap();
            *val = value;
        }
        if delta < tolerance {
            break;
        }
    }
    values
}

pub fn policy_improvement<E, S, A>(
    agent: &mut mdp::Agent<S, A>,
    enviorment: E,
    states: &Vec<S>,
    gamma: f32,
    tolerance: f32,
    mut values: HashMap<S, f32>,
) -> HashMap<S, f32>
where
    A: mdp::Action,
    S: mdp::State,
    E: mdp::Enviorment<S, A>,
{
    let mut policy_stable = false;
    for state in states {
        match &mut agent.policy {
            mdp::Policy::Deterministic(policy) => {
                let old_action = policy[&state].clone();
                let mut max_action = old_action.clone();
                let mut max_action_value = -f32::INFINITY;

                for action in enviorment.posible_actions(&state) {
                    let probs = enviorment.dynamics(&state, &action);
                    let mut sum = 0.0;
                    for ((next_state, reward), prob) in probs.iter() {
                        sum += prob * (*reward as f32 + gamma * values[next_state])
                    }
                    if max_action_value < sum {
                        max_action = action;
                        max_action_value = sum;
                    };
                }
                let action = policy.get_mut(&state).unwrap();
                *action = max_action;
                if *action != old_action {
                    policy_stable = false;
                }
            }
            mdp::Policy::Stochastic(_policy) => {
                unimplemented!()
            }
        }
    }
    if policy_stable {
        return values;
    }
    values = policy_evaluation(agent, &enviorment, states, values, gamma, tolerance);
    return policy_improvement(agent, enviorment, states, gamma, tolerance, values);
}

pub fn policy_iteration<E, S, A>(
    agent: &mut mdp::Agent<S, A>,
    enviorment: E,
    gamma: f32,
    tolerance: f32,
) -> HashMap<S, f32>
where
    A: mdp::Action,
    S: mdp::State,
    E: mdp::Enviorment<S, A>,
{
    let mut rng = thread_rng();
    let mut values: HashMap<S, f32> = HashMap::new();
    let states = enviorment.get_states();
    for state in &states {
        if state.is_terminal() {
            values.insert(state.clone(), 1.0);
        } else {
            values.insert(state.clone(), rng.gen());
        }
    }
    values = policy_evaluation(agent, &enviorment, &states, values, gamma, tolerance);
    return policy_improvement(agent, enviorment, &states, gamma, tolerance, values);
}

pub fn value_iteration<E, S, A>(
    enviorment: &E,
    states: &Vec<S>,
    mut values: HashMap<S, f32>,
    gamma: f32,
    tolerance: f32,
) -> HashMap<S, f32>
where
    A: mdp::Action,
    S: mdp::State,
    E: mdp::Enviorment<S, A>,
{
    loop {
        let mut delta: f32 = 0.0;
        for (i, state) in states.iter().enumerate() {
            let v = values[state];
            let actions = enviorment.posible_actions(&state);
            let mut max_action = &actions[0];
            let mut max_action_value = -f32::INFINITY;

            for action in &actions {
                let probs = enviorment.dynamics(&state, &action);
                let mut sum = 0.0;
                for ((next_state, reward), prob) in probs.iter() {
                    sum += prob * (*reward as f32 + gamma * values[next_state])
                }
                if max_action_value < sum {
                    max_action = action;
                    max_action_value = sum;
                };
            }
            let action = max_action;
            let probs = enviorment.dynamics(&state, &action);
            let mut sum = 0.0;
            for ((next_state, reward), prob) in probs.iter() {
                sum += prob * (*reward as f32 + gamma * values[next_state])
            }
            let value = sum;
            delta = delta.max((v - value).abs());
            let val = values.get_mut(state).unwrap();
            *val = value;
            println!("{i:?}");
            println!("{delta:?}");

            //println!("{values:?}")
        }
        if delta < tolerance {
            break;
        }
    }
    values
}
