use crate::card::Card;
use crate::display::display_hands_and_scores;
use crate::math::NUM_PLAYERS_AND_DEALER;
use std::io::stdin;

/// The enum for the player types. 
/// A player can be human or a bot.
pub enum Player {
    Bot,
    Human,
}

/// The enum for player actions. Currently supported
/// actions are Hit, Stand and Split.
#[derive(PartialEq)]
pub enum PlayerAction {
    Hit,
    Stand,
    //Double,
    Split,
}

/// This method asks the player at a given index for 
/// his bet at this round, by reading his answer in the terminal.
pub fn human_bet(index: usize, available: u32) -> u32 {
    println!("------------------------");
    println!("You are player {}.", index + 1);
    println!("You have {} units of money available.", available);
    println!("\nPlease enter your bet amount : ");
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

        match s.parse::<u32>() {
            Ok(va) => return va,
            Err(_) => (),
        }
    }
}

/// This method asks a human player for the action
/// by reading his answer from the terminal.
pub fn human_play(
    scores: &[(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
    is_second: bool,
    index: usize,
) -> PlayerAction {
    println!("------------------------");
    println!(
        "You are player {}{}.",
        index + 1,
        if is_second { ", second hand" } else { "" }
    );
    println!("The board is currently in the following state : \n");
    display_hands_and_scores(scores, player_hands, dealer_hand);
    println!("\nPlease enter your move : Hit, Stand or Split.");
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

/// This method implements the decision algorithm used by bots to play
/// It will be the focus of future work on AI improvements.
pub fn bot_play(
    scores: &[(u32, Option<u32>); NUM_PLAYERS_AND_DEALER],
    _player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    _dealer_hand: &Vec<Card>,
    is_second: bool,
    index: usize,
) -> PlayerAction {
    if (!is_second && scores[index].0 < 15) || (is_second && scores[index].1.unwrap() < 15) {
        PlayerAction::Hit
    } else {
        PlayerAction::Stand
    }
}
