use crate::card::{Card, Rank, Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandValue {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
pub enum PokerError {
    NotEnoughCards(String),
}

impl Hand {
    /// Return an empty hand.
    pub fn empty_hand() -> Hand {
        Hand { cards: vec![] }
    }
}

impl Deck {
    /// Return a standard, shuffled 52 card deck.
    pub fn get_full_deck() -> Self {
        let mut cards: Vec<Card> = Suit::iter()
            .cartesian_product(Rank::iter())
            .map(|(suit, rank)| Card { suit, rank })
            .collect();
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards: cards }
    }

    /// Deal cards from the deck.
    pub fn deal_cards(&mut self, num_cards: usize) -> Result<Hand, PokerError> {
        if num_cards > self.cards.len() {
            Err(PokerError::NotEnoughCards(
                "Tried to deal more cards than in deck.".to_owned(),
            ))
        } else {
            Ok(Hand {
                cards: self.cards.split_off(self.cards.len() - num_cards),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::poker;

    #[test]
    fn empty_hand() {
        let hand = poker::Hand::empty_hand();
        assert_eq!(0, hand.cards.len());
    }

    #[test]
    fn full_deck() {
        let deck = poker::Deck::get_full_deck();
        assert_eq!(52, deck.cards.len());
    }

    #[test]
    fn deal_cards_valid() {
        let mut deck = poker::Deck::get_full_deck();
        let hand = deck.deal_cards(7).unwrap();
        assert_eq!(7, hand.cards.len());
        assert_eq!(52, hand.cards.len() + deck.cards.len());
    }

    #[test]
    fn deal_cards_invalid() {
        let mut deck = poker::Deck::get_full_deck();
        let _ = deck.deal_cards(40).unwrap();
        let should_be_error = deck.deal_cards(40);
        assert!(should_be_error.is_err());
    }
}
