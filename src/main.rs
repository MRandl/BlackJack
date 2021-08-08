//! A CLI blackjack game.

mod bjmath;
mod card;
mod display;
mod round;

use bjmath::*;
use card::*;
use display::*;
use round::*;

fn main() {
    print!("\n\n");

    let mut multi_card_pack: Vec<Card> = Vec::new();
    let mut player_hands: Vec<Vec<Card>> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    init_game(&mut player_hands, &mut dealer_hand, &mut multi_card_pack);
    play_round(&mut player_hands, &mut dealer_hand, &mut multi_card_pack);

    let scores = compute_scores(&player_hands, &dealer_hand);
    display_hand_and_scores(&scores, &player_hands, &dealer_hand);
    
    let (winner_index, winner_score) = compute_winner(scores);
    display_winner(winner_index, winner_score);
}
