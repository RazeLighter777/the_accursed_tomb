
use std::fmt::Display;

use rand::{self, seq::SliceRandom};

/**
 * Represents a suit. 
 * PartialEq and Eq allow testing equality in suits, and Ord and PartialOrd allow sorting.
 * I don't have to put these numbers here (Like Hearts = 1, etc), but it allows comparison among enum values
 */
#[derive(Debug, PartialEq,Eq,PartialOrd,Ord,Clone)]
pub enum Suit {
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
    Spades = 4,
    Joker = 5,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
            Suit::Joker => write!(f, "🤡"),
        }
    }
}


/**
 * Represents a face. 
 * PartialEq and Eq allow testing equality in faces, and Ord and PartialOrd allow sorting.
 */
#[derive(Debug, PartialEq,Eq,PartialOrd,Ord,Clone)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 12,
    Queen = 13,
    King = 14,
    Joker = 15
}

impl Display for Rank {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            //represents a player
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "⑩"),

            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Joker => write!(f, "J"),
        }
    }

}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd, Ord)]
pub struct Card(pub Suit, pub Rank);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Card {
    pub fn get_card_rendering_vertical(&self) -> String {
        //draw the top edge
        let mut card_rendering = String::from("┌──┐\n");
        //draw the middle and suit/rank
        card_rendering.push_str(&format!("|{}{}|\n", self.0, self.1));
        //draw the bottom half
        card_rendering.push_str("|  |\n");
        //draw the bottom edge
        card_rendering.push_str("└──┘\n");
        card_rendering
    }
    pub fn get_card_rendering_horizontal(&self) -> String {
        //draw the top edge
        let mut card_rendering = String::from("┌─────┐\n");
        //draw the middle and suit/rank
        card_rendering.push_str(&format!("|  {}{} |\n", self.0, self.1));
        //draw the bottom edge
        card_rendering.push_str("└─────┘");
        card_rendering

    }
    pub fn get_card_meaning(&self) -> CardMeaning {
        match self {
            //queens are blessings
            Card(_, Rank::Queen) => CardMeaning::Blessing,
            //all tens are arch daemons
            Card(suit, Rank::Ten) => CardMeaning::ArchDaemon(
                match suit {
                    Suit::Hearts => ArchDaemon::UrAmon,
                    Suit::Diamonds => ArchDaemon::UrIo,
                    Suit::Spades => ArchDaemon::UrViveh,
                    Suit::Clubs => ArchDaemon::UrZhulah,
                    _ => panic!("Joker is not a valid suit for a ten"),
                }
            ),
            //7 is goblin
            Card(_, Rank::Seven) => CardMeaning::Goblin,
            //8 is orc
            Card(_, Rank::Eight) => CardMeaning::Orc,
            //9 is troll
            Card(_, Rank::Nine) => CardMeaning::Troll,
            //2 of diamonds is merchant
            Card(Suit::Diamonds, Rank::Two) => CardMeaning::Merchant,
            //2 of hearts is fountain
            Card(Suit::Hearts, Rank::Two) => CardMeaning::Fountain,
            //2 of spades is altar
            Card(Suit::Spades, Rank::Two) => CardMeaning::Altar,
            //2 of clubs is portal
            Card(Suit::Clubs, Rank::Two) => CardMeaning::Portal,
            //jokers are traps
            Card(Suit::Joker, _) => CardMeaning::Trap,
            //3s are weapons
            Card(_, Rank::Three) => CardMeaning::Weapon,
            //4,5 are walls
            Card(_, Rank::Four | Rank::Five) => CardMeaning::Wall,
            //all others are hallways
            _ => CardMeaning::Hallway,
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();
        for suit in vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
                cards.push(Card(suit.clone(), rank));
            }
        }
        // Add two jokers
        cards.push(Card(Suit::Joker, Rank::Joker));
        cards.push(Card(Suit::Joker, Rank::Joker));
        Deck { cards }
    }
}

impl Deck {

    pub fn shuffled_full_deck() -> Deck {
        let mut deck = Deck::default();
        deck.shuffle();
        deck
    }

    pub fn extend(&mut self, other : Deck) {
        self.cards.extend(other.cards);
    }

    pub fn sort_hand(&mut self) {
        self.cards.sort();
    }

    pub fn empty_deck() -> Deck {
        Deck { cards: Vec::new() }
    }
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }
    pub fn peek_and_move_to_back(&mut self) -> Option<&Card> {
        let card = self.cards.pop();
        if let Some(card) = card {
            self.cards.insert(0, card.clone());
        }
        self.cards.get(0)
    }
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    

    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }
}

pub enum ArchDaemon {
    UrZhulah,
    UrIo,
    UrAmon,
    UrViveh,
}

pub enum CardMeaning {
    Fountain,
    Hallway,
    Altar,
    Wall,
    ArchDaemon(ArchDaemon),
    Weapon,
    Portal,
    Blessing,
    //level 1
    Goblin,
    //level 2
    Orc,
    //level 3
    Troll,
    Merchant,
    Trap
}

impl CardMeaning {
    //turn enum into emoji
    pub fn emoji(&self) -> &str {
        match self {
            CardMeaning::Fountain => "⛲",
            CardMeaning::Hallway => "+",
            CardMeaning::Altar => "🕯️",
            CardMeaning::Wall => "#",
            CardMeaning::ArchDaemon(_) => "👿",
            CardMeaning::Weapon => "🗡️",
            CardMeaning::Portal => "🕳️",
            CardMeaning::Blessing => "🙏",
            CardMeaning::Goblin => "👺",
            CardMeaning::Orc => "👹",
            CardMeaning::Troll => "👾",
            CardMeaning::Merchant => "🤑",
            CardMeaning::Trap => "🔥",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_peek_and_move_to_back() {
        let mut deck = Deck::default();
        let card = deck.peek_and_move_to_back();
        assert!(card.is_some(),"Could not peek at card");
    }
    #[test]
    fn draw_all() {
        let mut deck = Deck::default();
        while let Some(card) = deck.draw() {
            println!("{}", card.get_card_rendering_horizontal());
            println!("{}", card.get_card_rendering_vertical());
        }
        assert_eq!(deck.cards_remaining(), 0,"Whole deck was not drawn");
    }

    #[test]
    fn extend() {
        //create two decks and extend and check the length.
        let mut deck1 = Deck::default();
        let deck2 = Deck::default();
        deck1.extend(deck2);
        assert_eq!(deck1.cards_remaining(), 108,"Deck could not be extended");
    }
    #[test]
    fn shuffle() {
        //compare a shuffled deck to a sorted deck
        //ok because probability of them being the same is 1/52 factorial
        let mut deck1 = Deck::default();
        let mut deck2 = Deck::default();
        deck1.shuffle();
        deck2.sort_hand();
        assert_ne!(deck1.cards, deck2.cards,"Deck did not shuffle properly");
    }

}

