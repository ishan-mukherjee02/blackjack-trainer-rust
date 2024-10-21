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

    pub fn to_string(&self) -> String {
        format!("{} {}", &self.rank, &self.suit)
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
    
    pub fn to_string(&self) -> String {
        let mut hand = String::new();

        for card in &self.cards  {
            hand.push_str(&card.to_string());
            hand.push(' ');
        }
        return hand;
    }
}

fn main() {
    let card1 = Card {
        suit: String::from("Clubs"),
        rank: 12,
    };

    println!("{}", card1.to_string());
}
