use rand::prelude::*;

pub struct Card{
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

pub struct Hand{
    cards: Vec<Card>
}
impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        let mut cards = Vec::new();
        cards.push(card1);
        cards.push(card2);
        Hand{ cards }
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

pub struct Shoe{
    decks: usize,
    shoe: Vec<Card>
} 
impl Shoe {
    /**
     * Constructs a shoe with the specified number of decks.
     * This shoe will be shuffled.
     * @param decks the number of decks
     */
    pub fn new(init_decks: usize) -> Self {
        let init_shoe = Vec::<Card>::new();
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
        self.shoe.pop() // `pop` returns an `Option<Card>`, no need to manually unwrap.
    }

    /**
     * Returns the number of cards left in this shoe
     * @return the number of cards left in this shoe
     */
    pub fn cards_left(&self) -> usize {
        return self.shoe.len();
    }

    /**
     * Resets this shoe to contain all of its original cards.
     * This shoe will be shuffled.
     */
    pub fn reset(&mut self) {
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

const DECKS: usize = 6;
const CARDS_PER_DECK: usize = 54;

pub struct Blackjack {
    shoe: Shoe,
    player_hand: Option<Hand>,
    dealer_hand: Option<Hand>,
}
impl Blackjack {
    pub fn new() -> Self {
        let init_shoe = Shoe::new(DECKS);

        let new_blackjack = Blackjack{
            shoe: init_shoe,
            player_hand: None,
            dealer_hand: None,
        };
        new_blackjack
    }

    /**
    * Resets for another round, including reseting shoe if necessary
    */
    pub fn reset(mut self) {
        if self.shoe.cards_left() as f64 / (CARDS_PER_DECK as f64 * DECKS as f64) <= 0.25 {
            self.shoe.reset()
        }
    }

    /**
     * Places a bet at the start of a round. Deals cards to the player and dealer.
     * @param amount the amount to bet
     */
    pub fn deal_card(&mut self) {
        let card1 = self.shoe.deal_card().expect("No more cards in the deck");
        let card2 = self.shoe.deal_card().expect("No more cards in the deck");
        let card3 = self.shoe.deal_card().expect("No more cards in the deck");
        let card4 = self.shoe.deal_card().expect("No more cards in the deck");

        self.player_hand = Some(Hand::new(card1, card2));
        self.dealer_hand = Some(Hand::new(card3, card4));
    }

    /**
     * Returns true if the player can hit, false otherwise
     */
    pub fn can_hit(&self) -> bool {
        let hand1 = &self.player_hand.as_ref().expect("NONE");
        let hand2 = &self.dealer_hand.as_ref().expect("NONE");
        
        return hand1.get_value() < 21 && !hand2.is_blackjack();
    }

    /**
     * Deals another card to the player's hand.
     * 
     * Precondition: canHit()
     */
    pub fn hit(&mut self) {
        self.player_hand.as_mut().expect("NONE").add_card(self.shoe.deal_card().expect("EMPTY"))
    }

    /**
     * Plays the dealer's hand.
     */
    pub fn play_dealers_hand(&mut self) {
        // Make sure dealer's hand exists.
        let dealer_hand = self.dealer_hand.as_mut().expect("Dealer hand is not initialized");

        // While the value of the dealer's hand is less than 17, continue to deal cards
        while dealer_hand.get_value() < 17 {
            let card = self.shoe.deal_card(); // Deal a card from the shoe
            dealer_hand.add_card(card.expect("None")); // Add it to the dealer's hand
        }

        // Print the dealer's final hand
        println!("Dealer has: {}", dealer_hand.to_string());
    }

    /**
     * Returns true if the player's hand is a push, false otherwise
     */
    pub fn is_push(&self) -> bool {
        let dealer_hand = self.dealer_hand.as_ref().expect("Dealer hand is not initialized");
        let player_hand = self.player_hand.as_ref().expect("Player hand is not initialized");

        // Condition 1: Dealer has blackjack, player does not, and player has 21
        if dealer_hand.is_blackjack() && !player_hand.is_blackjack() && player_hand.get_value() == 21 {
            return false;
        }

        // Condition 2: Player has blackjack, dealer does not, and dealer has 21
        if player_hand.is_blackjack() && !dealer_hand.is_blackjack() && dealer_hand.get_value() == 21 {
            return false;
        }

        // Condition 3: It's a push if both hands have the same value
        return dealer_hand.get_value() == player_hand.get_value();
    }

    /**
     * Returns true if the player's hand is a player win, false otherwise
     */
    pub fn is_player_win(&self) -> bool {
        let dealer_hand = self.dealer_hand.as_ref().expect("Dealer hand is not initialized");
        let player_hand = self.player_hand.as_ref().expect("Player hand is not initialized");

        // Dealer has blackjack, player loses
        if dealer_hand.is_blackjack() {
            return false;
        }

        // Player has blackjack, player wins
        if player_hand.is_blackjack() {
            return true;
        }

        // Dealer busts (over 21) and player does not bust
        if dealer_hand.get_value() > 21 && player_hand.get_value() <= 21 {
            return true;
        }

        // Player has a higher hand value than the dealer and doesn't bust
        if player_hand.get_value() > dealer_hand.get_value() && player_hand.get_value() <= 21 {
            return true;
        }

        // In all other cases, player loses
        return false;
    }

    pub fn get_players_hand(&self) -> Option<&Hand> {
        return self.player_hand.as_ref();
    }

    pub fn get_dealers_hand(&self) -> Option<&Hand> {
        return self.dealer_hand.as_ref();
    }

}

