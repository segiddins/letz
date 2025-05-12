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

            if game.next_voucher() != Voucher::Telescope {
                return None;
            }
            game.purchase_voucher(Voucher::Telescope);
            if game.random_choice::<Voucher, _>([Type::Voucher.into(), KeyPart::Ante(2)])
                != Voucher::Observatory
            {
                return None;
            }

            if game.next_tag() != Tag::Charm_Tag {
                return None;
            }

            let mut perkeo_found = None;
            let mut temperance_found = None;

            for i in 1..=5 {
                if game.random([Source::Soul.into(), Type::Tarot.into(), KeyPart::Ante(1)]) <= 0.997
                {
                    if temperance_found.is_none()
                        && game.random_choice::<TarotCard, _>([
                            Type::Tarot.into(),
                            Source::Arcana.into(),
                            KeyPart::Ante(1),
                        ]) == TarotCard::Temperance
                    {
                        temperance_found = Some(i);
                    }
                    continue;
                }

                if game.next_legendary_joker() != LegendaryJoker::Perkeo {
                    continue;
                }

                if game.random([
                    Type::Joker_Edition.into(),
                    Source::Soul.into(),
                    KeyPart::Ante(1),
                ]) <= 0.997
                {
                    continue;
                }

                perkeo_found = Some(i);
            }

            let (Some(perkeo), Some(temperance)) = (perkeo_found, temperance_found) else {
                return None;
            };

            let r = format!(
                "Seed: {}, soul card {perkeo} temperance {temperance}",
                game.seed(),
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
