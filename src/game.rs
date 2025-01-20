use crate::utils::*;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Game {
  pub dynamic_lobby_entities: Vec<Box<dyn DynamicEntity>>,
  pub dynamic_game_entities: Vec<Box<dyn DynamicEntity>>,
  pub state: GameState,
}

impl Game {
  pub async fn new_default() -> Game {
    let mut entities = setup_dynamic_lobby_entities().await;
    Game {
      dynamic_lobby_entities: entities,
      dynamic_game_entities: Vec::new(),
      state: GameState::Lobby,
    }
  }
  pub fn tick(&mut self) {
    let mut optional_game: Option<Game> = None; 
    //TODO match self.state in order to do the right stuff
    match self.state {
      GameState::Lobby => {
        // uhh so in order for buttons and stuff 
        // be able to change the state of
        // the game, I just ended up
        // returning a whole new game
        // with the correct state changes
        for entity in self.dynamic_lobby_entities.iter_mut() {
        if let Some(new_game) = entity.update() {
          optional_game = Some(new_game);
          break;
        }
          entity.draw();
        }
        if let Some(game) = optional_game {
          *self = game;
        }
      }
      GameState::Searching => {

      }
      GameState::Playing => {

      }
      _ => {}
    }
  }
}

#[derive(Debug)]
pub enum GameState {
  Lobby,
  Playing,
  Searching,
}
