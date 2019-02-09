use crate::poker::{Deck, Hand, HandValue, PokerError};

type GameResult = Result<(), GameError>;

#[derive(Clone, Debug)]
pub struct Player {
    name: u8,
    hand: Hand,
    penalties: u8,
}

#[derive(Clone, Debug)]
pub struct GameState {
    players: Vec<Player>,
    current_turn: usize,
    current_bet: Option<HandValue>,
    deck: Deck,
}

#[derive(Debug)]
pub enum GameMove {
    NewGame(u8),
    Bet(HandValue),
    Call(),
}

#[derive(Debug)]
pub enum GameError {
    Poker(PokerError),
}

impl From<PokerError> for GameError {
    fn from(e: PokerError) -> Self {
        GameError::Poker(e)
    }
}

impl Player {
    pub fn is_out(&self) -> bool {
        self.penalties >= 3
    }
}

impl GameState {
    pub fn process_move(&mut self, game_move: GameMove) -> GameResult {
        match game_move {
            GameMove::NewGame(num_players) => self.create_new_game(num_players)?,
            GameMove::Bet(value) => self.process_bet(value),
            GameMove::Call() => self.process_call(),
        };
        Ok(())
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
        unimplemented!()
    }

    fn process_call(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::game;
    use crate::poker;
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
        let mut state = game::GameState {
            players: vec![],
            current_turn: 0,
            current_bet: None,
            deck: poker::Deck::get_full_deck(),
        };
        state.create_new_game(3).unwrap();
        assert_eq!(3, state.players.len());
        assert!(state
            .players
            .iter()
            .all(|player| player.hand.cards.len() == 1));
    }
}
