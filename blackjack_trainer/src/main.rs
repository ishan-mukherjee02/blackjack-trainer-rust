use std::fmt::format;
use rand::prelude::*;

struct Card{
    suit: String,
    rank: i64,
}
impl Card {
    pub fn new(suit: String, rank: i64) -> Self{
        Card{
            suit,
            rank
        }
    }

    /**
     * Returns this card with the 1 or 2 character value (A, 2-10, J, Q, K)
     * followed by the 1 character suit (D, H, S, C)
     * Examples: JD, 10H, AS, 9C
     */
    pub fn to_string(&self) -> String {
        let mut card = String::new();

        if self.rank == 1 {
            card.push('A');
        } else if self.rank == 11 {
            card.push('J');
        } else if self.rank == 12 {
            card.push('Q');
        } else if self.rank == 13 {
            card.push('K');
        } else {
            card.push_str(&self.rank.to_string());
        }
        return format!("{}{}", card, &self.suit)
    }
}

struct Hand{
    cards: Vec<Card>
}
impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        let mut cards1 = Vec::new();
        cards1.push(card1);
        cards1.push(card2);
        Hand{
           cards: cards1
        }
    }

    /**
     * Returns the numerical value of this hand according to the rules of blackjack
     * @return the numerical value of this hand
     */
    pub fn get_value(&self) -> i64 {       
        let mut sum = 0;   
        let mut aceCounter = 0;

        for card in &self.cards {
            if card.rank == 11 || card.rank == 12 || card.rank == 13 {
                sum += 10;
            }
            else {
                sum += card.rank
            }

            if card.rank == 1 {
                aceCounter += 1;
            }
        }

        if sum < 12 && aceCounter > 0 {
            sum += 10
        }

        return sum
    }

    /**
     * Returns true if this hand is a blackjack, false otherwise
     * @return true if this hand is a blackjack, false otherwise
     */
    pub fn is_blackjack(&self) -> bool {
        if self.cards.len() == 2 && self.get_value() == 21 {
            return true;
        }
        return false;
    }
    
     /**
     * Returns the cards in this hand followed by their numerical value
     * Ex: JS AH (21)
     */
    pub fn to_string(&self) -> String {
        let mut hand = String::new();

        for card in &self.cards  {
            hand.push_str(&card.to_string());
            hand.push(' ');
        }

        let result = format!("{}({})", hand, self.get_value());
        return result;
    }

    /**
     * Adds the specified card to this hand
     * @param card the card to add
     */
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

struct Shoe{
    decks: i64,
    shoe: Vec<Card>
} 
impl Shoe {

    /**
     * Constructs a shoe with the specified number of decks.
     * This shoe will be shuffled.
     * @param decks the number of decks
     */
    pub fn new(init_decks: i64) -> Self {
        let mut init_shoe = Vec::<Card>::new();
        let mut new_shoe = Shoe{
           decks: init_decks,
           shoe: init_shoe
        };
        new_shoe.reset();
        new_shoe
    }

     /**
     * Removes and returns a card from this shoe
     * @return the card removed from this shoe.
     */
    pub fn deal_card(&mut self) -> Option<Card> {
        return self.shoe.pop();
    }

    /**
     * Returns the number of cards left in this shoe
     * @return the number of cards left in this shoe
     */
    pub fn cards_left(self) -> usize {
        return self.shoe.len();
    }

    /**
     * Resets this shoe to contain all of its original cards.
     * This shoe will be shuffled.
     */
    pub fn reset(&mut self) {
        //  TODO: Implement
        let suits = ["H", "D", "C", "S"];
        self.shoe.clear();

        for suit in suits {
            for value in 1..=13 {
                for _ in 0..self.decks {
                    self.shoe.push( Card {
                        suit: suit.to_string(),
                        rank: value,
                    });
                }
            }
        }

        let mut rng = thread_rng();
        self.shoe.shuffle(&mut rng);
    }
}

fn main() {
    let card1 = Card {
        suit: String::from("C"),
        rank: 12,
    };
    let card2 = Card {
        suit: String::from("S"),
        rank: 1,
    }; 

    let hand1 = Hand::new(card1, card2);
        
    println!("{}", hand1.to_string());
}
