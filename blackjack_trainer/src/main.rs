mod blackjack;
mod blackjackbasicstrategy;

use crate::blackjack::Blackjack;
use std::io::{self, Write};

pub struct BlackjackUI {
    bj: Blackjack,
}

impl BlackjackUI {
    // Constructs a Blackjack game
    pub fn new() -> Self {
        BlackjackUI {
            bj: Blackjack::new(),
        }
    }

    // Plays a single hand of blackjack
    pub fn play_hand(&mut self) {
        self.bj.deal_cards();
        self.play_players_hand();
        self.bj.play_dealers_hand();
        self.display_result();
    }

    // Plays blackjack hands until the user chooses to quit
    pub fn play_hands_until_quit(&mut self) {
        let mut input = String::new();

        loop {
            self.play_hand();
            println!("Keep playing? (yes/no): ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "yes" {
                break;
            }
        }

        println!("Thanks for playing");
    }

    // Allows the player to hit until it is no longer possible or until the player chooses to stand
    fn play_players_hand(&mut self) {
        println!("You have: {}", self.bj.get_players_hand().expect("nothing").to_string());
        println!("Dealer has: {}", self.bj.get_dealers_hand().expect("nothing").to_string());

        let mut response = String::new();

        let bj = &mut self.bj;

        while bj.can_hit() {
            println!("Do you want to hit or stand?");
            io::stdout().flush().unwrap();
            response.clear();
            io::stdin().read_line(&mut response).unwrap();
            
            match response.trim().to_lowercase().as_str() {
                "hit" => {
                    bj.hit();
                    if bj.get_players_hand().expect("nothing").get_value() > 21 {
                        println!("You now have: {}", bj.get_players_hand().expect("nothing").to_string());
                        println!("You are bust.");
                        continue;
                    }
                }
                "stand" => break,
                _ => println!("Invalid option, please type 'hit', 'stand', or 'double'."),
            }
    
            println!(
                "You have: {}",
                bj.get_players_hand().expect("nothing").to_string()
            );
        }
    }

    // Displays the result of the hand (push, player win, player blackjack, or loss)
    fn display_result(&mut self) {
        if self.bj.get_dealers_hand().expect("Empty").is_blackjack() && self.bj.get_players_hand().expect("Empty").is_blackjack() {
            println!("Y'all both got blackjack, it's a push.");
        } else if self.bj.get_players_hand().expect("Empty").is_blackjack() {
            println!("YOU GOT BLACKJACK!");
        } else if self.bj.is_player_win() {
            println!("Player win.");
        } else if self.bj.is_push() {
            println!("Push.");
        } else {
            println!("Player loss.");
        }
    }
}

/**
 * A text based user interface that allows the user to play a game of blackjack.
 */
fn main() {
    let mut game = BlackjackUI::new();
    game.play_hands_until_quit();
}
