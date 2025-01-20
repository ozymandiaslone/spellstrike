use macroquad::prelude::*;
use crate::wizard::*;
use crate::game::*;
#[derive(Eq, Hash, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

pub struct Complex {
  real: DBig,
  imaginary: Dbig,
}

pub struct Animation {
  pub sheet: Texture2D,
  pub current_frame: usize,
  pub n_frames: usize,
  pub frame_size: Vec2,
  pub frame_duration: f32,
  pub timer: f32
}

pub trait DynamicEntity {
  fn update(&mut self) -> Option<Game>;
  fn draw(&self);
}

pub async fn setup_dynamic_lobby_entities() -> Vec<Box<dyn DynamicEntity>> {

  let mut entities: Vec<Box<dyn DynamicEntity>> = Vec::new();

  // TODO create and place the JoinLobbyButton()
  let mut join_button = JoinButton::new(
    load_texture("assets/join-button.png").await.unwrap(),
    vec2(0., 0.,),
    vec2(250., 250.)
    );

  // TODO create and place the CreateLobbyButton()
  let mut create_lobby_button = CreateLobbyButton::new(
    load_texture("assets/join-button.png").await.unwrap(),
    vec2(250., 0.,),
    vec2(250., 250.),
  );
  // TODO create and place the SettingsButton() 
  let mut settings_button = SettingsButton::new(
    load_texture("assets/join-button.png").await.unwrap(),
    vec2(500., 0.),
    vec2(250., 250.)
  );
  entities.push(Box::new(join_button));
  entities.push(Box::new(create_lobby_button));
  entities.push(Box::new(settings_button));
  entities
}

pub struct JoinButton{
  texture: Texture2D,
  // x and y, obviously
  pos: Vec2,
  // width and height
  click_box: Vec2,
  scale: f32,
  hovered: bool,
}
impl JoinButton {
  pub fn new(texture: Texture2D, pos: Vec2, click_box: Vec2) -> JoinButton {
    let mut scale = 1.;
    JoinButton {
      texture,
      pos,
      click_box,
      scale,
      hovered: false,
    }
  }
}
pub struct CreateLobbyButton {
  texture: Texture2D,
  pos: Vec2,
  click_box: Vec2,
  scale: f32,
  hovered: bool,
}
impl CreateLobbyButton {
  pub fn new(texture: Texture2D, pos: Vec2, click_box: Vec2) -> CreateLobbyButton {
    let mut scale = 1.;
    CreateLobbyButton {
      texture,
      pos,
      click_box,
      scale,
      hovered: false,
    }
  }
}
pub struct SettingsButton {
  texture: Texture2D,
  pos: Vec2,
  click_box: Vec2,
  scale: f32,
  hovered: bool,
}
impl SettingsButton {
  pub fn new(texture: Texture2D, pos: Vec2, click_box: Vec2) -> SettingsButton {
    let mut scale = 1.;
    SettingsButton {
      texture,
      pos,
      click_box,
      scale,
      hovered: false
    }
  }
}
impl DynamicEntity for JoinButton{
  fn update(&mut self) -> Option<Game> {
    self.hovered = false;
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      // ok the mouse is within the click box
      self.hovered = true;
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
        return Some(Game {
          dynamic_lobby_entities: Vec::new(),
          dynamic_game_entities: Vec::new(),
          state: GameState::Playing
        })
      }
    }
    None
  }
  fn draw(&self) {
  //TODO draw the button  
  let mut offset = 0.;
  if self.hovered {
    offset = 250.;
  }
  draw_texture_ex(
    &self.texture,
    self.pos.x,
    self.pos.y,
    WHITE,
    DrawTextureParams {
    source: Some(Rect::new(
                0. + offset,
                0.,
                250.,
                250.,
            )),
      ..Default::default()
    })
  }
}
impl DynamicEntity for CreateLobbyButton{
  fn update(&mut self) -> Option<Game> {
    self.hovered = false;
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      self.hovered = true;
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
      }
    }
    None
  }
  fn draw(&self) {
    //TODO draw the button  
    let mut offset = 0.;
    if self.hovered {
      offset = 250.;
    }
    draw_texture_ex(
      &self.texture,
      self.pos.x,
      self.pos.y,
      WHITE,
      DrawTextureParams {
      source: Some(Rect::new(
                  0. + offset,
                  0.,
                  250.,
                  250.,
              )),
        ..Default::default()
    })
  }
}
impl DynamicEntity for SettingsButton{
  fn update(&mut self) -> Option<Game> {
    self.hovered = false;
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      self.hovered = true;
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
      }
    }
    None
  }
  fn draw(&self) {
    //TODO draw the button  
    let mut offset = 0.;
    if self.hovered {
      offset = 250.;
    }
    draw_texture_ex(
      &self.texture,
      self.pos.x,
      self.pos.y,
      WHITE,
      DrawTextureParams {
      source: Some(Rect::new(
                  0. + offset,
                  0.,
                  250.,
                  250.,
              )),
        ..Default::default()
    })
  }
}

