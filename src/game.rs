use std::{f64, hash::Hash, num::NonZero};

use enum_map::EnumMap;
use strum::{EnumCount, EnumDiscriminants, EnumIter, IntoStaticStr, VariantArray};

use crate::rng::Rng;
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
    enum_map::Enum,
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

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
enum Suit {
    Heart,
    Club,
    Diamond,
    Spade,
}

#[derive(Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq)]
enum Blind {
    Small,
    Big,
    Boss(Boss),
}

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
enum BoosterPack {
    Arcana_Pack,
    Jumbo_Arcana_Pack,
    Mega_Arcana_Pack,
    Celestial_Pack,
    Jumbo_Celestial_Pack,
    Mega_Celestial_Pack,
    Standard_Pack,
    Jumbo_Standard_Pack,
    Mega_Standard_Pack,
    Buffoon_Pack,
    Jumbo_Buffoon_Pack,
    Mega_Buffoon_Pack,
    Spectral_Pack,
    Jumbo_Spectral_Pack,
    Mega_Spectral_Pack,
}

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
enum Enhancement {
    Bonus_Card,
    Mult_Card,
    Wild_Card,
    Glass_Card,
    Steel_Card,
    Stone_Card,
    Gold_Card,
    Lucky_Card,
}

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
enum Edition {
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(
    Debug, Clone, strum::IntoStaticStr, strum::Display, Hash, PartialEq, Eq, EnumIter, Copy,
)]
enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
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
    enum_map::Enum,
)]
pub enum SpectralCard {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    Deja_Vu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    The_Soul,
    Black_Hole,
}

impl From<SpectralCard> for Item {
    fn from(val: SpectralCard) -> Self {
        Item::SpectralCard(val)
    }
}

// struct Card {
//     suit: Suit,
//     value: u8,
//     enhancement: Option<Enhancement>,
//     edition: Option<Edition>,
//     seal: Option<Seal>,
// }

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
    enum_map::Enum,
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
    enum_map::Enum,
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
    enum_map::Enum,
)]
pub enum LegendaryJoker {
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

pub enum Deck {
    Red_Deck,
    Blue_Deck,
    Yellow_Deck,
    Green_Deck,
    Black_Deck,
    Magic_Deck,
    Nebula_Deck,
    Ghost_Deck,
    Abandoned_Deck,
    Checkered_Deck,
    Zodiac_Deck,
    Painted_Deck,
    Anaglyph_Deck,
    Plasma_Deck,
    Erratic_Deck,
    Challenge_Deck,
}

#[derive(
    enum_map::Enum, Debug, Clone, strum::Display, Hash, PartialEq, Eq, EnumCount, EnumDiscriminants,
)]
#[strum_discriminants(derive(VariantArray))]
pub enum Item {
    LegendaryJoker(LegendaryJoker),
    Voucher(Voucher),
    Tag(Tag),
    Boss(Boss),
    Tarot(TarotCard),
    SpectralCard(SpectralCard),
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
            Item::SpectralCard(s) => {
                LegendaryJoker::COUNT as u8
                    + Voucher::COUNT as u8
                    + Tag::COUNT as u8
                    + SmallBoss::COUNT as u8
                    + BigBoss::COUNT as u8
                    + TarotCard::COUNT as u8
                    + *s as u8
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
        value -= Voucher::COUNT as u8;
        if value < Tag::COUNT as u8 {
            return Tag::VARIANTS[value as usize].into();
        }
        value -= Tag::COUNT as u8;
        if value < SmallBoss::COUNT as u8 {
            return SmallBoss::VARIANTS[value as usize].into();
        }
        value -= SmallBoss::COUNT as u8;
        if value < BigBoss::COUNT as u8 {
            return BigBoss::VARIANTS[value as usize].into();
        }
        value -= BigBoss::COUNT as u8;
        if value < TarotCard::COUNT as u8 {
            return TarotCard::VARIANTS[value as usize].into();
        }
        value -= TarotCard::COUNT as u8;
        if value < SpectralCard::COUNT as u8 {
            return SpectralCard::VARIANTS[value as usize].into();
        }
        panic!("Invalid item value: {}", value);
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

#[derive(Debug, Clone, strum::Display, Hash, PartialEq, Eq, IntoStaticStr, enum_map::Enum)]
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
    Debug,
    Clone,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    VariantArray,
    IntoStaticStr,
    Copy,
    EnumCount,
    enum_map::Enum,
)]
pub enum SmallBoss {
    Serpent,
    The_Mouth,
}

#[derive(
    Debug,
    Clone,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    VariantArray,
    IntoStaticStr,
    Copy,
    EnumCount,
    enum_map::Enum,
)]
pub enum BigBoss {
    VioletVessel,
    CrimsonHeart,
}

#[derive(
    Debug,
    Clone,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    VariantArray,
    IntoStaticStr,
    Copy,
    EnumCount,
    enum_map::Enum,
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
    Debug,
    Clone,
    strum::Display,
    Hash,
    PartialEq,
    Eq,
    VariantArray,
    IntoStaticStr,
    Copy,
    EnumCount,
    enum_map::Enum,
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
    locked_items: EnumMap<Item, bool>,
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
        self.locked_items[item.into()] = false;
    }

    pub fn lock<T: Into<Item>>(&mut self, item: T) {
        self.locked_items[item.into()] = true;
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
            locked_items: Default::default(),
        };
        game.default_locked_items();
        game
    }

    pub fn reset(&mut self, seed: &'a str) {
        self.rng.reseed(seed);
        self.locked_items.clear();
        self.ante.number = 1;
        self.default_locked_items();
    }

    pub fn random_choice<E: VariantArray + Copy + Into<Item>, K: IntoKey>(&mut self, key: K) -> E {
        let mut key = key.key();
        let choices = E::VARIANTS;
        for i in 0..u8::MAX {
            if i > 0 {
                key.resample(NonZero::new(i).unwrap());
            }
            let idx = self.rng.roll(key).range(0, choices.len() as u64 - 1);
            let choice = choices[idx as usize];
            if !self.locked_items[choice.into()] {
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
                ItemDiscriminants::SpectralCard => {
                    SpectralCard::VARIANTS.iter().map(|i| (*i).into()).collect()
                }
            };
            items
        })
        .collect();

    assert!(all_items.len() <= u8::MAX as usize);

    for (idx, item) in all_items.iter().enumerate() {
        assert_eq!(idx, item.as_int() as usize, "Item {item:?}");
        assert_eq!(
            *item,
            Item::from_u8(idx as u8),
            "Item {item:?} from_u8 {idx}"
        );
    }
}

#[test]
fn test_item_enum_map() {
    let map: EnumMap<Item, bool> = EnumMap::default();
    assert_eq!(map.len(), 105);
}

static_assertions::assert_eq_size!(Item, u16);
