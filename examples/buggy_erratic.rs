extern crate letz;

use indicatif::{ParallelProgressIterator, ProgressBar};
use letz::game::*;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

fn main() {
    let seeds = letz::seeds::par_seeds();
    let progress = ProgressBar::new(seeds.len() as u64)
        .with_style(
        indicatif::ProgressStyle::default_bar()
        .template("[{elapsed_precise}] est [{duration_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10} ({percent:>3}%) {per_sec:>5} {msg}")
        .unwrap()
    );
    progress.set_draw_target(indicatif::ProgressDrawTarget::stdout_with_hz(1));
    seeds
        .progress_with(progress.clone())
        .map_with(Game::<'static>::new(""), |game, seed| {
            unsafe {
                // extend lifetime of &seed to 'static
                // SAFETY: this is safe because the game is not used outside of this scope before being reseeded
                game.reset(std::mem::transmute::<&str, &'static str>(&seed));
            }

            let r = game.random([KeyPart::Type(Type::Erratic)]);
            match r {
                0.0..=1.0 => None,
                _ => Some(r),
            }
        })
        .filter_map(|x| x)
        .for_each(|result| {
            progress.suspend(|| {
                println!("{}", result);
            });
        });
}

/*
Seed: FKDRASH6, soul card 4 temperance 3
Seed: UB9DVWII, soul card 5 temperance 3
Seed: OMAUAS39, soul card 2 temperance 3
Seed: NHPJJME5, soul card 2 temperance 5
Seed: XC8SENE5, soul card 4 temperance 3
Seed: RBGL86Y8, soul card 5 temperance 2
Seed: 1I2K35JI, soul card 1 temperance 2
 */
