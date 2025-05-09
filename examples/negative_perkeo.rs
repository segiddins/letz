extern crate letz;

use letz::game::*;

fn main() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    let mut seed = String::new();

    for i in 38480000000..usize::MAX {
        seed.clear();
        for idx in 0..8 {
            let idx = (i >> (idx * 3)) % alphabet.len();
            seed.push(alphabet.as_bytes()[idx] as char);
        }

        if i % 10_000_000 == 0 {
            println!("Count: {}", i);
        }

        let mut game = Game::new(&seed);
        if game.random_choice::<Tag>("Tag1") != Tag::Charm_Tag {
            continue;
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
            continue;
        }

        // println!("Seed: {}, tarot card {}", seed, perkeo_found.unwrap());

        let mut blueprint_found = None;

        for pos in 1..=2 {
            if game.random("cdt1") * 28.0 > 20.0 {
                continue;
            }

            if game.random("rarity1sho") < 0.95 {
                continue;
            }

            if game.random("edisho1") <= 0.997 {
                continue;
            }

            blueprint_found = Some(pos);
        }

        // now look for a negative brainstorm in the shop...
        if blueprint_found.is_none() {
            continue;
        }
        println!(
            "Seed: {}, tarot card {} negative rare {}",
            seed,
            perkeo_found.unwrap(),
            blueprint_found.unwrap()
        );
    }
}
