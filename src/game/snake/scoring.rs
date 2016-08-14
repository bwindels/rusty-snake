use super::StepResult;
use std::cmp::max;

#[derive(Copy, Clone)]
pub struct Scoring {
  combo_multiplier: u8,
  score: u32
}

impl Scoring {
  pub fn new() -> Scoring {
      Scoring {
          combo_multiplier: 0,
          score: 0
      }
  }

  pub fn adjust_for_move(&mut self) {
    if self.combo_multiplier > 0 {
      self.combo_multiplier -= 1;
    }
  }

  pub fn adjust_for_apple(&mut self) {
    self.score += 100 * self.multiplier();
    self.combo_multiplier = 30;
  }

  pub fn score(self) -> u32 {
    self.score
  }

  pub fn multiplier(self) -> u32 {
    let level_multiplier = ((self.score as f32).log10() - 1f32).max(1f32) as u32;
    let combo_multiplier = max(1, self.combo_multiplier);

    return level_multiplier * combo_multiplier as u32;
  }
}
