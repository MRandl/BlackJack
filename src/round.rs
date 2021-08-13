use crate::card::*;
use crate::math::*;
use crate::player::*;
use crate::utils::pick_card;

pub fn play_round(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &mut Vec<Card>,
    pack: &mut Vec<Card>,
    player_types: &Vec<Player>,
    bets: &mut Vec<u32>,
    bank: &mut Vec<u32>,
) {
    for (index, typ) in player_types.iter().enumerate() {
        if index < NUM_PLAYERS {
            let mut try_bet = pick_bet(index, typ, bank[index]);
            while try_bet > bank[index] {
                try_bet = pick_bet(index, typ, bank[index]);
            }
            bets.push(try_bet);
            bank[index] -= try_bet;
        }
    }

    for hand in player_hands.iter_mut() {
        hand.0.push(pick_card(pack));
        hand.0.push(pick_card(pack));
    }
    dealer_hand.push(pick_card(pack));

    for (index, player_type) in player_types.iter().enumerate() {
        play_turn(
            player_hands,
            dealer_hand,
            pack,
            index,
            player_type,
            false,
            bets,
            bank,
        );

        if index < NUM_PLAYERS && player_hands[index].1.is_some() {
            play_turn(
                player_hands,
                dealer_hand,
                pack,
                index,
                player_type,
                true,
                bets,
                bank,
            )
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
    bets: &mut Vec<u32>,
    bank: &mut Vec<u32>,
) {
    let mut score = compute_scores(player_hands, dealer_hand);
    let mut action = pick_action(
        &score,
        player_hands,
        dealer_hand,
        index,
        is_second_turn,
        player_type,
        bets,
        bank,
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
                    bets,
                    bank,
                );
            }
            PlayerAction::Split => {
                let hand = player_hands.get_mut(index).unwrap();
                let card = hand.0.pop().unwrap();
                hand.1 = Some(vec![card]);
                bank[index] -= bets[index];

                score = compute_scores(player_hands, dealer_hand);
                action = pick_action(
                    &score,
                    player_hands,
                    dealer_hand,
                    index,
                    false,
                    player_type,
                    bets,
                    bank,
                );
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
    bets: &mut Vec<u32>,
    bank: &mut Vec<u32>,
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
                        && bank[index] >= bets[index]
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

fn pick_bet(index: usize, player_type: &Player, available : u32) -> u32 {
    match player_type {
        Player::Bot => available >> 1,
        Player::Human => human_bet(index, available)
    }
}
