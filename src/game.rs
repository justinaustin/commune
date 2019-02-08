use crate::poker::{Hand, HandValue};

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
}

#[derive(Debug)]
pub enum GameMove {
    // TODO: Make struct with num_players and
    // NewGame(u8, u8),
    Bet(HandValue),
    Call(),
}

impl Player {
    pub fn is_out(&self) -> bool {
        self.penalties >= 3
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
}
