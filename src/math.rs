use crate::card::*;

/// The number of packs to use when shuffling the cards.
pub const NUM_PACKS: usize = 4;

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
) -> Vec<(u32, Option<u32>)> {
    let mut scores = Vec::with_capacity(player_hands.len() + 1);
    for hand in player_hands {
        scores.push((hand_value(&hand.0), hand.1.as_ref().map(|v| hand_value(&v))));
    }
    scores.push((hand_value(dealer_hand), None));
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
    scores: Vec<(u32, Option<u32>)>,
    blackjacks: Vec<(bool, Option<bool>)>,
) -> (
    Vec<(usize, bool)>,
    Vec<(usize, bool)>,
    Vec<(usize, bool)>,
    Vec<(usize, bool)>,
) {
    let mut three_two_index: Vec<(usize, bool)> = Vec::new();
    let mut winner_index: Vec<(usize, bool)> = Vec::new();
    let mut equal_index: Vec<(usize, bool)> = Vec::new();
    let mut loser_index: Vec<(usize, bool)> = Vec::new();
    let num_players = scores.len() - 1;

    let dealer_has_blackjack = blackjacks.last() == Some(&(true, None));
    let dealer_score = scores[num_players].0;

    for (index, score) in scores.iter().enumerate() {
        if index != num_players {
            if dealer_has_blackjack {
                if blackjacks[index].0 {
                    equal_index.push((index, false));
                } else {
                    loser_index.push((index, false));
                }
                if let Some(bj) = blackjacks[index].1 {
                    if bj {
                        equal_index.push((index, true));
                    } else {
                        loser_index.push((index, true));
                    }
                }
            } else {
                // dealer does not have blackjack

                if blackjacks[index].0 {
                    three_two_index.push((index, false));
                } else if score.0 <= 21 && (score.0 > dealer_score || dealer_score > 21) {
                    winner_index.push((index, false));
                } else if score.0 <= 21 && score.0 == dealer_score {
                    equal_index.push((index, false));
                } else {
                    loser_index.push((index, false));
                }

                if let Some(value) = score.1 {
                    if blackjacks[index].1.unwrap() {
                        three_two_index.push((index, true));
                    } else if value <= 21 && (value > dealer_score || dealer_score > 21) {
                        winner_index.push((index, true));
                    } else if value <= 21 && value == dealer_score {
                        equal_index.push((index, true));
                    } else {
                        loser_index.push((index, true));
                    }
                }
            }
        }
    }

    (three_two_index, winner_index, equal_index, loser_index)
}

pub fn compute_blackjack_index(
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
) -> Vec<(bool, Option<bool>)> {
    let mut bj = Vec::with_capacity(player_hands.len() + 1);
    for hand in player_hands {
        bj.push((
            is_blackjack(&hand.0),
            hand.1.as_ref().map(|e| is_blackjack(&e)),
        ));
    }
    bj.push((is_blackjack(dealer_hand), None));
    bj
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
            vec![(0, None); 3],
            compute_scores(&vec!((vec!(), None), (vec!(), None)), &vec!())
        );
        assert_eq!(
            vec![(2, None); 3],
            compute_scores(
                &vec!((vec!(card1), None), (vec!(card2), None)),
                &vec!(card3)
            )
        );
    }

    #[test]
    fn compute_result_test() {
        let (thre, win, equ, los) = compute_result(vec![(0, None); 5], Vec::new());
        assert!(thre.is_empty() && win.is_empty() && equ.len() == 4 && los.is_empty());

        let (thre, win, equ, los) =
            compute_result(vec![(10, Some(10)), (0, Some(0)), (5, None)], Vec::new());
        assert!(thre.is_empty() && win.len() == 2 && equ.is_empty() && los.len() == 2);
    }
}
