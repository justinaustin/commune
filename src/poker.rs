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
    pub fn get_top_rank(self) -> Rank {
        self.top_rank
    }

    /// Return the bottom rank of the struct.
    pub fn get_bottom_rank(self) -> Rank {
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
    pub fn get_triple(self) -> Rank {
        self.triple
    }

    /// Return the pair of the struct.
    pub fn get_pair(self) -> Rank {
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

/// Poker value of a hand. For a `Straight`, the inner `Rank` is the highest rank of the straight.
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
            Rank::iter().map(HandValue::HighCard).collect::<Vec<Self>>(),
            Rank::iter().map(HandValue::OnePair).collect::<Vec<Self>>(),
            TwoPair::all_possible()
                .into_iter()
                .map(HandValue::TwoPair)
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(HandValue::ThreeOfAKind)
                .collect::<Vec<Self>>(),
            Rank::iter().map(HandValue::Straight).collect::<Vec<Self>>(),
            FullHouse::all_possible()
                .into_iter()
                .map(HandValue::FullHouse)
                .collect::<Vec<Self>>(),
            Rank::iter()
                .map(HandValue::FourOfAKind)
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

    pub fn calculate_hand_value_probabilities(
        &self,
        other_cards_in_play: usize,
    ) -> Vec<(HandValue, Probability)> {
        let mut hand_value_probabilities: Vec<(HandValue, Probability)> = HandValue::all_possible()
            .into_iter()
            .map(|hand_value| {
                (
                    hand_value,
                    self.calculate_hand_probability(hand_value, other_cards_in_play),
                )
            })
            .collect();
        hand_value_probabilities
    }

    /// Calculate the probability of the input hand value given the player's cards and how many
    /// other cards are in play.
    ///
    /// If `p_i` is the probability of the `i`th group of needed cards, then the probability of
    /// none of the sets of needed cards appearing is `(1 - p_1)(1 - p_2)...(1 - p_n)`. So, the
    /// probability of at least one set of needed cards, and therefore the probability of the
    /// hand_value, is 1 - that.
    fn calculate_hand_probability(
        &self,
        hand_value: HandValue,
        other_cards_in_play: usize,
    ) -> Probability {
        let total_cards = Deck::full_deck_size() - self.cards.len();
        let needed_card_sets = self.get_needed_cards_for_hand_value(hand_value);

        let probability_of_each = needed_card_sets.iter().map(|cards| {
            Self::permutation(other_cards_in_play, cards.len()) as f64
                / Self::permutation(total_cards, cards.len()) as f64
        });
        let probability_of_none: f64 = probability_of_each.map(|prob| 1.0 - prob).product();
        Probability(1.0 - probability_of_none)
    }

    fn get_needed_cards_for_hand_value(&self, hand_value: HandValue) -> Vec<HashSet<Card>> {
        match hand_value {
            HandValue::HighCard(rank) => self.get_needed_cards_for_rank_hand(1, rank),
            HandValue::OnePair(rank) => self.get_needed_cards_for_rank_hand(2, rank),
            HandValue::TwoPair(two_pair) => unimplemented!(),
            HandValue::ThreeOfAKind(rank) => self.get_needed_cards_for_rank_hand(3, rank),
            HandValue::Straight(rank) => unimplemented!(),
            HandValue::FullHouse(full_house) => unimplemented!(),
            HandValue::FourOfAKind(rank) => self.get_needed_cards_for_rank_hand(4, rank),
        }
    }

    /// Return the minimal needed sets of Cards for a given number of `Rank` cards.
    fn get_needed_cards_for_rank_hand(&self, size: u8, rank: Rank) -> Vec<HashSet<Card>> {
        // TODO: Test me
        let mut needed_card_sets = vec![];
        let all_cards_in_rank = Card::get_all_with_rank(rank);
        let rank_cards_not_in_hand: Vec<Card> = all_cards_in_rank
            .iter()
            .filter(|card| !self.cards.contains(card))
            .cloned()
            .collect();
        for i in 0..=rank_cards_not_in_hand.len() {
            let combinations: Vec<Vec<Card>> = rank_cards_not_in_hand
                .iter()
                .cloned()
                .combinations(i)
                .collect();
            let augmented_hands = combinations.iter().map(|card_set| {
                let mut current_cards = self.cards.clone();
                current_cards.extend(card_set);
                current_cards
            });
            for augmented_hand in augmented_hands {
                let future_commune = Commune {
                    cards: augmented_hand,
                };
                if future_commune.contains_x_cards_of_rank(size, rank) {
                    needed_card_sets.push(future_commune.cards.into_iter().collect());
                }
            }

            if needed_card_sets.len() > 0 {
                return needed_card_sets;
            }
        }

        needed_card_sets
    }

    fn permutation(n: usize, k: usize) -> usize {
        (n - k + 1..=n).product()
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

        Self { cards }
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
    pub fn contains_hand_value(&self, value: HandValue) -> bool {
        match value {
            HandValue::FourOfAKind(rank) => self.contains_x_cards_of_rank(4, rank),
            HandValue::FullHouse(full_house) => {
                self.contains_hand_value(HandValue::ThreeOfAKind(full_house.get_triple()))
                    && self.contains_hand_value(HandValue::OnePair(full_house.get_pair()))
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
    use crate::card::{Card, Rank, Suit};
    use crate::poker::*;
    use assert_approx_eq::assert_approx_eq;
    use itertools::Itertools;
    use std::collections::HashSet;

    fn default_commune() -> Commune {
        Commune {
            cards: vec![
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spades,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamonds,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Hearts,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spades,
                },
            ],
        }
    }

    #[test]
    fn invalid_two_pair() {
        let res = TwoPair::new(Rank::Three, Rank::Three);
        assert!(res.is_err());
    }

    #[test]
    fn all_possible_two_pairs() {
        let all_two_pairs = TwoPair::all_possible();
        assert_eq!(78, all_two_pairs.len());
    }

    #[test]
    fn invalid_full_house() {
        let res = FullHouse::new(Rank::Queen, Rank::Queen);
        assert!(res.is_err());
    }

    #[test]
    fn all_possible_full_house() {
        let all_full_house = FullHouse::all_possible();
        assert_eq!(156, all_full_house.len());
    }

    #[test]
    fn all_possible_hand_value() {
        let all_hand_value = HandValue::all_possible();
        assert_eq!(299, all_hand_value.len());
    }

    #[test]
    fn empty_hand() {
        let hand = Hand::empty_hand();
        assert_eq!(0, hand.cards.len());
    }

    #[test]
    fn full_deck() {
        let deck = Deck::get_full_deck();
        assert_eq!(52, deck.cards.len());
    }

    #[test]
    fn deal_cards_valid() {
        let mut deck = Deck::get_full_deck();
        let hand = deck.deal_cards(7).unwrap();
        assert_eq!(7, hand.cards.len());
        assert_eq!(52, hand.cards.len() + deck.cards.len());
    }

    #[test]
    fn deal_cards_invalid() {
        let mut deck = Deck::get_full_deck();
        let _ = deck.deal_cards(40).unwrap();
        let should_be_error = deck.deal_cards(40);
        assert!(should_be_error.is_err());
    }

    #[test]
    fn contains_hand_value_pairs_triples() {
        let commune = default_commune();
        assert!(commune.contains_hand_value(HandValue::FullHouse(
            FullHouse::new(Rank::Queen, Rank::Nine).unwrap()
        )));
        assert!(!commune.contains_hand_value(HandValue::ThreeOfAKind(Rank::Nine)));
    }

    #[test]
    fn contains_hand_value_straight() {
        let commune = default_commune();
        assert!(commune.contains_hand_value(HandValue::Straight(Rank::Queen)));
        assert!(!commune.contains_hand_value(HandValue::Straight(Rank::Eight)));
        assert!(!commune.contains_hand_value(HandValue::Straight(Rank::King)));
    }

    #[test]
    fn partial_factorial() {
        assert_eq!(20, Hand::permutation(5, 2));
        assert_eq!(9_480_240, Hand::permutation(57, 4));
    }

    #[test]
    fn get_needed_cards_for_hand_value_none_needed() {
        let hand = Hand {
            cards: Card::get_all_with_rank(Rank::Ace),
        };
        let empty: Vec<HashSet<Card>> = vec![];
        assert_eq!(
            empty,
            hand.get_needed_cards_for_hand_value(HandValue::FourOfAKind(Rank::Ace))
        );
    }

    #[test]
    fn get_needed_cards_for_hand_value_rank() {
        let rank = Rank::Queen;
        let hand = Hand {
            cards: vec![Card { suit: Suit::Clubs, rank }],
        };
        let mut first_combination = HashSet::new();
        first_combination.insert(Card { suit: Suit::Diamonds, rank});
        first_combination.insert(Card { suit: Suit::Hearts, rank});

        let mut second_combination = HashSet::new();
        second_combination.insert(Card { suit: Suit::Diamonds, rank});
        second_combination.insert(Card { suit: Suit::Spades, rank});

        let mut third_combination = HashSet::new();
        third_combination.insert(Card { suit: Suit::Hearts, rank});
        third_combination.insert(Card { suit: Suit::Spades, rank});

    }

    #[test]
    fn get_needed_cards_for_hand_value_straight() {
        let hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Two,
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Three,
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Four,
                },
            ],
        };
        let expected: Vec<HashSet<Card>> = Card::get_all_with_rank(Rank::Five)
            .into_iter()
            .cartesian_product(Card::get_all_with_rank(Rank::Six).into_iter())
            .map(|(rank_five, rank_six)| {
                let mut set = HashSet::new();
                set.insert(rank_five);
                set.insert(rank_six);
                set
            })
            .collect();
        let actual = hand.get_needed_cards_for_hand_value(HandValue::Straight(Rank::Six));

        assert_eq!(expected.len(), actual.len());
        assert!(actual.iter().all(|cards| expected.contains(cards)));
    }

    #[test]
    fn calculate_hand_probability() {
        let hand = Hand {
            cards: vec![Card {
                suit: Suit::Clubs,
                rank: Rank::Ten,
            }],
        };
        assert_approx_eq!(
            0.12390956,
            hand.calculate_hand_probability(HandValue::ThreeOfAKind(Rank::Ten), 11)
                .0
        );
    }
}
