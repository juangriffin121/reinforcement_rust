/*
Exercise 5.10: Racetrack (programming) Consider driving a race car around a turn like those shown in
Figure 5.5. You want to go as fast as possible, but not so fast as to run off the track. In our simplified
racetrack, the car is at one of a discrete set of grid positions, the cells in the diagram. The velocity is
also discrete, a number of grid cells moved horizontally and vertically per time step. The actions are
increments to the velocity components. Each may be changed by +1, −1, or 0 in each step, for a total
of nine (3 × 3) actions. Both velocity components are restricted to be nonnegative and less than 5,
and they cannot both be zero except at the starting line. Each episode begins in one of the randomly
selected start states with both velocity components zero and ends when the car crosses the finish line.
The rewards are −1 for each step until the car crosses the finish line. If the car hits the track boundary,
it is moved back to a random position on the starting line, both velocity components are reduced to
zero, and the episode continues. Before updating the car’s location at each time step, check to see if
the projected path of the car intersects the track boundary. If it intersects the finish line, the episode
ends; if it intersects anywhere else, the car is considered to have hit the track boundary and is sent
back to the starting line. To make the task more challenging, with probability 0.1 at each time step
the velocity increments are both zero, independently of the intended increments. Apply a Monte Carlo
control method to this task to compute the optimal policy from each starting state. Exhibit several
trajectories following the optimal policy (but turn the noise off for these trajectories).
*/

use std::{collections::HashMap, iter::zip, usize};

use rand::thread_rng;

use rand::seq::SliceRandom;

use crate::bases::{
    mdp::{Action, Enviorment, Policy, State},
    monte_carlo_control::first_visit_monte_carlo_control,
};

use super::ex4_3::Casino;

pub fn get_race_track() -> RaceTrack {
    let squares = [
        6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 10, 17, 17, 16,
        15, 15, 14,
    ];

    let starting_points = [
        3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2,
        2, 3,
    ];

    let finish_line = vec![(16, 26), (16, 27), (16, 28), (16, 29), (16, 30), (16, 31)];
    let starting_line = vec![(4, 0), (5, 0), (6, 0), (7, 0)];
    //println!("{}", squares.len());
    //println!("{}", starting_points.len());
    let track = make_track(&squares, &starting_points, 17);
    let racetrack = RaceTrack {
        track,
        finish_line: finish_line.clone(),
        starting_line,
    };
    graph_track(&squares, &starting_points, &finish_line, &racetrack, 17);
    racetrack
}

pub fn intersects_boundary(position: (usize, usize), racetrack: &RaceTrack) -> bool {
    let track = &racetrack.track;
    let (x, y) = position;
    let ends = is_in_finish_line(&racetrack.finish_line, (x as u32, y as u32));
    let starts = racetrack.starting_line.contains(&(x as u32, y as u32));
    if starts || ends {
        return false;
    }
    let beyond_array =
        y + 1 >= track.len() || y as i32 - 1 < 0 || x + 1 >= track[0].len() || x as i32 - 1 < 0;
    if beyond_array {
        return true;
    }
    !track[y][x] || !track[y + 1][x] || !track[y - 1][x] || !track[y][x + 1] || !track[y][x - 1]
}

pub fn intersects_finish_line(finish_line: &[(u32, u32)], state: &CarState) -> bool {
    let max_y: u32 = finish_line.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_y: u32 = finish_line.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let finish_x = finish_line[0].0;
    let car_x = state.position.0 + state.velocity.0;
    let car_y = state.position.1 + state.velocity.1;
    if is_in_finish_line(finish_line, (car_x, car_y)) {
        return true;
    }
    car_y <= min_y && car_y >= max_y && car_x >= finish_x
}
pub fn is_in_finish_line(finish_line: &[(u32, u32)], position: (u32, u32)) -> bool {
    finish_line.contains(&position)
}
pub fn graph_track(
    squares: &[i32],
    starting_points: &[i32],
    finish_line: &[(u32, u32)],
    track: &RaceTrack,
    max_width: i32,
) {
    let mut txt = "".to_string();
    for (y, (square, starting_point)) in zip(squares, starting_points).enumerate().rev() {
        for x in 0..max_width {
            if x < *starting_point {
                txt += ".";
            } else if x < *starting_point + *square {
                if y == 0 {
                    txt += "S";
                } else if is_in_finish_line(
                    finish_line,
                    (x.try_into().unwrap(), y.try_into().unwrap()),
                ) {
                    txt += "F";
                } else if intersects_boundary((x.try_into().unwrap(), y.try_into().unwrap()), track)
                {
                    txt += "B";
                } else {
                    txt += "#";
                }
            } else {
                txt += ".";
            }
        }
        txt += "\n"
    }
    println!("{}", txt)
}

pub fn make_track(squares: &[i32], starting_points: &[i32], max_width: i32) -> Vec<Vec<bool>> {
    let mut track = Vec::new();
    for (square, starting_point) in zip(squares, starting_points) {
        let mut track_slice = Vec::new();
        for x in 0..max_width {
            if x >= *starting_point && x < *starting_point + square {
                track_slice.push(true)
            } else {
                track_slice.push(false)
            }
        }
        track.push(track_slice);
    }
    track
}

pub struct RaceTrack {
    pub track: Vec<Vec<bool>>,
    pub finish_line: Vec<(u32, u32)>,
    pub starting_line: Vec<(u32, u32)>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct CarAction {
    pub velocity_increment: (i32, i32),
}
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct CarState {
    pub velocity: (u32, u32),
    pub position: (u32, u32),
}

impl Action for CarAction {}
impl State for CarState {}

impl<'a> Enviorment<'a, CarState, CarAction> for RaceTrack {
    fn response(&self, state: &CarState, action: &CarAction) -> (CarState, i32) {
        let mut rng = thread_rng();
        if self.is_terminal(state) {
            return (*state, 0);
        }
        let (mut x, mut y) = state.position;
        let (mut vx, mut vy) = state.velocity;
        let (ax, ay) = action.velocity_increment;

        (x, y) = (x + vx, y + vy);
        (vx, vy) = (
            (vx as i32 + ax).min(5).max(0).try_into().unwrap(),
            (vy as i32 + ay).min(5).max(0).try_into().unwrap(),
        );

        if intersects_boundary((x.try_into().unwrap(), y.try_into().unwrap()), &self) {
            (x, y) = self.starting_line.choose(&mut rng).unwrap().clone(); // choose random from starting_points
            (vx, vy) = (0, 1);
        }

        let next_state = CarState {
            velocity: (vx, vy),
            position: (x, y),
        };
        let reward = -1;
        (next_state, reward)
    }
    fn is_terminal(&self, state: &CarState) -> bool {
        let answer = intersects_finish_line(&self.finish_line, state);
        answer
    }
    fn posible_actions(&self, state: &CarState) -> Vec<CarAction> {
        let _ = state;
        let mut actions = Vec::new();
        for ax in -1..2 {
            for ay in -1..2 {
                actions.push(CarAction {
                    velocity_increment: (ax, ay),
                })
            }
        }
        actions
    }
    fn get_states(&self) -> Vec<CarState> {
        let mut states = Vec::new();
        for (y, track_slice) in self.track.iter().enumerate() {
            for (x, square) in track_slice.iter().enumerate() {
                if *square {
                    for vx in 0..6 {
                        for vy in 0..6 {
                            states.push(CarState {
                                position: (x.try_into().unwrap(), y.try_into().unwrap()),
                                velocity: (vx, vy),
                            })
                        }
                    }
                }
            }
        }
        states
    }
}

pub fn solution5_10() {
    let mut rng = thread_rng();
    let env = get_race_track();
    let mut map = HashMap::new();
    let episodes = 1000;
    let epsilon = 0.2;
    let gamma = 1.0;
    let states = env.get_states();
    let actions = env.posible_actions(&states[0]);
    for state in &states {
        let max_action;
        if state.position.1 < 13 {
            max_action = CarAction {
                velocity_increment: (0, 1),
            };
        } else if state.position.1 < 24 {
            max_action = CarAction {
                velocity_increment: (0, -1),
            };
        } else {
            max_action = CarAction {
                velocity_increment: (0, -1),
            };
        }
        let mut choice_dist = HashMap::new();
        for action in &actions {
            let prob = if *action == max_action {
                1.0 - epsilon + epsilon / actions.len() as f32
            } else {
                epsilon / actions.len() as f32
            };
            choice_dist.insert(action, prob);
        }
        map.insert(state, choice_dist);
    }

    let init_pol = Policy::Stochastic(map);
    let mut init_vals = HashMap::new();

    for state in &states {
        for action in &actions {
            init_vals.insert((state.clone(), action.clone()), -500.0);
        }
    }
    let init_states: Vec<CarState> = env
        .starting_line
        .iter()
        .map(|&x| CarState {
            velocity: (0, 0),
            position: x,
        })
        .collect();

    first_visit_monte_carlo_control(
        init_pol,
        init_vals,
        init_states,
        episodes,
        env,
        epsilon,
        gamma,
    )
}
