use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  prelude::{CrosstermBackend, Stylize, Terminal},
  widgets::Paragraph,
};

use std::io::Result;
use std::io::{stderr, stdout};

fn main() -> Result<()> {
  enable_raw_mode()?;
  execute!(stderr(), EnterAlternateScreen)?;
  let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
  terminal.clear()?;

  // mutable state
  let mut counter = 0;

  loop {
    // draw
    let _ = terminal.draw(|frame| {
      let area = frame.size();
      frame.render_widget(
        Paragraph::new(format!("Counter: {counter}"))
          .white()
          .on_blue(),
        area,
      );
    })?;

    // handle event
    if event::poll(std::time::Duration::from_millis(16))? {
      if let event::Event::Key(key) = event::read()? {
        if key.kind == crossterm::event::KeyEventKind::Press {
          match key.code {
            crossterm::event::KeyCode::Char('j') => counter += 1,
            crossterm::event::KeyCode::Char('k') => counter -= 1,
            crossterm::event::KeyCode::Char('q') => break,
            _ => {}
          }
        }
      }
    }
  }

  execute!(stderr(), LeaveAlternateScreen)?;
  disable_raw_mode()?;

  Ok(())
}
