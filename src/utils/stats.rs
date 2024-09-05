use core::panic;
use std::{collections::HashMap, hash::Hash};

use rand::{thread_rng, Rng};

pub fn sample_from_hashmap_dist<I>(distribution: &HashMap<I, f32>) -> I
where
    I: Clone,
{
    let mut rng = thread_rng();
    let cutoff: f32 = rng.gen_range(0.0..1.0);
    let mut cdf = 0.;
    let mut last_item: I = distribution.iter().next().unwrap().0.clone();
    for (item, prob) in distribution.iter() {
        last_item = item.clone();
        cdf += prob;
        if cdf > cutoff {
            return last_item;
        }
    }
    last_item
}

pub struct HashMapDistribution<I>
where
    I: Hash,
{
    pub map: HashMap<I, f32>,
}
