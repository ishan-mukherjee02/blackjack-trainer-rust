mod blackjack;
mod blackjackbasicstrategy;

use crate::blackjackbasicstrategy::BlackjackBasicStrategy;
use std::io::{self, Write};

fn main() {
    let from_keyboard = io::stdin();
    let hand1 = BlackjackBasicStrategy::new();

    println!("Welcome to the Blackjack Trainer!");

    loop {
        print!("\nEnter how many cards you have (-1 to quit the simulation): ");
        io::stdout().flush().unwrap(); // Flush the output to ensure it prints immediately

        let mut cards_input = String::new();
        from_keyboard.read_line(&mut cards_input).expect("Failed to read input");
        let cards: i32 = cards_input.trim().parse().expect("Please enter a valid number");

        if cards == -1 {
            break;
        }

        let cards = cards as usize;
        let mut player_hand = Vec::with_capacity(cards);

        for i in 0..cards {
            print!("Enter current hand (Enter face cards as 10s, and aces as 1s): ");
            io::stdout().flush().unwrap();
            let mut hand_input = String::new();
            from_keyboard.read_line(&mut hand_input).expect("Failed to read input");
            let hand: i32 = hand_input.trim().parse().expect("Please enter a valid card");
            player_hand.push(hand);
        }

        print!("Enter dealer card: ");
        io::stdout().flush().unwrap();
        let mut dealer_card_input = String::new();
        from_keyboard
            .read_line(&mut dealer_card_input)
            .expect("Failed to read input");
        let dealer_card: i32 = dealer_card_input.trim().parse().expect("Please enter a valid number");

        let correct_play = hand1.get_correct_play(&player_hand, dealer_card);

        match correct_play {
            1 => println!("Play: Hit"),
            2 => println!("Play: Stand"),
            3 => {
                if player_hand.len() > 2 {
                    println!("Play: Hit");
                } else {
                    println!("Play: Double");
                }
            }
            4 => {
                if player_hand.len() > 2 {
                    println!("Play: Stand");
                } else {
                    println!("Play: Double");
                }
            }
            5 => println!("Play: Split"),
            _ => println!("Invalid move"),
        }
    }

    println!("\nYou have ended the Blackjack Hand Trainer! Thank you for playing!");
}
