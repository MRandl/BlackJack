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
use round::*;
use utils::*;

fn main() {
    print!("\n\n");

    let mut card_deck: Vec<Card> = Vec::new(); //will be filled with a few card packs
    let player_types = ask_for_player_types();
    let mut player_hands: Vec<(Vec<Card>, Option<Vec<Card>>)> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    let mut bank = Vec::new(); //contains the reserves of each player
    let mut bets = Vec::new(); //contains the current bets the players make
    init_game(
        &mut player_hands,
        &mut card_deck,
        &mut bank,
        player_types.len() - 1,
    );

    while is_playable(&bank) {
        play_round(
            &mut player_hands,
            &mut dealer_hand,
            &mut card_deck,
            &player_types,
            &mut bets,
            &mut bank,
        );

        let scores = compute_scores(&player_hands, &dealer_hand);
        display_hands_and_scores(&scores, &player_hands, &dealer_hand);

        println!();
        let bj_index = compute_blackjack_index(&player_hands, &dealer_hand);
        let (three_two_index, winner_index, equal_index, loser_index) =
            compute_result(scores, bj_index);
            
        update_bank(
            &three_two_index,
            &winner_index,
            &equal_index,
            &mut bank,
            &bets,
        );

        display_results(&three_two_index, &winner_index, &equal_index, &loser_index);
        display_bank(&bank);

        for mut hand in player_hands.iter_mut() {
            hand.0.clear();
            hand.1 = None
        }
        dealer_hand.clear();
        bets.clear();
        println!();
        wait_for_enter();
    }
}
