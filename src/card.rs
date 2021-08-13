use std::fmt::Display;

/// The four Suits of any classical card 
/// game. 
/// 
/// This enum implements PartialEq and 
/// Display.
/// It also includes a static method [Suit::from_int] 
/// that maps an u32 supplied as argument to one of the suits.
#[derive(PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Spades => "♠",
                Suit::Hearts => "♥",
                Suit::Diamonds => "♦",
                Suit::Clubs => "♣",
            }
        )
    }
}

impl Suit {
    pub fn from_int(a: u32) -> Suit {
        match a % 4 {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}

/// The thirteen Ranks of any classical card 
/// game. 
/// 
/// This enum implements PartialEq and 
/// Display.
/// It also includes a static method [Rank::from_int] 
/// that maps an u32 supplied as argument to one of the ranks.
#[derive(PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}

impl Rank {
    pub fn from_int(a: u32) -> Rank {
        match a % 13 {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            _ => Rank::Ace,
        }
    }
}


/// A playing Card composed of a [Suit] and a
/// [Rank].
/// 
/// This struct implements Display.
/// It also includes a static method [Card::card_pack] to
/// create a new card pack of 52 [Card]s,
/// made of all combinations of [Suit]s and [Rank]s
#[derive(PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn card_pack() -> Vec<Card> {
        (0..4)
            .flat_map(|s| {
                (0..13).map(move |r| Card {
                    rank: Rank::from_int(r),
                    suit: Suit::from_int(s),
                })
            })
            .collect()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
