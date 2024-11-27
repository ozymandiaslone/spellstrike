use crate::utils::*;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Game {
  pub dynamic_lobby_entities: Vec<Box<dyn DynamicEntity>>,
  pub state: GameState,
}

impl Game {
  pub async fn new_default() -> Game {
    let mut entities = setup_dynamic_lobby_entities().await;
    Game {
      dynamic_lobby_entities: entities,
      state: GameState::Lobby,
    }
  }
  pub fn tick(&mut self) {

    //TODO match self.state in order to do the right stuff
    match self.state {
      GameState::Lobby => {
        for entity in self.dynamic_lobby_entities.iter_mut() {
          entity.update();
          entity.draw();
        }
      }
      GameState::Searching => {

      }
      GameState::Playing => {

      }
      GameState::Quitting => {

      }
      _ => {}
    }
  }
}

#[derive(Debug)]
enum GameState {
  Lobby,
  Searching,
  Playing,
  Quitting
}
