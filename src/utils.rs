use crate::math::{NUM_PACKS, NUM_PLAYERS};
use crate::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn init_game(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    pack: &mut Vec<Card>,
    bank: &mut Vec<u32>,
) {
    for _ in 0..NUM_PACKS {
        pack.extend(Card::card_pack());
    }
    pack.shuffle(&mut thread_rng());

    for _ in 0..NUM_PLAYERS {
        player_hands.push((Vec::new(), None));
        bank.push(300);
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

pub fn update_bank(
    win: &Vec<(usize, bool)>,
    equ: &Vec<(usize, bool)>,
    bank: &mut Vec<u32>,
    bets: &Vec<u32>,
) {
    for &(index, _) in win {
        bank[index] += 2 * bets[index];
    }
    for &(index, _) in equ {
        bank[index] += bets[index];
    }
}
