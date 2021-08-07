use crate::card::*;

pub fn hand_value(hand: Vec<Card>) -> u32 {
    let mut value = 0;
    let mut num_aces = 0;

    for elem in hand.iter() {
        value += match &elem.rank {
            Rank::Ace => {
                num_aces += 1;
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

    for _ in 0..num_aces {
        if value < 12 {
            value += 10
        }
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
