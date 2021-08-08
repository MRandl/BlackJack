#![allow(dead_code)]

pub enum Player {
    Bot,
    Human,
}

#[derive(PartialEq)]
pub enum PlayerAction {
    Hit,
    Stand,
    //Double,
}

pub fn human_play() -> PlayerAction {
    PlayerAction::Stand
}

pub fn bot_play() -> PlayerAction {
    PlayerAction::Stand
}