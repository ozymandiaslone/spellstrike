use crate::utils::*;
use crate::game::*;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum WizardAction {
  Fly,
  Walk,
  Idle,
  Shield,
  Zap,
}

pub struct Wizard {
  pub direction: Direction,
  pub action: WizardAction,
  pub animations: HashMap<WizardAction, HashMap<Direction, Animation>>,
  pub pos: Vec2,
  pub scale: f32
}
impl DynamicEntity for Wizard{
  fn draw(&self) {
    if let Some(animation_map) = self.animations.get(&self.action) {
      if let Some(animation) = animation_map.get(&self.direction) {
        let frame_x = animation.current_frame as f32 * animation.frame_size.x;
        let src = Rect::new(
          frame_x,
          0.,
          animation.frame_size.x,
          animation.frame_size.y
        );
        animation.sheet.set_filter(FilterMode::Nearest);

        draw_texture_ex(
          &animation.sheet,
          self.pos.x,
          self.pos.y,
          WHITE,
          DrawTextureParams {
            source: Some(src),
            dest_size: Some(Vec2 {
              x: animation.frame_size.x * self.scale,
              y: animation.frame_size.y * self.scale,
            }),
            ..Default::default()
          },
        )
      }
    }
  }

  fn update(&mut self) -> Option<Game> {
    let dt = get_frame_time();
    if is_key_down(KeyCode::W) {
      self.pos.y -= 10.;
    }
    if is_key_down(KeyCode::A) {
      self.pos.x -=10.;
      self.direction = Direction::Left;
    }
    if is_key_down(KeyCode::S) {
      self.pos.y += 10.;
    }
    if is_key_down(KeyCode::D) {
      self.pos.x += 10.;
      self.direction = Direction::Right;
    }

    if let Some(mut animation_map) = self.animations.get_mut(&self.action) {
      if let Some(mut animation) = animation_map.get_mut(&self.direction) {
        animation.timer += dt;

        if animation.timer >= animation.frame_duration {
          animation.timer = 0.;
          animation.current_frame = (animation.current_frame + 1) % animation.n_frames;
        }
      }
    }
    None
  }
}

pub async fn setup_default_wizard() -> Wizard {
  let wiz_fly_left: Texture2D = load_texture("assets/wizard-fly-left.png").await.unwrap();
  let mut map: HashMap<Direction, Texture2D> = HashMap::new();
  map.insert(Direction::Left, wiz_fly_left);
  let mut animations = HashMap::new();
  let mut fly_map: HashMap<Direction, Animation> = HashMap::new();

  let fly_left_animation = Animation {
    sheet: load_texture("assets/wizard-fly-left.png").await.unwrap(),
    current_frame: 0,
    frame_duration: 0.19,
    frame_size: vec2(40., 40.),
    n_frames: 11,
    timer: 0.,
  };

  let fly_right_animation = Animation {
    sheet: load_texture("assets/wizard-fly-right.png").await.unwrap(),
    current_frame: 0,
    frame_duration: 0.16,
    frame_size: vec2(40., 40.),
    n_frames: 11,
    timer: 0.
  };

  fly_map.insert(Direction::Left, fly_left_animation);
  fly_map.insert(Direction::Right, fly_right_animation);
  animations.insert(WizardAction::Fly, fly_map);

  Wizard {
    direction: Direction::Left,
    action: WizardAction::Fly,
    pos: Vec2::new(0., 0.),
    animations: animations,
    scale: 10.,
  }
}
