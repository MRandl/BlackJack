use crate::card::*;

/// The number of packs to use when shuffling the cards.
pub const NUM_PACKS: usize = 4;

/// The amount of non-dealer players.
pub const NUM_PLAYERS: usize = 2;

/// The amount of players, including the dealer.
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

/// This method returns true whenever its argument
/// is a hand that corresponds to a BlackJack.
pub fn is_blackjack(hand: &Vec<Card>) -> bool {
    hand.len() == 2 && hand_value(hand) == 21
}

/// This method returns true whenever its argument
/// is a hand that can be legally split.
///
/// This occurs when the player has not hit yet,
/// and both his cards have the same rank.
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

/// This method computes which player beat the dealer,
/// reached equality, or lost, based on an array of scores.
///
/// This outputs three Vectors of (usize, bool).
/// The first Vector corresponds to hands that won against
/// the dealer. The second correspond to equalities, and the
/// third to losses. The tuples correspond to the index of
/// the player, and a boolean equal to true if and only if
/// the hand is the second of the player, in case of a split.
/// Typically, the boolean is false.
pub fn compute_result(
    scores: [(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
) -> (Vec<(usize, bool)>, Vec<(usize, bool)>, Vec<(usize, bool)>) {
    let mut winner_index: Vec<(usize, bool)> = Vec::new();
    let mut equal_index: Vec<(usize, bool)> = Vec::new();
    let mut loser_index: Vec<(usize, bool)> = Vec::new();

    let dealer_score = scores[NUM_PLAYERS].0;

    for (current_index, &score) in scores.iter().enumerate() {
        if current_index != NUM_PLAYERS {
            if score.0 <= 21 && (score.0 > dealer_score || dealer_score > 21) {
                winner_index.push((current_index, false));
            } else if score.0 == dealer_score || (dealer_score > 21 && score.0 > 21) {
                equal_index.push((current_index, false));
            } else {
                loser_index.push((current_index, false));
            }

            if let Some(value) = score.1 {
                if value <= 21 && (value > dealer_score || dealer_score > 21) {
                    winner_index.push((current_index, true));
                } else if value == dealer_score || (dealer_score > 21 && value > 21) {
                    equal_index.push((current_index, true));
                } else {
                    loser_index.push((current_index, true));
                }
            }
        }
    }
    (winner_index, equal_index, loser_index)
}

#[cfg(test)]

mod tests {
    use crate::math::*;

    #[test]
    fn black_jack_test() {
        assert!(is_blackjack(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::King
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
        )));
        assert!(!is_blackjack(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Eight
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
        )));
        assert!(!is_blackjack(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::King
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ten
            }
        )))
    }

    #[test]
    fn is_splittable_test() {
        assert!(is_splittable(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::King
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::King
            }
        )));
        assert!(!is_splittable(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Eight
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
        )));
        assert!(!is_splittable(&vec!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::King
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::King
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::King
            }
        )))
    }

    #[test]
    fn compute_scores_test() {
        let card1 = Card {
            suit: Suit::Diamonds,
            rank: Rank::Two,
        };
        let card2 = Card {
            suit: Suit::Clubs,
            rank: Rank::Two,
        };
        let card3 = Card {
            suit: Suit::Hearts,
            rank: Rank::Two,
        };

        assert_eq!(
            [(0, None); 3],
            compute_scores(&vec!((vec!(), None), (vec!(), None)), &vec!())
        );
        assert_eq!(
            [(2, None); 3],
            compute_scores(
                &vec!((vec!(card1), None), (vec!(card2), None)),
                &vec!(card3)
            )
        );
    }

    #[test]
    fn compute_result_test() {
        let (win, equ, los) = compute_result([(0, Some(0)); 3]);
        assert!(win.is_empty() && equ.len() == 4 && los.is_empty());

        let (win, equ, los) = compute_result([(10, Some(10)), (0, Some(0)), (5, None)]);
        assert!(win.len() == 2 && equ.is_empty() && los.len() == 2);
    }
}
