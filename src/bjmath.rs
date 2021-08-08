use crate::card::*;

pub const NUM_PACKS: u32 = 4;
pub const NUM_PLAYERS: u32 = 2; //does not include the dealer

/// Computes the total value of a playing hand.
///
/// The result must be less than or equal to 21
/// to be able to win the round.
///
/// Aces are counted as
/// 11 whenever possible, 1 otherwise.
/// See <https://en.wikipedia.org/wiki/Blackjack>
///
/// For example, hand_value(&vec!(Queen of Hearts, 4 of
/// spades, Ace of Hearts)) == 15
pub fn hand_value(hand: &Vec<Card>) -> u32 {
    let mut value = 0;
    let mut found_ace = false;

    for elem in hand.iter() {
        value += match &elem.rank {
            Rank::Ace => {
                found_ace = true;
                1
            }
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            _ => 10,
        }
    }

    if found_ace && value < 12 {
        value += 10
    }

    value
}

pub fn is_blackjack(hand: &Vec<Card>) -> bool {
    let ranks: Vec<&Rank> = hand.iter().map(|c| &c.rank).collect();
    ranks.len() == 2
        && ranks.contains(&&Rank::Ace)
        && (ranks.contains(&&Rank::Jack)
            || ranks.contains(&&Rank::Queen)
            || ranks.contains(&&Rank::King))
}
