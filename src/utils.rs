use crate::math::{NUM_PACKS, NUM_PLAYERS};
use crate::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Initializes the card deck to a few card packs, shuffles it
/// and gives a starting fund to the bank of each player.
pub fn init_game(
    player_hands: &mut Vec<(Vec<Card>, Option<Vec<Card>>)>,
    pack: &mut Vec<Card>,
    bank: &mut Vec<u32>,
) {
    for _ in 0..NUM_PACKS {
        pack.extend(Card::card_pack());
    }
    pack.shuffle(&mut thread_rng());

    for _ in 0..NUM_PLAYERS {
        player_hands.push((Vec::new(), None));
        bank.push(300);
    }
}

/// Picks a card from the deck. If the deck is empty,
/// replaces it with a fresh shuffled deck and picks a card
/// from it.
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

/// Adds rewards to the bank of players
/// that won or reached equality, according to their bets.
pub fn update_bank(
    win: &Vec<(usize, bool)>,
    equ: &Vec<(usize, bool)>,
    bank: &mut Vec<u32>,
    bets: &Vec<u32>,
) {
    for &(index, _) in win {
        bank[index] += 2 * bets[index];
    }
    for &(index, _) in equ {
        bank[index] += bets[index];
    }
}

/// Checks whether all players have some funds to keep playing.
pub fn is_playable(bank: &Vec<u32>) -> bool {
    for &b in bank {
        if b == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    #[test]
    fn init_test() {
        let mut hands = vec![];
        let mut pack = vec![];
        let mut bank = vec![];
        init_game(&mut hands, &mut pack, &mut bank);
        assert_eq!(NUM_PLAYERS, hands.len());
        assert_eq!(4 * 52, pack.len());
        assert_eq!(vec!(300, 300), bank);
    }

    #[test]
    fn pick_test() {
        let mut pack = vec![];
        let _ = pick_card(&mut pack);
        assert!(pack.len() == 4 * 52 - 1);
        let _ = pick_card(&mut pack);
        assert!(pack.len() == 4 * 52 - 2);
    }

    #[test]
    fn update_bank_test() {
        let mut bank = vec![300, 300];
        let bets = vec![300, 300];
        let win = vec![(0, false)];
        let equ = vec![(1, false)];
        update_bank(&win, &equ, &mut bank, &bets);

        assert_eq!(vec!(900, 600), bank);
    }

    #[test]
    fn is_playable_test() {
        let bank1 = vec![300, 300, 22];
        let bank2 = vec![300, 0, 4];

        assert!(is_playable(&bank1) && !is_playable(&bank2));
    }
}
