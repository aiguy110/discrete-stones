mod stones;
use stones::Stone;

mod guessing;
use guessing::{StoneGuesses, StoneGuessRefinementOpts};

use std::cmp::min;
use std::io::{stdout, Write};
use std::fs::File;

fn main() {
    // Get exclusive access to stdout so we can flush it while rendering progress
    let mut std_out = stdout().lock();

    // Do simulations on different numbers of stones from 2 to 100
    let mut all_stone_guesses = Vec::new();
    let sims_per_level = 100;
    let max_level = 100;
    for n in 2..=max_level {
        let refinement_opts = StoneGuessRefinementOpts {
            comb_sizes: (2..=min(n, 5)).collect(),
            ..Default::default()
        };

        for s in 0..sims_per_level {
            let stones = Stone::gen_stones(n);
            let mut guesses = StoneGuesses::init_from_stones(stones);
            guesses.refine(&refinement_opts);
            all_stone_guesses.push(guesses);

            print!("({}/{}) {:.1}%  \r", n, max_level, (s+1) as f32 / sims_per_level as f32 * 100.0);
            std_out.flush().unwrap();
        }
        println!();
    }

    // Write sim results to a file
    let out_file = File::options()
        .write(true)
        .create(true)
        .open("./stone_sims.json")
        .unwrap(); 
    serde_json::to_writer_pretty(out_file, &all_stone_guesses).unwrap();
}
