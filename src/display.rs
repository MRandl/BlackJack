use crate::card::Card;
use crate::math::is_blackjack;
use crate::player::PlayerType;

/// This method displays the hands and scores
/// of all players in a human-readable format.
///
/// # Arguments
///
/// * `scores` An array of scores obtained by
/// the players and the dealer. All players must
/// have at least one score, in the first part of
/// their corresponding tuples. Players may also
/// have another hand in case they "split", which
/// is why another optional score is available. As
/// the dealer cannot split, their optional hand is
/// assumed to be None.
/// * `player_hands` A vector of player hands, each
/// composed of either one or two Vectors of [Card]s,
/// the second one being present in case of a split.
/// This argument must have a length equal to the number
/// of non-dealer players.
/// * `dealer_hand` The hand of the dealer.
pub fn display_hands_and_scores(
    scores: &Vec<(u32, Option<u32>)>,
    player_hands: &Vec<(Vec<Card>, Option<Vec<Card>>)>,
    dealer_hand: &Vec<Card>,
) {
    for (index, score) in scores.into_iter().enumerate() {
        let player_hand: (&Vec<Card>, Option<&Vec<Card>>) = if index < player_hands.len() {
            let tup = player_hands.get(index).unwrap();
            (&tup.0, tup.1.as_ref())
        } else {
            (dealer_hand, None)
        };

        let vec = if player_hand.1.is_none() {
            vec![player_hand.0]
        } else {
            vec![player_hand.0, player_hand.1.unwrap()]
        };
        for (index_of_split, elem) in vec.iter().enumerate() {
            let mut stri = String::from("{");
            for card in *elem {
                stri.push_str(&format!("/ {} /", card));
            }
            stri.push('}');
            println!(
                "{} got hand {} with value {}!{}",
                player_name(index, player_hands.len()),
                stri,
                if index_of_split == 0 {
                    //if we are not in the split hand
                    score.0
                } else {
                    score.1.unwrap()
                },
                if is_blackjack(elem) {
                    " Blackjack!"
                } else {
                    ""
                }
            );
        }
    }
}

/// This method displays the results of
/// the current round in a human-readable format.
///
/// # Arguments
/// * `winner_index` A vector containing the index
/// of the players whose hand beat the hand of the
/// dealer. Along with that index is a boolean that is
/// true if and only if this hand
/// is a second hand for their respective player.
/// Note that a player index may appear twice if a player
/// split and both of their hands beat the dealer.
/// * `equal_index` Same as winner_index,
/// but for hands that have the same value as the
/// hand of the dealer, instead of winning.
/// * `loser_index` Same as winner_index,
/// but for hands that have less value as the
/// hand of the dealer, instead of winning.
pub fn display_results(
    winner_index: &Vec<(usize, bool)>,
    equal_index: &Vec<(usize, bool)>,
    loser_index: &Vec<(usize, bool)>,
) {
    display_result_vector(&winner_index, "winners");
    display_result_vector(&equal_index, "equalities");
    display_result_vector(&loser_index, "losers");
}

/// Internally used by [display_results]. Displays a
/// vector to the user.
fn display_result_vector(index: &Vec<(usize, bool)>, name: &str) {
    if index.len() == 0 {
        println!("There are no {} this turn!\n", name);
    } else {
        println!("The {} are : ", name);
        let mut stri = String::new();
        for (usi, bol) in index {
            if !bol {
                stri.push_str(&format!("Player {}, ", usi + 1));
            } else {
                stri.push_str(&format!("Second player {}, ", usi + 1));
            }
        }
        stri.pop();
        stri.pop();
        println!("{}\n", stri);
    }
}

/// This function computes the name of a player
/// based on their index.
///
/// Indices from 0 to the number
/// of non-dealer players are mapped to the String
/// `Player $i` and the other numbers map to "Dealer".
fn player_name(index: usize, num_players: usize) -> String {
    if index < num_players {
        format!("Player {}", index + 1)
    } else {
        String::from("Dealer")
    }
}

/// This method displays the current state of
/// the bank of the players (not including current bets) to the user.
pub fn display_bank(bank: &Vec<u32>) {
    let mut stri = String::from("Bank is : ");
    for elem in bank {
        stri.push_str(&format!("{}, ", elem));
    }
    stri.pop();
    stri.pop();
    println!("{}", stri);
}

/// Blocks until the user presses enter
pub fn wait_for_enter() {
    println!("Please press ENTER to continue.");
    let _ = std::io::stdin().read_line(&mut String::new());
}

pub fn ask_for_player_types() -> Vec<PlayerType> {
    let mut ret = Vec::new();
    println!(
        "Please enter the amount of players (not including the dealer) your game should have:"
    );
    let player_num = read_num();
    for index in 0..player_num {
        println!("Please enter type of player {}.", index + 1);
        ret.push(read_player_type());
    }
    ret.push(PlayerType::Bot); //dealer
    ret
}

fn read_num() -> u32 {
    loop {
        let mut s = String::new();
        std::io::stdin()
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
            Err(_) => println!("Not a number! Try again"),
        }
    }
}

fn read_player_type() -> PlayerType {
    loop {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }

        match s.as_str() {
            "Human" => return PlayerType::Human,
            "Bot" => return PlayerType::Bot,
            _ => println!("Not a valid type! Try again"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::display::*;

    #[test]
    fn call_displays_do_not_crash() {
        //kind of a weak test but i don't want to add boiler plate for DI
        display_results(&vec![], &vec![], &vec![]);
        display_bank(&vec![]);
        display_hands_and_scores(
            &vec![(0, None); 3],
            &vec![(vec![], None), (vec![], None)],
            &vec![],
        )
    }

    #[test]
    fn player_name_test() {
        assert_eq!("Dealer", &player_name(5, 5));
        assert_eq!("Player 1", &player_name(0, 1))
    }
}
