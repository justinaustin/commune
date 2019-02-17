mod card;
mod game;
mod poker;

#[macro_use]
extern crate strum_macros;

use crate::card::{Rank, Suit};
use crate::game::{GameError, GameMove, GameResult, GameState};
use crate::poker::{FullHouse, HandValue, TwoPair};
use std::io;

fn main() -> GameResult {
    println!("Welcome to Commune!");
    let mut state = new_game()?;
    game_loop(&mut state)?;
    Ok(())
}

fn new_game() -> Result<GameState, GameError> {
    println!("How many players?");
    let num_players = parse_players()?;
    Ok(GameState::init_game(num_players))
}

fn game_loop(state: &mut GameState) -> GameResult {
    loop {
        state.display();
        println!("Current Bet: {:?}", state.current_bet);
        println!(
            "Player {} - What is your next move? (new, bet, call)",
            state.players[state.current_turn].name
        );
        process_user_input(state)?;
    }
}

fn process_user_input(state: &mut GameState) -> GameResult {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            match input.trim() {
                "new" => {
                    let new_state = new_game()?;
                    *state = new_state;
                }
                "bet" => {
                    let handvalue = parse_handvalue()?;
                    state.process_move(GameMove::Bet(handvalue))?;
                }
                "call" => {
                    state.process_move(GameMove::Call())?;
                }
                _ => println!("Invalid input!"),
            };
            Ok(())
        }
        Err(error) => Err(GameError::IO),
    }
}

fn parse_players() -> Result<u8, GameError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let num = input.trim().parse()?;
            Ok(num)
        }
        Err(_) => Err(GameError::IO),
    }
}

fn parse_handvalue() -> Result<HandValue, GameError> {
    println!("Enter Your Bet (e.g. quad A):");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let vec: Vec<&str> = input.split(" ").collect();
            let hand = vec[0].trim();
            let rank_one = Rank::from_str(vec[1].trim());
            let rank_two = if vec.len() == 3 {
                Rank::from_str(vec[2].trim())
            } else {
                None
            };

            // TODO: refactor into a separate method. Three nested matches isn't good!
            match rank_one {
                None => Err(GameError::IO),
                Some(rank) => {
                    let handvalue = match hand {
                        "high" => HandValue::HighCard(rank),
                        "pair" => HandValue::OnePair(rank),
                        "twopair" => match rank_two {
                            None => return Err(GameError::IO),
                            Some(rank2) => HandValue::TwoPair(TwoPair::new(rank, rank2)?),
                        },
                        "triple" => HandValue::ThreeOfAKind(rank),
                        "straight" => HandValue::Straight(rank),
                        "fullhouse" => match rank_two {
                            None => return Err(GameError::IO),
                            Some(rank2) => HandValue::FullHouse(FullHouse::new(rank, rank2)?),
                        },
                        "quad" => HandValue::FourOfAKind(rank),
                        _ => return Err(GameError::IO),
                    };
                    Ok(handvalue)
                }
            }
        }
        Err(error) => Err(GameError::IO),
    }
}
