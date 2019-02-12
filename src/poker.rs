use crate::card::{Card, LineNumber, Rank, Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TwoPair {
    top_rank: Rank,
    bottom_rank: Rank,
}

impl TwoPair {
    /// Construct a new TwoPair struct, returning an error if top_rank and bottom_rank are
    /// equal to each other. This method will also swap top_rank and bottom_rank if necessary
    /// to ensure that top_rank > bottom_rank.
    pub fn new(top_rank: Rank, bottom_rank: Rank) -> Result<Self, PokerError> {
        if top_rank > bottom_rank {
            Ok(Self {
                top_rank,
                bottom_rank,
            })
        } else if top_rank < bottom_rank {
            Ok(Self {
                top_rank: bottom_rank,
                bottom_rank: top_rank,
            })
        } else {
            Err(PokerError::InvalidArguments(
                "Top rank and bottom rank cannot be equal.".to_owned(),
            ))
        }
    }

    /// Return the top rank of the struct.
    pub fn get_top_rank(&self) -> Rank {
        self.top_rank
    }

    /// Return the bottom rank of the struct.
    pub fn get_bottom_rank(&self) -> Rank {
        self.bottom_rank
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FullHouse {
    triple: Rank,
    pair: Rank,
}

impl FullHouse {
    /// Construct a new FullHouse struct, returning an error if top_rank and bottom_rank are
    /// equal to each other.
    pub fn new(triple: Rank, pair: Rank) -> Result<Self, PokerError> {
        if triple != pair {
            Ok(Self { triple, pair })
        } else {
            Err(PokerError::InvalidArguments(
                "Top rank and bottom rank cannot be equal.".to_owned(),
            ))
        }
    }

    /// Return the triple of the struct.
    pub fn get_triple(&self) -> Rank {
        self.triple
    }

    /// Return the pair of the struct.
    pub fn get_pair(&self) -> Rank {
        self.pair
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandValue {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(TwoPair),
    ThreeOfAKind(Rank),
    Straight(Rank),
    FullHouse(FullHouse),
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

#[derive(Clone, Debug)]
pub enum PokerError {
    NotEnoughCards(String),
    InvalidArguments(String),
}

impl Hand {
    /// Return an empty hand.
    pub fn empty_hand() -> Hand {
        Hand { cards: vec![] }
    }

    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        for i in LineNumber::iter() {
            for card in &self.cards {
                output.push_str(&card.to_single_string(i));
                output.push_str(" ");
            }
            output.push_str("\n");
        }
        output
    }
}

impl Commune {
    /// Return True iff the Commune contains the input HandValue.
    pub fn contains_handvalue(&self, value: HandValue) -> bool {
        match value {
            HandValue::FourOfAKind(rank) => self.contains_x_cards_of_rank(4, rank),
            HandValue::FullHouse(full_house) => {
                self.contains_handvalue(HandValue::ThreeOfAKind(full_house.get_triple()))
                    && self.contains_handvalue(HandValue::OnePair(full_house.get_pair()))
            }
            HandValue::Straight(top_rank) => {
                if top_rank < Rank::Six {
                    false
                } else {
                    self.contains_straight(top_rank)
                }
            }
            HandValue::ThreeOfAKind(rank) => self.contains_x_cards_of_rank(3, rank),
            HandValue::TwoPair(two_pair) => {
                self.contains_x_cards_of_rank(2, two_pair.get_top_rank())
                    && self.contains_x_cards_of_rank(2, two_pair.get_bottom_rank())
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
        let ranks_in_straight = &all_ranks[top_rank_index - 6..top_rank_index - 1];
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

    fn default_commune() -> poker::Commune {
        poker::Commune {
            cards: vec![
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Spades,
                },
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Hearts,
                },
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Clubs,
                },
                card::Card {
                    rank: card::Rank::Queen,
                    suit: card::Suit::Diamonds,
                },
                card::Card {
                    rank: card::Rank::Jack,
                    suit: card::Suit::Diamonds,
                },
                card::Card {
                    rank: card::Rank::Ten,
                    suit: card::Suit::Spades,
                },
                card::Card {
                    rank: card::Rank::Nine,
                    suit: card::Suit::Clubs,
                },
                card::Card {
                    rank: card::Rank::Nine,
                    suit: card::Suit::Diamonds,
                },
                card::Card {
                    rank: card::Rank::Eight,
                    suit: card::Suit::Hearts,
                },
                card::Card {
                    rank: card::Rank::Three,
                    suit: card::Suit::Spades,
                },
            ],
        }
    }

    #[test]
    fn invalid_two_pair() {
        let res = poker::TwoPair::new(card::Rank::Three, card::Rank::Three);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_full_house() {
        let res = poker::FullHouse::new(card::Rank::Queen, card::Rank::Queen);
        assert!(res.is_err());
    }

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
    fn contains_handvalue_pairs_triples() {
        let commune = default_commune();
        assert!(commune.contains_handvalue(poker::HandValue::FullHouse(
            poker::FullHouse::new(card::Rank::Queen, card::Rank::Nine).unwrap()
        )));
        assert!(!commune.contains_handvalue(poker::HandValue::ThreeOfAKind(card::Rank::Nine)));
    }

    #[test]
    fn contains_handvalue_straight() {
        let commune = default_commune();
        assert!(commune.contains_handvalue(poker::HandValue::Straight(card::Rank::Queen)));
        assert!(!commune.contains_handvalue(poker::HandValue::Straight(card::Rank::Eight)));
        assert!(!commune.contains_handvalue(poker::HandValue::Straight(card::Rank::King)));
    }
}
