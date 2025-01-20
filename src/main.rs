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
    let fps: i32 = get_fps();
    // Display fps for development purposes
    draw_text(&format!("FPS: {}", fps), 20.0, 20.0, 30.0, WHITE);
    next_frame().await
  }
}
