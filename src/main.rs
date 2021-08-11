//! A CLI blackjack game.

mod card;
mod display;
mod math;
mod player;
mod round;
mod utils;

use card::*;
use display::*;
use math::*;
use player::*;
use round::*;
use utils::*;

fn main() {
    print!("\n\n");

    let mut multi_card_pack: Vec<Card> = Vec::new(); //will be filled with a few card packs
    let player_types = vec![Player::Human, Player::Bot, Player::Bot];
    let mut player_hands: Vec<(Vec<Card>, Option<Vec<Card>>)> = Vec::new();
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
    let (winner_index, equal_index, loser_index) = compute_result(scores); //todo remove when implementing bets
    display_results(&winner_index, &equal_index, &loser_index);
}
