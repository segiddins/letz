use std::{collections::HashMap, f64, hash::Hash};

use strum::{EnumIter, VariantArray};

use crate::lua_random::LuaRandom;

struct Shop {}
const EXTRA: f64 = 0.561892350821;

#[derive(
    Debug, Clone, strum::AsRefStr, strum::Display, Hash, PartialEq, Eq, EnumIter, VariantArray,
)]
enum Source {
    #[strum(to_string = "sho")]
    Shop,
    #[strum(to_string = "emp")]
    Emperor,
    #[strum(to_string = "pri")]
    High_Priestess,
    #[strum(to_string = "jud")]
    Judgement,
    #[strum(to_string = "wra")]
    Wraith,
    #[strum(to_string = "ar1")]
    Arcana,
    #[strum(to_string = "pl1")]
    Celestial,
    #[strum(to_string = "spe")]
    Spectral,
    #[strum(to_string = "sta")]
    Standard,
    #[strum(to_string = "buf")]
    Buffoon,
    #[strum(to_string = "vag")]
    Vagabond,
    #[strum(to_string = "sup")]
    Superposition,
    #[strum(to_string = "8ba")]
    Eight_Ball,
    #[strum(to_string = "sea")]
    Seance,
    #[strum(to_string = "sixth")]
    Sixth_Sense,
    #[strum(to_string = "top")]
    Top_Up,
    #[strum(to_string = "rta")]
    Rare_Tag,
    #[strum(to_string = "uta")]
    Uncommon_Tag,
    #[strum(to_string = "blusl")]
    Blue_Seal,
    #[strum(to_string = "8ba")]
    Purple_Seal,
    #[strum(to_string = "sou")]
    Soul,
    #[strum(to_string = "rif")]
    Riff_Raff,
    #[strum(to_string = "car")]
    Cartomancer,
}

// enum Suit {
//     Spade,
//     Heart,
//     Diamond,
//     Club,
// }

// enum Enhancement {}
// enum Edition {}
// enum Seal {}

// struct Card {
//     suit: Suit,
//     value: u8,
//     enhancement: Option<Enhancement>,
//     edition: Option<Edition>,
//     seal: Option<Seal>,
// }

#[derive(Debug, Clone, strum::AsRefStr, strum::Display, Hash, PartialEq, Eq, EnumIter)]
pub enum Type {
    #[strum(to_string = "Joker1")]
    Joker_Common,
    #[strum(to_string = "Joker2")]
    Joker_Uncommon,
    #[strum(to_string = "Joker3")]
    Joker_Rare,
    #[strum(to_string = "Joker4")]
    Joker_Legendary,
    #[strum(to_string = "rarity")]
    Joker_Rarity,
    #[strum(to_string = "edi")]
    Joker_Edition,
    #[strum(to_string = "misprint")]
    Misprint,
    #[strum(to_string = "stdset")]
    Standard_Has_Enhancement,
    #[strum(to_string = "Enhanced")]
    Enhancement,
    #[strum(to_string = "front")]
    Card,
    #[strum(to_string = "standard_edition")]
    Standard_Edition,
    #[strum(to_string = "stdseal")]
    Standard_Has_Seal,
    #[strum(to_string = "stdsealtype")]
    Standard_Seal,
    #[strum(to_string = "shop_pack")]
    Shop_Pack,
    #[strum(to_string = "Tarot")]
    Tarot,
    #[strum(to_string = "Spectral")]
    Spectral,
    #[strum(to_string = "Tag")]
    Tags,
    #[strum(to_string = "nr")]
    Shuffle_New_Round,
    #[strum(to_string = "cdt")]
    Card_Type,
    #[strum(to_string = "Planet")]
    Planet,
    #[strum(to_string = "lucky_mult")]
    Lucky_Mult,
    #[strum(to_string = "lucky_money")]
    Lucky_Money,
    #[strum(to_string = "sigil")]
    Sigil,
    #[strum(to_string = "ouija")]
    Ouija,
    #[strum(to_string = "wheel_of_fortune")]
    Wheel_of_Fortune,
    #[strum(to_string = "gros_michel")]
    Gros_Michel,
    #[strum(to_string = "cavendish")]
    Cavendish,
    #[strum(to_string = "Voucher")]
    Voucher,
    #[strum(to_string = "Voucher_fromtag")]
    Voucher_Tag,
    #[strum(to_string = "orbital")]
    Orbital_Tag,
    #[strum(to_string = "soul_")]
    Soul,
    #[strum(to_string = "erratic")]
    Erratic,
    #[strum(to_string = "stake_shop_joker_eternal")]
    Eternal,
    #[strum(to_string = "ssjp")]
    Perishable,
    #[strum(to_string = "ssjr")]
    Rental,
    #[strum(to_string = "etperpoll")]
    Eternal_Perishable,
    #[strum(to_string = "packssjr")]
    Rental_Pack,
    #[strum(to_string = "packetper")]
    Eternal_Perishable_Pack,
    #[strum(to_string = "boss")]
    Boss,
}

#[derive(
    Debug, Clone, strum::AsRefStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy, VariantArray,
)]
pub enum Tag {
    Uncommon_Tag,
    Rare_Tag,
    Negative_Tag,
    Foil_Tag,
    Holographic_Tag,
    Polychrome_Tag,
    Investment_Tag,
    Voucher_Tag,
    Boss_Tag,
    Standard_Tag,
    Charm_Tag,
    Meteor_Tag,
    Buffoon_Tag,
    Handy_Tag,
    Garbage_Tag,
    Ethereal_Tag,
    Coupon_Tag,
    Double_Tag,
    Juggle_Tag,
    D6_Tag,
    Top_up_Tag,
    Speed_Tag,
    Orbital_Tag,
    Economy_Tag,
}

#[derive(
    Debug, Clone, strum::AsRefStr, strum::Display, Hash, PartialEq, Eq, EnumIter, VariantArray, Copy,
)]
pub enum LegendaryJoker {
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

#[derive(Debug, Clone)]
struct Rng {
    seed: String,
    hashed_seed: f64,
    states: HashMap<String, f64>,
}

impl Rng {
    fn new(seed: String) -> Self {
        let hashed_seed = pseudohash(&seed);
        Rng {
            seed,
            hashed_seed,
            states: HashMap::new(),
        }
    }

    fn roll<S: AsRef<str>>(&mut self, key: S) -> LuaRandom {
        let state = self
            .states
            .entry(key.as_ref().to_string())
            .or_insert_with_key(|key| pseudohash(format!("{}{}", key, self.seed)));

        let value = (*state * 1.72431234 + 2.134453429141).fract().abs();
        let value = round13(value);
        *state = value;
        let seed = (value + self.hashed_seed) / 2.0;
        LuaRandom::seed(seed)
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    rng: Rng,
}

impl Game {
    pub fn new<S: AsRef<str>>(seed: S) -> Self {
        let rng = Rng::new(seed.as_ref().to_string());
        Game { rng }
    }

    pub fn random_choice<E: VariantArray + Copy>(&mut self, id: &str) -> E {
        let choices = E::VARIANTS;
        let idx = self.rng.roll(id).range(0, choices.len() as u64 - 1);
        choices[idx as usize]
    }

    pub fn random(&mut self, id: &str) -> f64 {
        let mut rng = self.rng.roll(id);
        rng.random()
    }
}

fn pseudohash<S: AsRef<str>>(s: S) -> f64 {
    let mut num = 1.0;
    for i in (0..s.as_ref().len()).rev() {
        let c = s.as_ref().as_bytes()[i] as f64;
        num =
            ((1.1239285023 / num) * c * f64::consts::PI + f64::consts::PI * (i + 1) as f64).fract();
    }
    if num.is_nan() {
        return f64::NAN;
    }
    num
}

fn round13(f: f64) -> f64 {
    format!("{:.13}", f).parse::<f64>().unwrap()
}

#[test]
fn test_game_tag_1() {
    let mut game = Game::new("TESTABCD");

    assert_eq!(
        game.random_choice::<Tag>("Tag1"),
        Tag::Coupon_Tag,
        "First tag in ante 1"
    );
    assert_eq!(
        game.random_choice::<Tag>("Tag1"),
        Tag::Speed_Tag,
        "Second tag in ante 1"
    );
}
