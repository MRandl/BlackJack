use crate::math::{NUM_PACKS, NUM_PLAYERS};
use crate::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn init_game(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
) {
    for _ in 0..NUM_PACKS {
        pack.extend(Card::card_pack());
    }
    pack.shuffle(&mut thread_rng());

    for _ in 0..NUM_PLAYERS {
        player_hands.push((Vec::new(), None))
    }

    for hand in player_hands {
        hand.0.push(pick_card(pack));
        hand.0.push(pick_card(pack));
    }

    dealer_hand.push(pick_card(pack));
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
