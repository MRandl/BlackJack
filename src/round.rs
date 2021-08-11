use crate::card::*;
use crate::math;
use crate::player::*;

use math::*;
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

pub fn play_round(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
    player_types: &Vec<Player>,
) {
    for (index, player_type) in player_types.iter().enumerate() {
        let mut score = compute_scores(player_hands, dealer_hand);

        let mut action = pick_action(&score, player_hands, dealer_hand, index, false, player_type);

        while action != PlayerAction::Stand {
            let new_card = pick_card(pack);
            if index < player_hands.len() {
                player_hands[index].0.push(new_card);
            } else {
                dealer_hand.push(new_card);
            }

            score = compute_scores(player_hands, dealer_hand);

            action = pick_action(&score, player_hands, dealer_hand, index, false, player_type);
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
    scores: &[(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
    index: usize,
    is_second: bool,
    player_type: &Player,
) -> PlayerAction {
    
    if (!is_second && scores[index].0 >= 21) || (is_second && scores[index].1.unwrap_or(22) >= 21) {
        PlayerAction::Stand
    } else {
        loop {
            // while action is illegal, try again
            let action = match player_type {
                Player::Bot => bot_play(scores, player_hands, dealer_hand, is_second, index),
                Player::Human => human_play(scores, player_hands, dealer_hand, is_second, index),
            };
            match action {
                //make sure splitting is legal, the rest always is
                PlayerAction::Split => {
                    let hand = if is_second {&player_hands[index].1.as_ref().unwrap()} else {&player_hands[index].0};
                    if index < NUM_PLAYERS && is_splittable(hand) {
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
