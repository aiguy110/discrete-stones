use crate::stones::Stone;
use rand::prelude::*;
use serde::Serialize;

use std::collections::HashSet;

#[derive(Serialize)]
pub struct StoneGuesses {
    pub stones: Vec<Stone>,
    pub guesses: Vec<f64>,
}

pub struct StoneGuessRefinementOpts {
    pub no_err_steps_to_stop: usize,
    pub comb_sizes: Vec<usize>,
    pub rate: f64,
    pub debug_interval: Option<u32>
}

impl Default for StoneGuessRefinementOpts {
    fn default() -> Self {
        StoneGuessRefinementOpts { 
            no_err_steps_to_stop: 1000, 
            comb_sizes: vec![2,3,4,5], 
            rate: 0.01,
            debug_interval: None
        }
    }
}


impl StoneGuesses {
    pub fn refine(&mut self, opts: &StoneGuessRefinementOpts) {
        let mut no_err_steps = 0;
        let mut total_steps = 0;
        
        while no_err_steps < opts.no_err_steps_to_stop {
            // Get a random sample of k stones
            let k = *opts.comb_sizes.choose(&mut rand::thread_rng()).unwrap();
            let sample_inds = self.get_random_sample_inds(k);
            let sample_stones = sample_inds.iter()
                .map(|i| self.stones[*i])
                .collect::<Vec<_>>();

            // Figure out how far off our guesses currently are
            let guessed_weight = sample_inds.iter()
                .map(|i| self.guesses[*i])
                .sum::<f64>()
                .floor() // TODO: Think hard about whether we really want to floor here
                as u32; 
            let measured_weight = Stone::weigh(&sample_stones);
            let err = guessed_weight as i32 - measured_weight as i32;

            // Apply correct (or report clean step)
            if err == 0 {
                no_err_steps += 1;
            } else {
                no_err_steps = 0;
                for i in sample_inds {
                    self.guesses[i] -= opts.rate * (err as f64) / (k as f64);
                }
            }

            // Print our progress if applicable
            if opts.debug_interval.is_some() && total_steps % opts.debug_interval.unwrap()  == 0 { 
                print!("Guesses: ");
                for i in 0..self.guesses.len() {
                    print!("{:.2}, ", self.guesses[i]);
                }
                println!();
            }

            total_steps += 1
        }
    }

    pub fn init_from_stones(stones: Vec<Stone>) -> Self {
        let n = stones.len();
        Self {
            stones,
            guesses: vec![0.5; n]
        }
    }

    fn get_random_sample_inds(&self, k: usize) -> Vec<usize> {
        if k > self.stones.len() {
            panic!("Can't pick {k} things from a set of {}!", self.stones.len());
        }
        let mut sample = HashSet::new();
        let mut rng = rand::thread_rng();

        while sample.len() < k {
            let i = rng.gen_range(0..self.stones.len());
            sample.insert(i);
        }

        sample.into_iter().collect()
    }
}

