mod blackjack_logic;
mod card;

use blackjack_logic::*;
use card::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

const NUM_PACKS: u32 = 4;
const NUM_PLAYERS: u32 = 2; //includes the dealer

fn main() {
    let mut mult_card_pack: Vec<Card> = Vec::new();

    for _ in 0..NUM_PACKS {
        mult_card_pack.extend(Card::card_pack());
    }
    mult_card_pack.shuffle(&mut thread_rng());

    let mut player_hands: Vec<Vec<Card>> = Vec::new();
    for _ in 0..NUM_PLAYERS {
        player_hands.push(Vec::new());
    }

    for hand in &mut player_hands {
        hand.push(pick_card(&mut mult_card_pack));
        hand.push(pick_card(&mut mult_card_pack));
    }

    let mut player = 1;
    for hand in &player_hands {
        let mut stri: String = String::from("{");
        for elem in hand {
            stri.push_str(&format!("/ {} /", elem));
        }
        stri.push('}');
        println!(
            "Player {} got hand {} with value {}!",
            player,
            stri,
            hand_value(hand)
        );
        player += 1;
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
