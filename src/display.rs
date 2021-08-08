use crate::card::Card;
use crate::math::{is_blackjack, NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};

pub fn display_hand_and_scores(
    scores: &[u32; NUM_PLAYERS_AND_DEALER],
    player_hands: &[Vec<Card>; NUM_PLAYERS],
    dealer_hand: &Vec<Card>,
) {
    let mut current_index: usize = 0;
    for score in scores[0..(NUM_PLAYERS)].into_iter() {
        let player_hand = player_hands.get(current_index).unwrap();
        let mut stri = String::from("{");
        for card in player_hand {
            stri.push_str(&format!("/ {} /", card));
        }
        stri.push('}');
        println!(
            "Player {} got hand {} with value {}! {}",
            current_index + 1,
            stri,
            score,
            if is_blackjack(player_hand) {
                "Blackjack!"
            } else {
                ""
            }
        );
        current_index += 1;
    }

    let mut stri = String::from("{");
    for card in dealer_hand {
        stri.push_str(&format!("/ {} /", card));
    }
    stri.push('}');
    println!(
        "Dealer got hand {} with value {}!",
        stri, scores[NUM_PLAYERS]
    );
}

pub fn display_winner(mut winner_index: Vec<usize>, winner_score: u32) {
    if winner_index.len() == 1 {
        println!(
            "\nPlayer {} wins with score {}!",
            winner_index.pop().unwrap() + 1,
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
