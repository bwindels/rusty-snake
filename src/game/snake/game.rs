use output::Screen;
use game::Game;
use geom::{Size, Point};
use input::Key;
use std::time::Duration;
use super::level::Level;
use super::field::Field;
use super::{RelativeDirection, StepResult};
use super::scoring::Scoring;
use random::Random;
use std::fmt::Write;

fn key_to_relative_direction(key_option: Option<Key>) -> RelativeDirection {
  match key_option {
    Some(key) => match key {
      Key::Left => RelativeDirection::Left,
      Key::Right => RelativeDirection::Right,
      _ => RelativeDirection::Straight
    },
    None => RelativeDirection::Straight
  }
}

pub struct SnakeGame<R> {
  level: Level,
  game_over: bool,
  random: R,
  scoring: Scoring,
  score_buffer: String
}

impl<R: Random> SnakeGame<R> {

  pub fn new(size: Size, random: R) -> SnakeGame<R> {
    let field = Field::new(size);
    let level = Level::new(field);
    
    let mut game = SnakeGame {
      level: level,
      game_over: false,
      random: random,
      scoring: Scoring::new(),
      score_buffer: String::new()
    };

    game.level.reset(&mut game.random);

    return game;
  }

  fn draw_game_state<S: Screen>(&mut self, screen: &mut S) {
    if self.game_over {
      screen.draw_text(Point::new(8, 5), "GAME OVER! HIT ESC TO LEAVE THE GAME");
    }
    else {
      let score = self.scoring.score();
      let multiplier = self.scoring.multiplier();
      {
        let mut buffer = &mut self.score_buffer;
        buffer.clear();
        write!(buffer, " Score: {0:.>#10} x{1: <#4}", score, multiplier);
        screen.draw_text(Point::new(2, 0), buffer.as_str());
      }
    }
  }

}

impl<R: Random> Game for SnakeGame<R> {

  fn update(&mut self, input: Option<Key>, passed_time: Duration) -> bool {

    if self.game_over {
      return false;
    }

    let rel_dir = key_to_relative_direction(input);
    let step_result = self.level.step(rel_dir, &mut self.random);

    match step_result {
      StepResult::Collision => {
        self.game_over = true;
      },
      StepResult::CaughtApple => {
        self.scoring.adjust_for_apple();
      }
      StepResult::Moved => {
        self.scoring.adjust_for_move();
      }
    };

    return true;
  }

  fn max_idle_time(&self) -> Option<Duration> {
    if self.game_over {
      None
    }
    else {
      Some(Duration::from_millis(200))
    }
  }

  fn full_draw<S: Screen>(&mut self, screen: &mut S) {
    self.level.full_draw(screen);
    self.draw_game_state(screen);
  }

  fn incremental_draw<S: Screen>(&mut self, screen: &mut S) {
    self.level.incremental_draw(screen);
    self.draw_game_state(screen);
  }

}
