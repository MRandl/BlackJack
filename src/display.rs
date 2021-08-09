use crate::card::Card;
use crate::math::{is_blackjack, NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};

pub fn display_hand_and_scores(
    scores: &[u32; NUM_PLAYERS_AND_DEALER],
    player_hands: &Vec<Vec<Card>>,
    dealer_hand: &Vec<Card>,
) {
    for (index, score) in scores.into_iter().enumerate() {
        let player_hand = if index < NUM_PLAYERS {
            player_hands.get(index).unwrap()
        } else {
            dealer_hand
        };

        let mut stri = String::from("{");
        for card in player_hand {
            stri.push_str(&format!("/ {} /", card));
        }
        stri.push('}');
        println!(
            "{} got hand {} with value {}!{}",
            player_name(index),
            stri,
            score,
            if is_blackjack(player_hand) {
                " Blackjack!"
            } else {
                ""
            }
        );
    }
}

pub fn display_winner(mut winner_index: Vec<usize>, winner_score: u32) {
    if winner_index.len() == 1 {
        let index_of_winner = winner_index.pop().unwrap();
        println!(
            "\n{} wins with score {}!",
            player_name(index_of_winner),
            winner_score
        )
    } else {
        let mut stri = String::new();
        for elem in winner_index {
            stri.push_str(&format!("{}, ", elem + 1))
        }
        stri.pop();
        stri.pop();
        println!("Equality between the players : {}!", stri)
    }
}

fn player_name(index: usize) -> String {
    if index < NUM_PLAYERS {
        format!("Player {}", index + 1)
    } else {
        String::from("Dealer")
    }
}
