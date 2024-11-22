use std::collections::HashMap;
use macroquad::prelude::*;



#[derive(Eq, Hash, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

#[derive(Eq, Hash, PartialEq)]
enum Action {
  Fly,
  Walk,
  Idle,
  Shield,
  Zap,
}

struct Animation {
  sheet: Texture2D,
  current_frame: usize,
  n_frames: usize,
  frame_size: Vec2,
  frame_duration: f32,
  timer: f32
}

struct Wizard {
  direction: Direction,
  action: Action,
  animations: HashMap<Action, HashMap<Direction, Animation>>,
  pos: Vec2,
  scale: f32
}

impl Wizard {
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
  fn update(&mut self) {
    let dt = get_frame_time();
    if let Some(mut animation_map) = self.animations.get_mut(&self.action) {
      if let Some(mut animation) = animation_map.get_mut(&self.direction) {
        animation.timer += dt;

        if animation.timer >= animation.frame_duration {
          animation.timer = 0.;
          animation.current_frame = (animation.current_frame + 1) % animation.n_frames;
        }
      }
    }
  }
}

#[macroquad::main("SpellStrike!")]
async fn main() {
  
  let mut wizard = setup_wizard().await;

  loop {
    clear_background(RED);
    draw_text("SPELLSTRIKE!", screen_width() / 2., screen_height() / 2., screen_width() / 20., WHITE);
    wizard.update();
    wizard.draw();
    next_frame().await
  }
}

async fn setup_wizard() -> Wizard {
  let wiz_fly_left: Texture2D = load_texture("assets/wizard-fly-left.png").await.unwrap();
  let mut map: HashMap<Direction, Texture2D> = HashMap::new();
  map.insert(Direction::Left, wiz_fly_left);
  let mut animations = HashMap::new();
  let mut fly_map: HashMap<Direction, Animation> = HashMap::new();

  let fly_left_animation = Animation {
    sheet: load_texture("assets/wizard-fly-left.png").await.unwrap(),
    current_frame: 0,
    frame_duration: 0.16,
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
  animations.insert(Action::Fly, fly_map);

  Wizard {
    direction: Direction::Left,
    action: Action::Fly,
    pos: Vec2::new(0., 0.),
    animations: animations,
    scale: 10.,
  }
}
