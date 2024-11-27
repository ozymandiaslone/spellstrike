use macroquad::prelude::*;
use crate::wizard::*;
#[derive(Eq, Hash, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
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
  fn update(&mut self);
  fn draw(&self);
}

pub async fn setup_dynamic_lobby_entities() -> Vec<Box<dyn DynamicEntity>> {

  let mut entities: Vec<Box<dyn DynamicEntity>> = Vec::new();
  // TODO create and place the JoinLobbyButton()
  let mut join_button = JoinButton::new(load_texture("assets/join-button.png").await.unwrap(), vec2(0., 0.,), vec2(250., 250.));
  // TODO create and place the CreateLobbyButton()
  // TODO create and place the SettingsButton() 

/*
  let mut wizard = setup_default_wizard().await;
  entities.push(Box::new(wizard));

*/
  entities.push(Box::new(join_button));
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
}
impl CreateLobbyButton {
  pub fn new(texture: Texture2D, pos: Vec2, click_box: Vec2) -> CreateLobbyButton {
    let mut scale = 1.;
    CreateLobbyButton {
      texture,
      pos,
      click_box,
      scale
    }
  }
}
pub struct SettingsButton {
  texture: Texture2D,
  pos: Vec2,
  click_box: Vec2,
  scale: f32,
}
impl SettingsButton {
  pub fn new(texture: Texture2D, pos: Vec2, click_box: Vec2) -> SettingsButton {
    let mut scale = 1.;
    SettingsButton {
      texture,
      pos,
      click_box,
      scale
    }
  }
}
impl DynamicEntity for JoinButton{
  fn update(&mut self) {
    self.hovered = false;
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      // ok the mouse is within the click box
      self.hovered = true;
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
      }
    }
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
  fn update(&mut self) {
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      // ok the mouse is within the click box
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
      }
    }
  }
  fn draw(&self) {
  //TODO draw the button  
  }
}
impl DynamicEntity for SettingsButton{
  fn update(&mut self) {
    let (x, y) = mouse_position();
    if x >= self.pos.x && x <= (self.pos.x + self.click_box.x) && y >= self.pos.y && y <= (self.pos.y + self.click_box.y) {
      // ok the mouse is within the click box
      if is_mouse_button_down(MouseButton::Left) {
        // do whatever you wanna do upon a click 
      }
    }
  }
  fn draw(&self) {
  //TODO draw the button  
  }
}

