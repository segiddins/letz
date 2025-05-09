extern crate letz;

use indicatif::ParallelProgressIterator;
use letz::{game::*, seeds::par_seeds};
use rayon::iter::ParallelIterator;

fn main() {
    par_seeds()
        .progress_with_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] est [{duration_precise}] {bar:40.cyan/blue} {pos:>10}/{len:10} ({percent:>3}%) {per_sec:>5}")
                .unwrap(),
        )
        .filter_map(|seed| {
            let mut game = Game::new(&seed);
            if game.random_choice::<Tag>("Tag1") != Tag::Charm_Tag {
                return None;
            }

            let mut perkeo_found = None;

            for i in 1..=5 {
                if game.random("soul_Tarot1") <= 0.997 {
                    return None;
                }

                if game.random_choice::<LegendaryJoker>(Type::Joker_Legendary.as_ref())
                    != LegendaryJoker::Perkeo
                {
                    return None;
                }

                if game.random("edisou1") <= 0.997 {
                    return None;
                }

                perkeo_found = Some(i);
            }

            if perkeo_found.is_none() {
                return None;
            }

            Some(format!(
                "Seed: {}, tarot card {}",
                seed,
                perkeo_found.unwrap()
            ))

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
        .for_each(|result| {
            println!("{}", result);
        });
}
