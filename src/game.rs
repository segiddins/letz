use std::{
    collections::{HashMap, HashSet},
    f64,
    hash::Hash,
};

use strum::{AsRefStr, EnumIter, VariantArray};

use crate::rng::Rng;

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

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq)]
pub enum Item {
    LegendaryJoker(LegendaryJoker),
    Voucher(Voucher),
    Tag(Tag),
    Boss(Boss),
}

impl Into<Item> for LegendaryJoker {
    fn into(self) -> Item {
        Item::LegendaryJoker(self)
    }
}

impl Into<Item> for Voucher {
    fn into(self) -> Item {
        Item::Voucher(self)
    }
}

impl Into<Item> for Tag {
    fn into(self) -> Item {
        Item::Tag(self)
    }
}

impl Into<Item> for Boss {
    fn into(self) -> Item {
        Item::Boss(self)
    }
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, AsRefStr)]
pub enum Boss {
    SmallBoss(SmallBoss),
    BigBoss(BigBoss),
}

impl Into<Boss> for SmallBoss {
    fn into(self) -> Boss {
        Boss::SmallBoss(self)
    }
}
impl Into<Boss> for BigBoss {
    fn into(self) -> Boss {
        Boss::BigBoss(self)
    }
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, AsRefStr)]
pub enum SmallBoss {
    Serpent,
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, AsRefStr)]
pub enum BigBoss {
    VioletVessel,
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, AsRefStr)]
pub enum Voucher {
    Blank,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ante {
    number: i8,
    skip_tags: [Tag; 2],
    boss: Boss,
    voucher: Voucher,
}

#[derive(Debug, Clone)]
pub struct Game {
    rng: Rng,
    ante: Ante,
    locked_items: HashSet<Item>,
}

impl Game {
    pub fn new<S: AsRef<str>>(seed: S) -> Self {
        let rng = Rng::new(seed.as_ref().to_string());
        let game = Game {
            rng,
            ante: Ante {
                number: 0,
                skip_tags: [Tag::Uncommon_Tag, Tag::Rare_Tag],
                boss: Boss::SmallBoss(SmallBoss::Serpent),
                voucher: Voucher::Blank,
            },
            locked_items: HashSet::new(),
        };
        game
    }

    pub fn reset<S: Into<String>>(&mut self, seed: S) {
        self.rng.reseed(seed);
        self.locked_items.clear();
        self.ante.number = 1;
    }

    pub fn random_choice<E: VariantArray + Copy + Into<Item>>(&mut self, id: &str) -> E {
        let choices = E::VARIANTS;
        let idx = self.rng.roll(id).range(0, choices.len() as u64 - 1);
        let choice = choices[idx as usize];
        // TODO: Handle item locking
        // if self.locked_items.contains(&choice.into()) {
        //     todo!("Handle locked items");
        // }
        choice
    }

    pub fn random(&mut self, id: &str) -> f64 {
        let mut rng = self.rng.roll(id);
        rng.random()
    }

    pub fn seed(&self) -> &str {
        &self.rng.seed
    }
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
