use std::cmp::Ordering;
use std::fmt;

use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineNumber {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self {
            &Suit::Clubs => "♣",
            &Suit::Diamonds => "♦",
            &Suit::Hearts => "♥",
            &Suit::Spades => "♠",
        };
        write!(f, "{}", representation)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self {
            &Rank::Two => "2",
            &Rank::Three => "3",
            &Rank::Four => "4",
            &Rank::Five => "5",
            &Rank::Six => "6",
            &Rank::Seven => "7",
            &Rank::Eight => "8",
            &Rank::Nine => "9",
            &Rank::Ten => "10",
            &Rank::Jack => "J",
            &Rank::Queen => "Q",
            &Rank::King => "K",
            &Rank::Ace => "A",
        };
        write!(f, "{}", representation)
    }
}

impl Rank {
    pub fn to_u8(&self) -> u8 {
        match self {
            &Rank::Two => 2,
            &Rank::Three => 3,
            &Rank::Four => 4,
            &Rank::Five => 5,
            &Rank::Six => 6,
            &Rank::Seven => 7,
            &Rank::Eight => 8,
            &Rank::Nine => 9,
            &Rank::Ten => 10,
            &Rank::Jack => 11,
            &Rank::Queen => 12,
            &Rank::King => 13,
            &Rank::Ace => 14,
        }
    }

    pub fn from_u8(num: u8) -> Option<Rank> {
        match num {
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            14 => Some(Rank::Ace),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Rank> {
        match s {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "10" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Card {
    pub fn to_single_string(&self, line: LineNumber) -> String {
        let card_boundary = "+-----+".to_owned();
        match line {
            LineNumber::Zero | LineNumber::Four => card_boundary,
            LineNumber::One => {
                if self.rank != Rank::Ten {
                    format!("|{} {}  |", self.rank, self.suit)
                } else {
                    format!("|{}{}  |", self.rank, self.suit)
                }
            }
            LineNumber::Two => format!("|  {}  |", self.suit),
            LineNumber::Three => {
                if self.rank != Rank::Ten {
                    format!("|  {} {}|", self.suit, self.rank)
                } else {
                    format!("|  {}{}|", self.suit, self.rank)
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = "".to_owned();
        let num_lines = LineNumber::lines().len();
        for (i, line) in LineNumber::lines().into_iter().enumerate() {
            output.push_str(&self.to_single_string(*line));
            if i < num_lines - 1 {
                output.push_str("\n");
            }
        }
        output
    }

    pub fn get_all_with_rank(rank: Rank) -> Vec<Card> {
        Suit::iter().map(|suit| Card { suit, rank }).collect()
    }

    pub fn get_all_with_suit(suit: Suit) -> Vec<Card> {
        Rank::iter().map(|rank| Card { suit, rank }).collect()
    }
}

impl LineNumber {
    pub fn lines() -> [LineNumber; 5] {
        [
            LineNumber::Zero,
            LineNumber::One,
            LineNumber::Two,
            LineNumber::Three,
            LineNumber::Four,
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::card;
    #[test]
    fn display_suit() {
        let suit = card::Suit::Hearts;
        assert_eq!("♥", format!("{}", suit));
    }
    #[test]
    fn display_rank() {
        let suit = card::Rank::Queen;
        assert_eq!("Q", format!("{}", suit));
    }
    #[test]
    fn card_ordering() {
        let card1 = card::Card {
            rank: card::Rank::Three,
            suit: card::Suit::Diamonds,
        };
        let card2 = card::Card {
            rank: card::Rank::Jack,
            suit: card::Suit::Clubs,
        };
        assert!(card1 < card2);
    }
    #[test]
    fn all_suits() {
        let mut cards = card::Card::get_all_with_suit(card::Suit::Hearts);
        let initial_len = cards.len();
        cards.sort();
        cards.dedup();
        assert_eq!(initial_len, cards.len());
    }
    #[test]
    fn all_ranks() {
        let mut cards = card::Card::get_all_with_rank(card::Rank::Ace);
        let initial_len = cards.len();
        cards.sort();
        cards.dedup();
        assert_eq!(initial_len, cards.len());
    }
}
