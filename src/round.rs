use crate::card::*;
use crate::math;
use crate::player::*;

use math::{NUM_PACKS, NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};
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
    _player_hands: &[Vec<Card>; NUM_PLAYERS],
    _dealer_hand: &mut Vec<Card>,
    _pack: &mut Vec<Card>,
    players: &[&Player; NUM_PLAYERS_AND_DEALER],
) {
    for (index, player) in players.iter().enumerate() {
        
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
