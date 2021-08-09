use crate::card::*;
use crate::math;
use crate::player::*;

use math::{compute_scores, NUM_PACKS, NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn init_game(
    player_hands: &mut [Vec<Card>; NUM_PLAYERS],
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
) {
    for _ in 0..NUM_PACKS {
        pack.extend(Card::card_pack());
    }
    pack.shuffle(&mut thread_rng());

    for hand in player_hands {
        hand.push(pick_card(pack));
        hand.push(pick_card(pack));
    }

    dealer_hand.push(pick_card(pack));
}

pub fn play_round(
    player_hands: &mut [Vec<Card>; NUM_PLAYERS],
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
    player_types: &[&Player; NUM_PLAYERS_AND_DEALER],
) {
    for (index, player_type) in player_types.iter().enumerate() {
        let mut score = compute_scores(player_hands, dealer_hand);

        let mut action = match player_type {
            Player::Bot => bot_play(&score, player_hands, dealer_hand, index),
            Player::Human => human_play(&score, player_hands, dealer_hand, index),
        };

        while action != PlayerAction::Stand {
            let new_card = pick_card(pack);
            if index < NUM_PLAYERS {
                player_hands[index].push(new_card);
            } else {
                dealer_hand.push(new_card);
            }

            score = compute_scores(player_hands, dealer_hand);

            action = match player_type {
                Player::Bot => bot_play(&score, player_hands, dealer_hand, index),
                Player::Human => human_play(&score, player_hands, dealer_hand, index),
            };
        }
    }
}

pub fn pick_card(pack: &mut Vec<Card>) -> Card {
    match pack.pop() {
        Some(card) => card,
        None => {
            //reshuffle
            for _ in 0..NUM_PACKS {
                pack.extend(Card::card_pack());
            }
            pack.shuffle(&mut thread_rng());
            pack.pop().unwrap()
        }
    }
}
