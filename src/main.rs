mod terminal;
use std::io::{self, Write};
use terminal::Terminal;

const BACKGROUND_COLOR: u8 = 0;
type DrawMatrix = Vec<(u16, u16, Vec<u8>)>;
type DrawMatrixArg = [(u16, u16, Vec<u8>)];

fn draw_pixel(
  x: u16,
  mut y: u16,
  color: u8,
  term: &Terminal,
  width: u16,
  height: u16,
) {
  for _ in 0..height {
    term.goto(x, y).unwrap();
    term.set_bg(color).unwrap();
    print!("{}", " ".repeat(width as usize));
    io::stdout().flush().unwrap();
    term.reset_colors().unwrap();
    y += 1;
  }
}

fn draw(values: &DrawMatrixArg, term: &Terminal) {
  let mut x = 1u16;
  let mut y = 1u16;
  let (term_width, term_height) = term.get_term_size().unwrap();

  term.clear_screen().unwrap();

  for &(width, height, ref colors) in values {
    for color in colors {
      draw_pixel(
        x,
        y,
        *color,
        term,
        width,
        height,
      );

      x += width;
    }

    draw_pixel(x, y, BACKGROUND_COLOR, term, term_width - x + 1, height);

    y += height;
    x = 1;
  }

  while y <= term_height {
    draw_pixel(1, y, BACKGROUND_COLOR, term, term_width, 1);
    y += 1;
  }
}

fn main() {
  let values: DrawMatrix = vec![
    (1, 1, vec![]),
    (11, 2, (0..=15).collect()),
    (1, 1, vec![]),
    (5, 2, (16..=51).collect()),
    (5, 2, (52..=87).collect()),
    (5, 2, (88..=123).collect()),
    (5, 2, (124..=159).collect()),
    (5, 2, (160..=195).collect()),
    (5, 2, (196..=231).collect()),
    (1, 1, vec![]),
    (8, 2, (232..=255).collect()),
  ];

  let term = Terminal::new();
  draw(&values, &term);
}
