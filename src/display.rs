use crate::card::Card;
use crate::math::{is_blackjack, NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};

pub fn display_hands_and_scores(
    scores: &[(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
) {
    for (index, score) in scores.into_iter().enumerate() {
        let player_hand: (&Vec<Card>, Option<&Vec<Card>>) = if index < NUM_PLAYERS {
            let tup = player_hands.get(index).unwrap();
            (&tup.0, tup.1.as_ref())
        } else {
            (dealer_hand, None)
        };

        let vec = if player_hand.1.is_none() {
            vec![player_hand.0]
        } else {
            vec![player_hand.0, player_hand.1.unwrap()]
        };
        for (index_of_split, elem) in vec.iter().enumerate() {
            let mut stri = String::from("{");
            for card in *elem {
                stri.push_str(&format!("/ {} /", card));
            }
            stri.push('}');
            println!(
                "{} got hand {} with value {}!{}",
                player_name(index),
                stri,
                if index_of_split == 0 {
                    //if we are not in the split hand
                    score.0
                } else {
                    score.1.unwrap()
                },
                if is_blackjack(elem) {
                    " Blackjack!"
                } else {
                    ""
                }
            );
        }
    }
}

pub fn display_results(
    winner_index: &Vec<(usize, bool)>,
    equal_index: &Vec<(usize, bool)>,
    loser_index: &Vec<(usize, bool)>,
) {
    display_result_vector(&winner_index, "winners");
    display_result_vector(&equal_index, "equalities");
    display_result_vector(&loser_index, "losers");
}

fn display_result_vector(index: &Vec<(usize, bool)>, name: &str) {
    if index.len() == 0 {
        println!("There are no {} this turn!\n", name);
    } else {
        println!("The {} are : ", name);
        let mut stri = String::new();
        for (usi, bol) in index {
            if !bol {
                stri.push_str(&format!("Player {}, ", usi + 1));
            } else {
                stri.push_str(&format!("Second player {}, ", usi + 1));
            }
        }
        stri.pop();
        stri.pop();
        println!("{}\n", stri);
    }
}

fn player_name(index: usize) -> String {
    if index < NUM_PLAYERS {
        format!("Player {}", index + 1)
    } else {
        String::from("Dealer")
    }
}

pub fn display_bank(bank: &Vec<u32>) {
    let mut stri = String::from("Bank is : ");
    for elem in bank {
        stri.push_str(&format!("{}, ", elem));
    }
    stri.pop();
    stri.pop();
    println!("{}", stri);
}
