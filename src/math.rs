use crate::card::*;

pub const NUM_PACKS: usize = 4;
pub const NUM_PLAYERS: usize = 2;
pub const NUM_PLAYERS_AND_DEALER: usize = NUM_PLAYERS + 1;

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
fn hand_value(hand: &Vec<Card>) -> u32 {
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
    hand.len() == 2 && hand_value(hand) == 21
}

pub fn is_splittable(hand: &Vec<Card>) -> bool {
    hand.len() == 2 && {
        let hand_0 = hand.get(0).unwrap();
        let hand_1 = hand.get(1).unwrap();
        hand_0.rank == hand_1.rank
    }
}

/// Computes the scores of all players, including the dealer,
/// according to [hand_value].
///
/// This method returns an array of scores as u32 values. The
/// dealer score is the last.
pub fn compute_scores(
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
) -> [(u32, Option<u32>); NUM_PLAYERS_AND_DEALER] {
    let mut scores = [(0, None); NUM_PLAYERS_AND_DEALER];
    for (index, hand) in player_hands.iter().enumerate() {
        scores[index] = (hand_value(&hand.0), hand.1.as_ref().map(|v| hand_value(&v)));
    }
    scores[NUM_PLAYERS] = (hand_value(dealer_hand), None);
    scores
}

//todo delete
pub fn compute_winner(scores: [(u32, Option<u32>); NUM_PLAYERS_AND_DEALER]) -> (Vec<usize>, u32) {
    let mut winner_index: Vec<usize> = Vec::new();
    let mut winner_score = 0;

    for (current_index, &score) in scores.iter().enumerate() {
        if score.0 == winner_score {
            winner_index.push(current_index);
        } else if score.0 > winner_score && score.0 <= 21 {
            winner_score = score.0;
            winner_index.clear();
            winner_index.push(current_index)
        }
    }
    (winner_index, winner_score)
}
