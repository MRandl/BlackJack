//! A CLI blackjack game.

mod card;
mod display;
mod math;
mod player;
mod round;

use card::*;
use display::*;
use math::*;
use player::*;
use round::*;

const EMPTY_HAND: Vec<Card> = Vec::new();

fn main() {
    print!("\n\n");

    let mut multi_card_pack: Vec<Card> = Vec::new();
    let player_types = [Player::Human, Player::Bot, Player::Bot];
    let mut player_hands = [EMPTY_HAND; NUM_PLAYERS];
    let mut dealer_hand: Vec<Card> = Vec::new();

    init_game(&mut player_hands, &mut dealer_hand, &mut multi_card_pack);
    play_round(
        &mut player_hands,
        &mut dealer_hand,
        &mut multi_card_pack,
        &player_types,
    );

    let scores = compute_scores(&player_hands, &dealer_hand);
    display_hand_and_scores(&scores, &player_hands, &dealer_hand);

    println!("");
    let (winner_index, winner_score) = compute_winner(scores);
    display_winner(winner_index, winner_score);
}
