use crate::poker::{Commune, Deck, Hand, HandValue, PokerError};
use std::num::ParseIntError;

pub type GameResult = Result<(), GameError>;

#[derive(Clone, Debug)]
pub struct Player {
    pub name: u8,
    pub hand: Hand,
    pub penalties: u8,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub current_bet: Option<HandValue>,
    pub deck: Deck,
}

#[derive(Debug)]
pub enum GameMove {
    Bet(HandValue),
    Call(),
}

#[derive(Debug)]
pub enum GameError {
    Poker(PokerError),
    CallWithNoBet,
    IO,
}

impl From<PokerError> for GameError {
    fn from(e: PokerError) -> Self {
        GameError::Poker(e)
    }
}

impl From<ParseIntError> for GameError {
    fn from(e: ParseIntError) -> Self {
        GameError::IO
    }
}

impl Player {
    pub fn is_out(&self) -> bool {
        self.penalties >= 3
    }
}

impl GameState {
    pub fn init_game(num_players: u8) -> Self {
        let mut new_game = Self {
            players: vec![],
            current_turn: 0,
            current_bet: None,
            deck: Deck::get_full_deck(),
        };
        new_game.create_new_game(num_players);
        new_game
    }

    pub fn process_move(&mut self, game_move: GameMove) -> GameResult {
        match game_move {
            GameMove::Bet(value) => self.process_bet(value),
            GameMove::Call() => self.process_call()?,
        };
        Ok(())
    }

    pub fn display(&self) {
        for player in &self.players {
            println!("Player {}: ", player.name);
            println!("{}", player.hand.to_string());
        }
    }

    fn create_new_game(&mut self, num_players: u8) -> GameResult {
        let players = (0..num_players)
            .map(|name| Player {
                name,
                hand: Hand::empty_hand(),
                penalties: 0,
            })
            .collect();
        let mut new_game = Self {
            players,
            current_turn: 0,
            current_bet: None,
            deck: Deck::get_full_deck(),
        };
        new_game.deal_hands()?;
        *self = new_game;
        Ok(())
    }

    fn deal_hands(&mut self) -> GameResult {
        for player in self.players.iter_mut() {
            let num_cards = player.penalties + 1;
            player.hand = self.deck.deal_cards(num_cards as usize)?;
        }
        Ok(())
    }

    fn process_bet(&mut self, value: HandValue) {
        self.current_bet = Some(value);
        self.increment_turn();
    }

    fn increment_turn(&mut self) {
        self.current_turn += 1;
        if self.current_turn == self.players.len() {
            self.current_turn = 0;
        }
    }

    fn process_call(&mut self) -> GameResult {
        match self.current_bet {
            None => Err(GameError::CallWithNoBet),
            Some(bet) => {
                let penalized_player = if self.gather_all_cards().contains_handvalue(bet) {
                    self.current_turn
                } else {
                    self.get_previous_player()
                };
                self.penalize_player(penalized_player);
                self.current_turn = penalized_player;
                self.current_bet = None;
                self.deal_hands()
            }
        }
    }

    fn get_previous_player(&self) -> usize {
        if self.current_turn > 0 {
            self.current_turn - 1
        } else {
            self.players.len() - 1
        }
    }

    fn penalize_player(&mut self, player: usize) {
        self.players[player].penalties += 1;
        if self.players[player].is_out() {
            self.players.remove(player);
        }
    }

    fn gather_all_cards(&self) -> Commune {
        Commune {
            cards: self
                .players
                .iter()
                .flat_map(|player| player.hand.cards.iter().map(|card| *card))
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::card;
    use crate::game;
    use crate::poker;

    fn default_gamestate() -> game::GameState {
        game::GameState {
            players: vec![],
            current_turn: 0,
            current_bet: None,
            deck: poker::Deck::get_full_deck(),
        }
    }

    #[test]
    fn player_is_out() {
        let in_player = game::Player {
            name: 0,
            hand: poker::Hand::empty_hand(),
            penalties: 1,
        };
        let out_player = game::Player {
            name: 0,
            hand: poker::Hand::empty_hand(),
            penalties: 3,
        };
        assert!(!in_player.is_out());
        assert!(out_player.is_out());
    }

    #[test]
    fn new_game() {
        let mut state = default_gamestate();
        state.create_new_game(3).unwrap();
        assert_eq!(3, state.players.len());
        assert!(state
            .players
            .iter()
            .all(|player| player.hand.cards.len() == 1));
    }

    #[test]
    fn bet() {
        let mut state = default_gamestate();
        state.create_new_game(3).unwrap();
        state.current_turn = 2;
        state.process_bet(poker::HandValue::ThreeOfAKind(card::Rank::Ten));
        assert_eq!(0, state.current_turn);
    }

    #[test]
    fn gather_all_cards() {
        let mut state = default_gamestate();
        state.create_new_game(3).unwrap();
        let gathered_cards = state.gather_all_cards().cards;
        assert!(state
            .players
            .iter()
            .all(|player| gathered_cards.contains(&player.hand.cards[0])));
        assert_eq!(3, gathered_cards.len());
    }

    #[test]
    fn unsuccessful_call() {
        let mut state = default_gamestate();
        state.create_new_game(3).unwrap();
        state.current_bet = Some(poker::HandValue::FourOfAKind(card::Rank::Ace));
        let penalized_player = 2;
        state.process_call().unwrap();
        assert_eq!(2, state.players[penalized_player].hand.cards.len());
        assert_eq!(1, state.players[0].hand.cards.len());
        assert_eq!(1, state.players[1].hand.cards.len());
    }

    #[test]
    fn successful_call() {
        let mut state = default_gamestate();
        state.create_new_game(3).unwrap();
        let existing_rank = state.players[0].hand.cards[0].rank;
        state.current_bet = Some(poker::HandValue::HighCard(existing_rank));
        let penalized_player = 0;
        state.process_call().unwrap();
        assert_eq!(2, state.players[penalized_player].hand.cards.len());
        assert_eq!(1, state.players[1].hand.cards.len());
        assert_eq!(1, state.players[2].hand.cards.len());
    }
}
