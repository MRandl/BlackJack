//! A CLI blackjack game.

mod bjmath;
mod card;
mod round;

use bjmath::*;
use card::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use round::*;

fn main() {
    print!("\n\n");

    let mut multi_card_pack: Vec<Card> = Vec::new();
    for _ in 0..NUM_PACKS {
        multi_card_pack.extend(Card::card_pack());
    }
    multi_card_pack.shuffle(&mut thread_rng());

    let mut player_hands: Vec<Vec<Card>> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    init_game(&mut player_hands, &mut dealer_hand, &mut multi_card_pack);
    play_round(&mut player_hands, &mut dealer_hand, &mut multi_card_pack);

    let scores = compute_scores(&player_hands, &dealer_hand);

    let mut winner_index: Vec<u32> = Vec::new();
    let mut winner_score = 0;

    let mut current_index = 0;
    for score in scores {
        if score == winner_score {
            winner_index.push(current_index);
        } else if score > winner_score && score <= 21 {
            winner_score = score;
            winner_index.clear();
            winner_index.push(current_index)
        }
        current_index += 1;
    }
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
