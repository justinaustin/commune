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
    pub cards: Vec<Card>,
}

impl Hand {
    // returns an empty hand
    pub fn empty_hand() -> Hand {
        Hand { cards: vec![] }
    }
}

impl Deck {
    // returns a standard 52 card deck
    pub fn get_full_deck() -> Hand {
        let mut cards = vec![];
        for rank_u8 in 2..15 {
            let rank = Rank::from_u8(rank_u8).unwrap();
            let to_add = Card::get_all_with_rank(rank);
            for card in &to_add {
                cards.push(*card);
            }
        }
        thread_rng().shuffle(&mut cards);
        Hand { cards: cards }
    }
}