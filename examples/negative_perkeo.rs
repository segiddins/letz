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
        .map_with(Game::new(""), |game, seed| {
            game.reset(seed);

            if game.random_choice::<Tag>("Tag1") != Tag::Charm_Tag {
                return None;
            }

            let mut perkeo_found = None;

            for i in 1..=5 {
                if game.random("soul_Tarot1") <= 0.997 {
                    continue;
                }

                if game.random_choice::<LegendaryJoker>(Type::Joker_Legendary.as_ref())
                    != LegendaryJoker::Perkeo
                {
                    continue;
                }

                if game.random("edisou1") <= 0.997 {
                    continue;
                }

                perkeo_found = Some(i);
            }

            if perkeo_found.is_none() {
                return None;
            }

            let r = format!(
                "Seed: {}, tarot card {}",
                game.seed(),
                perkeo_found.unwrap()
            );

            Some(r)

            // let mut blueprint_found = None;

            // for pos in 1..=2 {
            //     if game.random("cdt1") * 28.0 > 20.0 {
            //         continue;
            //     }

            //     if game.random("rarity1sho") < 0.95 {
            //         continue;
            //     }

            //     if game.random("edisho1") <= 0.997 {
            //         continue;
            //     }

            //     blueprint_found = Some(pos);
            // }

            // // now look for a negative brainstorm in the shop...
            // if blueprint_found.is_none() {
            //     continue;
            // }
            // println!(
            //     "Seed: {}, tarot card {} negative rare {}",
            //     seed,
            //     perkeo_found.unwrap(),
            //     blueprint_found.unwrap()
            // );
        })
        .filter_map(|x| x)
        .for_each(|result| {
            progress.suspend(|| {
                println!("{}", result);
            });
        });
}
