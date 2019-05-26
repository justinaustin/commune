use crate::card::{Card, LineNumber, Rank, Suit};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
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

    pub fn all_possible() -> Vec<Self> {
        Rank::iter()
            .cartesian_product(Rank::iter())
            .filter(|(top_rank, bottom_rank)| top_rank > bottom_rank)
            .filter_map(|(top_rank, bottom_rank)| Self::new(top_rank, bottom_rank).ok())
            .collect()
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

    pub fn all_possible() -> Vec<Self> {
        Rank::iter()
            .cartesian_product(Rank::iter())
            .filter_map(|(triple, pair)| Self::new(triple, pair).ok())
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Cumulative distribution function, where `p` is the probability of success, `n` is the
    /// number of trials, and `k` is the minimum number of successes.
    pub fn cumulative_probability(p: f64, n: u64, k: u64) -> Self {
        Self((k..=n).map(|i| Self::binomial_distribution(p, n, i).0).sum())
    }

    fn binomial_distribution(p: f64, n: u64, k: u64) -> Self {
        let n_choose_k = Self::factorial(n) / (Self::factorial(k) * Self::factorial(n - k));
        Self(n_choose_k * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32))
    }

    fn partial_factorial(lower_bound: u64, upper_bound: u64) -> f64 {
        (1..=n).map(|x| x as f64).product()
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

impl HandValue {
    pub fn all_possible() -> Vec<Self> {
        vec![
            Rank::iter()
                .map(|rank| HandValue::HighCard(rank))
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(|rank| HandValue::OnePair(rank))
                .collect::<Vec<Self>>(),
            TwoPair::all_possible()
                .into_iter()
                .map(|twopair| HandValue::TwoPair(twopair))
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(|rank| HandValue::ThreeOfAKind(rank))
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(|rank| HandValue::Straight(rank))
                .collect::<Vec<Self>>(),
            FullHouse::all_possible()
                .into_iter()
                .map(|fullhouse| HandValue::FullHouse(fullhouse))
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(|rank| HandValue::FourOfAKind(rank))
                .collect::<Vec<Self>>(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
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

    pub fn calculate_handvalue_probabilities(
        &self,
        other_cards_in_play: usize,
    ) -> Vec<(HandValue, Probability)> {
        let mut handvalue_probabilities: Vec<(HandValue, Probability)> = HandValue::all_possible()
            .into_iter()
            .map(|handvalue| {
                (
                    handvalue,
                    self.calculate_hand_probability(handvalue, other_cards_in_play),
                )
            })
            .collect();
        handvalue_probabilities
    }

    fn calculate_hand_probability(
        &self,
        handvalue: HandValue,
        other_cards_in_play: usize,
    ) -> Probability {
        let total_cards = Deck::full_deck_size() - self.cards.len();
        let needed_card_sets = self.get_needed_cards_for_handvalue(handvalue);

        let probability_of_each = needed_card_sets.iter().map(|cards| )
    }

    fn get_needed_cards_for_handvalue(&self, handvalue: HandValue) -> Vec<HashSet<Card>> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
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

    pub fn full_deck_size() -> usize {
        52
    }
}

#[derive(Clone, Debug)]
pub struct Commune {
    pub cards: Vec<Card>,
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
        println!("Rank: {:?}", top_rank);
        let top_rank_index = u8::from(top_rank) as usize;
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

#[derive(Clone, Debug)]
pub enum PokerError {
    NotEnoughCards(String),
    InvalidArguments(String),
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
    fn all_possible_two_pairs() {
        let all_two_pairs = poker::TwoPair::all_possible();
        assert_eq!(78, all_two_pairs.len());
    }

    #[test]
    fn invalid_full_house() {
        let res = poker::FullHouse::new(card::Rank::Queen, card::Rank::Queen);
        assert!(res.is_err());
    }

    #[test]
    fn all_possible_full_house() {
        let all_full_house = poker::FullHouse::all_possible();
        assert_eq!(156, all_full_house.len());
    }

    #[test]
    fn all_possible_hand_value() {
        let all_hand_value = poker::HandValue::all_possible();
        assert_eq!(299, all_hand_value.len());
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

    #[test]
    fn binomial_distribution() {
        let epislon = 1.0e-6;

        assert!((0.75 - poker::Probability::cumulative_probability(0.5, 2, 1).0).abs() < epislon);
        assert!((0.31532766369 - poker::Probability::cumulative_probability(0.33, 17, 7).0).abs() < epislon);
    }
}
