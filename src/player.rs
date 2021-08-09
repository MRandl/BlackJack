#![allow(dead_code)]
use crate::card::Card;
use crate::display::display_hand_and_scores;
use crate::math::{NUM_PLAYERS, NUM_PLAYERS_AND_DEALER};
use std::io::stdin;
pub enum Player {
    Bot,
    Human,
}

#[derive(PartialEq)]
pub enum PlayerAction {
    Hit,
    Stand,
    //Double,
    Split,
}

pub fn human_play(
    scores: &[u32; NUM_PLAYERS_AND_DEALER],
    player_hands: &Vec<Vec<Card>>,
    dealer_hand: &Vec<Card>,
    index: usize,
) -> PlayerAction {
    println!("------------------------");
    println!("You are player {}.", index + 1);
    println!("The board is currently in the following state : \n");
    display_hand_and_scores(scores, player_hands, dealer_hand);
    println!("\nPlease enter your move : Hit or Stand");
    loop {
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }

        let _ = match s.as_str() {
            "Hit" => return PlayerAction::Hit,
            "Stand" => return PlayerAction::Stand,
            "Split" => return PlayerAction::Split,
            _ => println!("Unrecognized move. Please input again:"),
        };
    }
}

pub fn bot_play(
    scores: &[u32; NUM_PLAYERS_AND_DEALER],
    _player_hands: &Vec<Vec<Card>>,
    _dealer_hand: &Vec<Card>,
    index: usize,
) -> PlayerAction {
    if scores[index] < 15 {
        PlayerAction::Hit
    } else {
        PlayerAction::Stand
    }
}
