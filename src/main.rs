mod wizard;
mod utils;
mod game;
use game::*;

use macroquad::prelude::*;
#[macroquad::main("SpellStrike!")]
async fn main() {
  set_fullscreen(true);
  let mut game = Game::new_default().await;
  loop {
    clear_background(BLUE);
    game.tick();
    next_frame().await
  }
}
