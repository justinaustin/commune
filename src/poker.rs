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
}
