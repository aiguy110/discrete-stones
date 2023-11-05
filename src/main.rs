mod stones;

use stones::Stone;
use rand::prelude::*;

use std::collections::HashSet;

struct StoneGuesses {
    stones: Vec<Stone>,
    guesses: Vec<f64>,
}


struct StoneGuessRefinementOpts {
    no_err_steps_to_stop: usize,
    comb_sizes: Vec<usize>,
    rate: f64,
    debug_interval: Option<u32>
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
    fn refine(&mut self, opts: StoneGuessRefinementOpts) {
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


fn main() {
    let n = 20;
    let stones = Stone::gen_stones(n);
    
    let mut stone_guesses = StoneGuesses {
        stones,
        guesses: vec![0.5; n]
    };

    print!("Actual:  ");
    for i in 0..n {
        print!("{:.2}, ", Stone::reveal_weight(&stone_guesses.stones[i]));
    }
    println!();

    //let mut refine_opts = StoneGuessRefinementOpts::default();
    //refine_opts.debug_interval = Some(100000); 
    //stone_guesses.refine(refine_opts);
    stone_guesses.refine(Default::default());

    print!("Final:   ");
    for i in 0..n {
        print!("{:.2}, ", stone_guesses.guesses[i]);
    }
    println!();
}
