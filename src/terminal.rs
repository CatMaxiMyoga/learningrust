use std::io::{self, Error, Result, Write};
use std::cell::Cell;

#[derive(PartialEq)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

#[derive(PartialEq)]
pub enum WrapMode {
  Same,
  Next,
  Previous,
}

pub struct Terminal {
  caret_pos: Cell<(u16, u16)>,
}

impl Terminal {
  pub fn new() -> Self {
    Terminal {
      caret_pos: Cell::new((0, 0)), // 0, 0 means no idea
    }
  }

  pub fn clear_sceen(&self) -> Result<()> {
    self.execute_ansi("\x1B[2J")
  }

  pub fn clear_line(&self) -> Result<()> {
    self.execute_ansi("\x1B[2K")
  }

  pub fn move_dir(&self, direction: Direction) -> Result<()> {
    let (w, h) = self.get_term_size()?;
    match direction {
      Direction::Up => {
        if self.caret_pos.get().1 != 1 {
          self.caret_pos.set((
            self.caret_pos.get().0,
            self.caret_pos.get().1 - 1,
          ))
        }
        self.execute_ansi("\x1B[A")
      },
      Direction::Down => {
        if self.caret_pos.get().1 != h {
          self.caret_pos.set((
            self.caret_pos.get().0,
            self.caret_pos.get().1 + 1,
          ))
        }
        self.execute_ansi("\x1B[B")
      },
      Direction::Right => {
        if self.caret_pos.get().0 != w {
          self.caret_pos.set((
            self.caret_pos.get().0 + 1,
            self.caret_pos.get().1,
          ))
        }
        self.execute_ansi("\x1B[C")
      },
      Direction::Left => {
        if self.caret_pos.get().0 != 1 {
          self.caret_pos.set((
            self.caret_pos.get().0 - 1,
            self.caret_pos.get().1,
          ))
        }
        self.execute_ansi("\x1B[D")
      },
    }
  }

  pub fn move_dir_wrap(
    &self,
    direction: Direction,
    col_mode: WrapMode,
    row_mode: WrapMode,
  ) -> Result<()> {
    if self.caret_pos == Cell::new((0u16, 0u16)) {
      return Err(Error::other(
        "Caret Position not yet tracked. Reset to home using home() to start\
        tracking",
      ));
    }

    let (width, height) = self.get_term_size()?;
    let wrap_col = match col_mode {
      WrapMode::Same => self.caret_pos.get().0,
      WrapMode::Next => {
        if self.caret_pos.get().0 == width - 1 {
          1
        } else {
          self.caret_pos.get().0 + 1
        }
      }
      WrapMode::Previous => {
        if self.caret_pos.get().0 == 1 {
          width
        } else {
          self.caret_pos.get().0 - 1
        }
      }
    };
    let wrap_row = match row_mode {
      WrapMode::Same => self.caret_pos.get().1,
      WrapMode::Next => {
        if self.caret_pos.get().1 == height - 1 {
          1
        } else {
          self.caret_pos.get().1 + 1
        }
      }
      WrapMode::Previous => {
        if self.caret_pos.get().1 == 1 {
          height
        } else {
          self.caret_pos.get().1 - 1
        }
      }
    };

    match (self.caret_pos.get().0, self.caret_pos.get().1, &direction) {
      (1, _, Direction::Left) => self.goto(width, wrap_row),
      (w, _, Direction::Right) if w == width => self.goto(1, wrap_row),
      (_, 1, Direction::Up) => self.goto(wrap_col, height),
      (_, h, Direction::Down) if h == height => self.goto(wrap_col, 1),
      _ => self.move_dir(direction),
    }
  }

  pub fn goto(&self, x: u16, y: u16) -> Result<()> {
    if x < 1 || y < 1 {
      return Err(Error::other(
        "Invalid position, x and y cannot be less than 1",
      ));
    }
    let (width, height) = self.get_term_size()?;
    if x > width || y > height {
      return Err(Error::other("Invalid position, x or y are out of bounds"));
    }
    self.caret_pos.set((x, y));
    self.execute_ansi(&format!("\x1B[{y};{x}H"))
  }

  pub fn get_term_size(&self) -> Result<(u16, u16)> {
    termsize::get()
      .map(|size| (size.cols, size.rows))
      .ok_or_else(|| Error::other("Failed to get terminal size"))
  }

  pub fn home(&self) -> Result<()> {
    self.execute_ansi("\x1B[H")?;
    self.caret_pos.set((1, 1));
    Ok(())
  }
  
  pub fn reset(&self) -> Result <()> {
    self.clear_sceen()?;
    self.home()
  }
  
  pub fn set_fg(&self, color: u8) -> Result<()> {
    self.execute_ansi(&format!("\x1B[38;5;{color}m"))
  }

  pub fn set_bg(&self, color: u8) -> Result<()> {
    self.execute_ansi(&format!("\x1B[48;5;{color}m"))
  }

  pub fn reset_colors(&self) -> Result<()> {
    self.execute_ansi("\x1B[0m")
  }

  fn execute_ansi(&self, command: &str) -> Result<()> {
    print!("{command}");
    io::stdout().flush()
  }
}
