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

#[derive(Clone, Debug)]
pub struct Commune {
    pub cards: Vec<Card>,
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

impl Commune {
    /// Return True iff the Commune contains the input HandValue.
    pub fn contains_handvalue(&self, value: HandValue) -> bool {
        match value {
            HandValue::FourOfAKind(rank) => self.contains_x_cards_of_rank(4, rank),
            HandValue::FullHouse(three_of, two_of) => {
                self.contains_handvalue(HandValue::ThreeOfAKind(three_of))
                    && self.contains_handvalue(HandValue::OnePair(two_of))
            }
            HandValue::Straight(top_rank) => {
                if top_rank < Rank::Six {
                    false
                } else {
                    self.contains_straight(top_rank)
                }
            }
            HandValue::ThreeOfAKind(rank) => self.contains_x_cards_of_rank(3, rank),
            HandValue::TwoPair(first, second) => {
                self.contains_x_cards_of_rank(2, first) && self.contains_x_cards_of_rank(2, second)
            }
            HandValue::OnePair(rank) => self.contains_x_cards_of_rank(2, rank),
            HandValue::HighCard(rank) => self.contains_x_cards_of_rank(1, rank),
        }
    }

    fn contains_x_cards_of_rank(&self, x: u8, rank: Rank) -> bool {
        let needed_cards = Card::get_all_with_rank(rank);
        let num_cards = needed_cards.iter().fold(0, |acc, card| {
            acc + if self.cards.contains(card) { 1 } else { 0 }
        });
        num_cards >= x
    }

    fn contains_straight(&self, top_rank: Rank) -> bool {
        let top_rank_index = top_rank.to_u8() as usize;
        let all_ranks: Vec<Rank> = Rank::iter().collect();
        let ranks_in_straight: Vec<Rank> = &all_ranks[0: top_rank_index]
            .iter()
            .rev()
            .take(5)
            .collect();
        let all_possible_cards_in_straight: Vec<Vec<Card>> = ranks_in_straight
            .iter()
            .map(|rank| Card::get_all_with_rank(*rank))
            .collect();
        all_possible_cards_in_straight
            .into_iter()
            .all(|cards_in_rank| {
                cards_in_rank
                    .into_iter()
                    .any(|card| self.cards.contains(&card))
            })
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
    use crate::card;
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

    #[test]
    fn contains_handvalue() {
        let commune = poker::Commune {
            cards: vec![
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Hearts,
                },
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Clubs,
                },
                card::Card {
                    rank: card::Rank::Nine,
                    suit: card::Suit::Clubs,
                },
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Diamonds,
                },
                card::Card {
                    rank: card::Rank::Nine,
                    suit: card::Suit::Diamonds,
                },
                card::Card {
                    rank: card::Rank::Three,
                    suit: card::Suit::Spades,
                },
            ],
        };
        assert!(commune.contains_handvalue(poker::HandValue::FullHouse(
            card::Rank::Queen,
            card::Rank::Nine,
        )));
        assert!(!commune.contains_handvalue(poker::HandValue::ThreeOfAKind(card::Rank::Nine)));
    }
}
