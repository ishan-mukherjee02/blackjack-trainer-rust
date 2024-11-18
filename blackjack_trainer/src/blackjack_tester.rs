use crate::blackjack::{Card, Hand, Shoe, Blackjack};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new("H".to_string(), 1);
        assert_eq!(card.to_string(), "AH");

        let card = Card::new("D".to_string(), 13);
        assert_eq!(card.to_string(), "KD");

        let card = Card::new("C".to_string(), 10);
        assert_eq!(card.to_string(), "10C");
    }

    #[test]
    fn test_hand_value() {
        let card1 = Card::new("H".to_string(), 1); // Ace
        let card2 = Card::new("S".to_string(), 10);
        let hand = Hand::new(card1, card2);
        assert_eq!(hand.get_value(), 21);
        assert!(hand.is_blackjack());

        let card3 = Card::new("D".to_string(), 7);
        let card4 = Card::new("C".to_string(), 6);
        let hand2 = Hand::new(card3, card4);
        assert_eq!(hand2.get_value(), 13);
        assert!(!hand2.is_blackjack());
    }

    #[test]
    fn test_hand_add_card() {
        let mut hand = Hand::new(Card::new("H".to_string(), 5), Card::new("S".to_string(), 6));
        hand.add_card(Card::new("D".to_string(), 10));
        assert_eq!(hand.get_value(), 21);
        assert!(!hand.is_blackjack()); // More than two cards, so not a blackjack
    }

    #[test]
    fn test_shoe_initialization_and_deal() {
        let mut shoe = Shoe::new(2); // Two decks
        assert_eq!(shoe.cards_left(), 104);

        let card = shoe.deal_card();
        assert!(card.is_some()); // Check that a card was dealt
        assert_eq!(shoe.cards_left(), 103); // One less card after dealing
    }

    #[test]
    fn test_shoe_reset() {
        let mut shoe = Shoe::new(2); // Two decks
        shoe.deal_card();
        shoe.deal_card();
        shoe.reset();
        assert_eq!(shoe.cards_left(), 104); // Should be reset to full deck count
    }

    #[test]
    fn test_blackjack_deal_and_play() {
        let mut game = Blackjack::new();
        game.deal_card();

        let player_hand = game.get_players_hand();
        let dealer_hand = game.get_dealers_hand();

        assert!(player_hand.is_some());
        assert!(dealer_hand.is_some());

        let player_value = player_hand.unwrap().get_value();
        let dealer_value = dealer_hand.unwrap().get_value();

        assert!(player_value > 0 && player_value <= 21);
        assert!(dealer_value > 0 && dealer_value <= 21);
    }

    #[test]
    fn test_blackjack_player_win_conditions() {
        let mut game = Blackjack::new();
        
        // Manually set a winning condition for the player
        game.player_hand = Some(Hand::new(Card::new("H".to_string(), 10), Card::new("S".to_string(), 10)));
        game.dealer_hand = Some(Hand::new(Card::new("D".to_string(), 9), Card::new("C".to_string(), 7)));
        
        assert!(game.is_player_win());
    }

    #[test]
    fn test_blackjack_push_conditions() {
        let mut game = Blackjack::new();
        
        // Set up a push condition where both have the same value
        game.player_hand = Some(Hand::new(Card::new("H".to_string(), 10), Card::new("S".to_string(), 10)));
        game.dealer_hand = Some(Hand::new(Card::new("D".to_string(), 10), Card::new("C".to_string(), 10)));
        
        assert!(game.is_push());
    }

    #[test]
    fn test_blackjack_dealer_plays_hand() {
        let mut game = Blackjack::new();
        game.deal_card();
        
        // Check if dealer plays to 17 or higher
        game.play_dealers_hand();
        let dealer_hand = game.get_dealers_hand().unwrap();
        assert!(dealer_hand.get_value() >= 17);
    }
}
