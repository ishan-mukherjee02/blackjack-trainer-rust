mod blackjack;
mod blackjackbasicstrategy;

use crate::blackjackbasicstrategy::BlackjackBasicStrategy;
use crate::blackjack::Blackjack;
use std::io::{self, Write};


pub struct BlackjackUI {
    bj: Blackjack,
}

impl BlackjackUI {
    // Constructs a Blackjack game with $1,000 in player bankroll
    pub fn new() -> Self {
        BlackjackUI {
            bj: Blackjack::new(),
        }
    }

    /// Returns a valid numerical bet obtained from the player
    fn get_valid_bet(&mut self) -> f64 {
        println!("What is your bet: ");
        let mut input = String::new();

        loop {
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(money) if money > 0.0 => return money,
                _ => println!("Please enter a valid bet greater than 0."),
            }
        }
    }

    /// Plays a single hand of blackjack
    pub fn play_hand(&mut self) {
        self.play_players_hand();
        self.bj.play_dealers_hand();
        self.display_result();
    }

    /// Plays blackjack hands until the user chooses to quit
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

    /// Allows the player to hit until it is no longer possible or until the player chooses to stand
    fn play_players_hand(&mut self) {
        let mut response = String::new();

        while self.bj.can_hit() {
            println!("Do you want to hit, stand, or double?");
            io::stdout().flush().unwrap();
            response.clear();
            io::stdin().read_line(&mut response).unwrap();

            match response.trim().to_lowercase().as_str() {
                "hit" => {
                    self.bj.hit();
                    if self.bj.get_players_hand().expect("Empty").get_value() > 21 {
                        println!("You are bust.");
                    }
                }
                "double" => {
                    if self.bj.get_players_hand().expect("Empty").get_value() > 21 {
                        println!("You are bust.");
                    }
                    println!("You have: {}", self.bj.get_players_hand().expect("Empty").to_string());
                    break;
                }
                "stand" => break,
                _ => println!("Invalid option, please type 'hit', 'stand', or 'double'."),
            }

            println!("You have: {}", self.bj.get_players_hand().expect("Empty").to_string());
        }
    }

    /// Displays the result of the hand (push, player win, player blackjack, or loss)
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

    /// Converts a string to a number or returns -1 if input is not numeric
    fn string_to_number(input: &str) -> f64 {
        input.trim().parse().unwrap_or(-1.0)
    }
}

/**
 * A text based user interface that allows the user to play a game of blackjack.
 */
fn main() {
    
}
