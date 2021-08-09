use crate::card::*;
use crate::math;
use crate::player::*;

use math::*;
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
    player_types: &[Player; NUM_PLAYERS_AND_DEALER],
) {
    for (index, player_type) in player_types.iter().enumerate() {
        let mut score = compute_scores(player_hands, dealer_hand);

        let mut action = pick_action(&score, player_hands, dealer_hand, index, player_type);

        while action != PlayerAction::Stand {
            let new_card = pick_card(pack);
            if index < NUM_PLAYERS {
                player_hands[index].push(new_card);
            } else {
                dealer_hand.push(new_card);
            }

            score = compute_scores(player_hands, dealer_hand);

            action = pick_action(&score, player_hands, dealer_hand, index, player_type);
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

fn pick_action(
    scores: &[u32; NUM_PLAYERS_AND_DEALER],
    player_hands: &[Vec<Card>; NUM_PLAYERS],
    dealer_hand: &Vec<Card>,
    index: usize,
    player_type: &Player,
) -> PlayerAction {
    if scores[index] >= 21 {
        PlayerAction::Stand
    } else {
        loop {
            // while action is illegal, try again
            let action = match player_type {
                Player::Bot => bot_play(scores, player_hands, dealer_hand, index),
                Player::Human => human_play(scores, player_hands, dealer_hand, index),
            };
            match action {
                //make sure splitting is legal, the rest always is
                PlayerAction::Split => {
                    if index < NUM_PLAYERS && is_splittable(&player_hands[index]) {
                        return action;
                    } else if index == NUM_PLAYERS && is_splittable(&dealer_hand) {
                        return action;
                    } else {
                        ()
                    }
                }
                _ => return action,
            }
        }
    }
}
