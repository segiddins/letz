use std::{f64, hash::Hash};

use strum::{EnumCount, EnumDiscriminants, EnumIter, IntoStaticStr, VariantArray};

use crate::rng::{Rng};
pub use crate::rng::{IntoKey, KeyPart};

#[derive(
    Debug,
    Clone,
    strum::IntoStaticStr,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    VariantArray,
    Copy,
)]
#[strum(const_into_str)]
pub enum Source {
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

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
#[strum(const_into_str)]
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
    Debug,
    Clone,
    strum::IntoStaticStr,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Copy,
    VariantArray,
    EnumCount,
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
    Debug,
    Clone,
    strum::IntoStaticStr,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    VariantArray,
    Copy,
    EnumCount,
)]
pub enum LegendaryJoker {
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, EnumCount, EnumDiscriminants)]
#[strum_discriminants(derive(VariantArray))]
pub enum Item {
    LegendaryJoker(LegendaryJoker),
    Voucher(Voucher),
    Tag(Tag),
    Boss(Boss),
    Tarot(TarotCard),
}

impl Item {
    pub fn as_int(&self) -> u8 {
        match self {
            Item::LegendaryJoker(i) => *i as u8,
            Item::Voucher(v) => LegendaryJoker::COUNT as u8 + *v as u8,
            Item::Tag(t) => LegendaryJoker::COUNT as u8 + Voucher::COUNT as u8 + *t as u8,
            Item::Boss(boss) => match boss {
                Boss::SmallBoss(small) => {
                    LegendaryJoker::COUNT as u8
                        + Voucher::COUNT as u8
                        + Tag::COUNT as u8
                        + *small as u8
                }
                Boss::BigBoss(large) => {
                    LegendaryJoker::COUNT as u8
                        + Voucher::COUNT as u8
                        + Tag::COUNT as u8
                        + SmallBoss::COUNT as u8
                        + *large as u8
                }
            },
            Item::Tarot(t) => {
                LegendaryJoker::COUNT as u8
                    + Voucher::COUNT as u8
                    + Tag::COUNT as u8
                    + SmallBoss::COUNT as u8
                    + BigBoss::COUNT as u8
                    + *t as u8
            }
        }
    }

    pub fn from_u8(mut value: u8) -> Self {
        if value < LegendaryJoker::COUNT as u8 {
            return LegendaryJoker::VARIANTS[value as usize].into();
        }
        value -= LegendaryJoker::COUNT as u8;
        if value < Voucher::COUNT as u8 {
            return Voucher::VARIANTS[value as usize].into();
        }
        unimplemented!()
    }
}

impl From<LegendaryJoker> for Item {
    fn from(val: LegendaryJoker) -> Self {
        Item::LegendaryJoker(val)
    }
}

impl From<Voucher> for Item {
    fn from(val: Voucher) -> Self {
        Item::Voucher(val)
    }
}

impl From<Tag> for Item {
    fn from(val: Tag) -> Self {
        Item::Tag(val)
    }
}

impl From<Boss> for Item {
    fn from(val: Boss) -> Self {
        Item::Boss(val)
    }
}

impl From<SmallBoss> for Item {
    fn from(val: SmallBoss) -> Self {
        Item::Boss(val.into())
    }
}

impl From<BigBoss> for Item {
    fn from(val: BigBoss) -> Self {
        Item::Boss(val.into())
    }
}

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, IntoStaticStr)]
pub enum Boss {
    SmallBoss(SmallBoss),
    BigBoss(BigBoss),
}

impl From<SmallBoss> for Boss {
    fn from(val: SmallBoss) -> Self {
        Boss::SmallBoss(val)
    }
}
impl From<BigBoss> for Boss {
    fn from(val: BigBoss) -> Self {
        Boss::BigBoss(val)
    }
}

#[derive(
    Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, IntoStaticStr, Copy, EnumCount,
)]
pub enum SmallBoss {
    Serpent,
    The_Mouth,
}

#[derive(
    Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, IntoStaticStr, Copy, EnumCount,
)]
pub enum BigBoss {
    VioletVessel,
    CrimsonHeart,
}

#[derive(
    Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, IntoStaticStr, Copy, EnumCount,
)]
pub enum TarotCard {
    The_Fool,
    The_Magician,
    The_High_Priestess,
    The_Empress,
    The_Emperor,
    The_Hierophant,
    The_Lovers,
    The_Chariot,
    Justice,
    The_Hermit,
    The_Wheel_of_Fortune,
    Strength,
    The_Hanged_Man,
    Death,
    Temperance,
    The_Devil,
    The_Tower,
    The_Star,
    The_Moon,
    The_Sun,
    Judgement,
    The_World,
}

impl From<TarotCard> for Item {
    fn from(val: TarotCard) -> Self {
        Item::Tarot(val)
    }
}

#[derive(
    Debug, Clone, strum::Display, Hash, PartialEq, Eq, VariantArray, IntoStaticStr, Copy, EnumCount,
)]
pub enum Voucher {
    Overstock,
    Overstock_Plus,
    Clearance_Sale,
    Liquidation,
    Hone,
    Glow_Up,
    Reroll_Surplus,
    Reroll_Glut,
    Crystal_Ball,
    Omen_Globe,
    Telescope,
    Observatory,
    Grabber,
    Nacho_Tong,
    Wasteful,
    Recyclomancy,
    Tarot_Merchant,
    Tarot_Tycoon,
    Planet_Merchant,
    Planet_Tycoon,
    Seed_Money,
    Money_Tree,
    Blank,
    Antimatter,
    Magic_Trick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    Directors_Cut,
    Retcon,
    Paint_Brush,
    Palette,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ante {
    number: i8,
    skip_tags: [Tag; 2],
    boss: Boss,
    voucher: Voucher,
}

#[derive(Debug, Clone)]
pub struct Game<'a> {
    rng: Rng<'a>,
    ante: Ante,
    locked_items: [bool; 256],
}

impl<'a> Game<'a> {
    fn default_locked_items(&mut self) {
        if self.ante.number < 2 {
            self.lock(Tag::Negative_Tag);
            self.lock(Tag::Standard_Tag);
            self.lock(Tag::Meteor_Tag);
            self.lock(Tag::Buffoon_Tag);
            self.lock(Tag::Handy_Tag);
            self.lock(Tag::Garbage_Tag);
            self.lock(Tag::Ethereal_Tag);
            self.lock(Tag::Top_up_Tag);
            self.lock(Tag::Orbital_Tag);

            Voucher::VARIANTS
                .iter()
                .enumerate()
                .filter_map(|(i, v)| if i.is_multiple_of(2) { None } else { Some(v) })
                .for_each(|v| {
                    self.lock(*v);
                });
        }
    }

    pub fn unlock<T: Into<Item>>(&mut self, item: T) {
        let i = item.into().as_int();
        self.locked_items[i as usize] = false;
    }

    pub fn lock<T: Into<Item>>(&mut self, item: T) {
        let i = item.into().as_int();
        self.locked_items[i as usize] = true;
    }

    pub fn new(seed: &'a str) -> Self {
        let rng = Rng::new(seed);
        let mut game = Game {
            rng,
            ante: Ante {
                number: 1,
                skip_tags: [Tag::Uncommon_Tag, Tag::Rare_Tag],
                boss: Boss::SmallBoss(SmallBoss::Serpent),
                voucher: Voucher::Blank,
            },
            locked_items: [false; 256],
        };
        game.default_locked_items();
        game
    }

    pub fn reset(&mut self, seed: &'a str) {
        self.rng.reseed(seed);
        self.locked_items.fill(false);
        self.ante.number = 1;
        self.default_locked_items();
    }

    pub fn random_choice<E: VariantArray + Copy + Into<Item>, K: IntoKey>(&mut self, key: K) -> E {
        let mut key = key.key();
        let choices = E::VARIANTS;
        for i in 0..u8::MAX {
            if i > 0 {
                key.resample(i - 1);
            }
            let idx = self.rng.roll(key).range(0, choices.len() as u64 - 1);
            let choice = choices[idx as usize];
            if !self.locked_items[choice.into().as_int() as usize] {
                return choice;
            }
        }
        unreachable!("Shouldn't need more than 256 iterations");
    }

    #[inline]
    pub fn next_voucher(&mut self) -> Voucher {
        self.random_choice::<Voucher, _>([Type::Voucher.into(), KeyPart::Ante(self.ante.number)])
    }

    #[inline]
    pub fn next_tag(&mut self) -> Tag {
        self.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(self.ante.number)])
    }

    #[inline]
    pub fn next_legendary_joker(&mut self) -> LegendaryJoker {
        self.random_choice::<LegendaryJoker, _>([Type::Joker_Legendary.into()])
    }

    pub fn purchase_voucher(&mut self, voucher: Voucher) {
        if voucher as isize % 2 == 0 {
            self.lock(voucher);
            self.unlock(Voucher::VARIANTS[voucher as usize + 1]);
        } else {
            self.lock(voucher);
        }
    }

    pub fn random<K: IntoKey>(&mut self, id: K) -> f64 {
        let mut rng = self.rng.roll(id);
        rng.random()
    }

    pub fn seed(&self) -> &str {
        self.rng.seed
    }
}

#[test]
fn test_game_tag_1() {
    let mut game = Game::new("TESTABCD");

    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Coupon_Tag,
        "First tag in ante 1"
    );
    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Speed_Tag,
        "Second tag in ante 1"
    );

    game.reset("ON2PPLR6");

    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Charm_Tag,
        "First tag in ante 1"
    );
    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Foil_Tag,
        "Second tag in ante 1"
    );

    game.reset("3FLBOIVG");
    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Coupon_Tag,
        "First tag in ante 1 for seed {}",
        game.seed()
    );
    assert_eq!(
        game.random_choice::<Tag, _>([Type::Tags.into(), KeyPart::Ante(1)]),
        Tag::Coupon_Tag,
        "Second tag in ante 1"
    );
    assert_eq!(
        game.random_choice::<Voucher, _>([Type::Voucher.into(), KeyPart::Ante(1)]),
        Voucher::Wasteful,
        "Voucher in ante 1"
    );
}

#[test]
fn test_item_reprs() {
    assert_eq!(2, std::mem::size_of::<Item>());
    let all_items: Vec<Item> = ItemDiscriminants::VARIANTS
        .iter()
        .flat_map(|disc| {
            let items: Vec<Item> = match disc {
                ItemDiscriminants::LegendaryJoker => LegendaryJoker::VARIANTS
                    .iter()
                    .map(|i| (*i).into())
                    .collect(),
                ItemDiscriminants::Voucher => {
                    Voucher::VARIANTS.iter().map(|i| (*i).into()).collect()
                }
                ItemDiscriminants::Tag => Tag::VARIANTS.iter().map(|i| (*i).into()).collect(),
                ItemDiscriminants::Boss => SmallBoss::VARIANTS
                    .iter()
                    .map(|i| (*i).into())
                    .chain(BigBoss::VARIANTS.iter().map(|i| (*i).into()))
                    .collect(),
                ItemDiscriminants::Tarot => {
                    TarotCard::VARIANTS.iter().map(|i| (*i).into()).collect()
                }
            };
            items
        })
        .collect();

    assert!(all_items.len() <= 256);

    for (idx, item) in all_items.iter().enumerate() {
        assert_eq!(idx, item.as_int() as usize, "Item {item:?}");
    }
}
