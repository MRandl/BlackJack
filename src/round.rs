use crate::card::*;
use crate::math::*;
use crate::player::*;
use crate::utils::pick_card;

pub fn play_round(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
    player_types: &Vec<Player>,
) {
    for (index, player_type) in player_types.iter().enumerate() {
        play_turn(player_hands, dealer_hand, pack, index, player_type, false);

        if index < NUM_PLAYERS && player_hands[index].1.is_some() {
            play_turn(player_hands, dealer_hand, pack, index, player_type, true)
        }
    }
}

fn play_turn(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
    index: usize,
    player_type: &Player,
    is_second_turn: bool,
) {
    let mut score = compute_scores(player_hands, dealer_hand);
    let mut action = pick_action(
        &score,
        player_hands,
        dealer_hand,
        index,
        is_second_turn,
        player_type,
    );
    while action != PlayerAction::Stand {
        match action {
            PlayerAction::Hit => {
                let new_card = pick_card(pack);
                if index < player_hands.len() {
                    {
                        if is_second_turn {
                            player_hands[index].1.as_mut().unwrap()
                        } else {
                            &mut player_hands[index].0
                        }
                    }
                    .push(new_card);
                } else {
                    dealer_hand.push(new_card);
                }

                score = compute_scores(player_hands, dealer_hand);

                action = pick_action(
                    &score,
                    player_hands,
                    dealer_hand,
                    index,
                    is_second_turn,
                    player_type,
                );
            }
            PlayerAction::Split => {
                let hand = player_hands.get_mut(index).unwrap();
                let card = hand.0.pop().unwrap();
                hand.1 = Some(vec![card]);

                score = compute_scores(player_hands, dealer_hand);
                action = pick_action(&score, player_hands, dealer_hand, index, false, player_type);
            }
            PlayerAction::Stand => unreachable!(),
        }
    }
}


fn pick_action(
    scores: &[(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
    index: usize,
    is_second: bool,
    player_type: &Player,
) -> PlayerAction {
    if (!is_second && scores[index].0 >= 21) || (is_second && scores[index].1.unwrap() >= 21) {
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
                    if player_hands[index].1.is_none()
                        && index < NUM_PLAYERS
                        && is_splittable(&player_hands[index].0)
                    {
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

