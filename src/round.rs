use crate::bjmath::*;
use crate::card::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn play_round(
  player_hands: &mut Vec<Vec<Card>>,
  dealer_hand: &mut Vec<Card>,
  pack: &mut Vec<Card>,
) {
  //todo: implement adding cards one by one for each player with a tiny recap every time
}

pub fn init_game(
  player_hands: &mut Vec<Vec<Card>>,
  dealer_hand: &mut Vec<Card>,
  pack: &mut Vec<Card>,
) {
  for _ in 0..NUM_PLAYERS {
      player_hands.push(Vec::new());
  }

  for hand in player_hands {
      hand.push(pick_card(pack));
      hand.push(pick_card(pack));
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

pub fn compute_scores(
  player_hands: &Vec<Vec<Card>>,
  dealer_hand: &Vec<Card>,
) -> [u32; (NUM_PLAYERS + 1) as usize] {
  let mut scores = [0; (NUM_PLAYERS + 1) as usize];
  let mut player = 0;
  for hand in player_hands {
      scores[player] = hand_value(hand);

      let mut stri: String = String::from("{");
      for elem in hand {
          stri.push_str(&format!("/ {} /", elem));
      }
      stri.push('}');
      println!(
          "Player {} got hand {} with value {}!",
          player + 1,
          stri,
          scores[player]
      );
      player += 1;
  }
  scores[player] = hand_value(dealer_hand);

  let mut stri: String = String::from("{");
  for elem in dealer_hand {
      stri.push_str(&format!("/ {} /", elem));
  }
  stri.push('}');
  println!(
      "Dealer got hand {} with value {}!",
      stri,
      scores[player]
  );
  scores
}