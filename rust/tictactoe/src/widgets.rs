use ratatui::prelude::{self, Buffer, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;

pub struct Board {
  pub content: String,
}

impl Widget for Board {
  fn render(self, area: Rect, buf: &mut Buffer) {
    buf.set_string(
      area.left(),
      area.top(),
      &self.content,
      Style::default().fg(Color::Green),
    );
  }
}
