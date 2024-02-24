use anyhow::Result;
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  prelude::{CrosstermBackend, Frame, Terminal},
  style::Stylize,
  widgets::Paragraph,
};
use std::io::{stderr, stdout};

struct State {
  counter: i64,
  quit: bool,
}

fn startup() -> Result<()> {
  enable_raw_mode()?;
  execute!(stderr(), EnterAlternateScreen)?;
  Ok(())
}

fn shutdown() -> Result<()> {
  execute!(stderr(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

fn draw(state: &State, frame: &mut Frame) {
  frame.render_widget(
    Paragraph::new(format!("Counter: {0}", state.counter))
      .white()
      .on_blue(),
    frame.size(),
  )
}

fn update(state: &mut State) -> Result<()> {
  if event::poll(std::time::Duration::from_millis(16))? {
    if let Key(key) = event::read()? {
      if key.kind == event::KeyEventKind::Press {
        match key.code {
          Char('j') => state.counter += 1,
          Char('k') => state.counter -= 1,
          Char('q') => state.quit = true,
          _ => {}
        }
      }
    }
  }
  Ok(())
}

fn run() -> Result<()> {
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  let mut state = State {
    counter: 0,
    quit: false,
  };

  loop {
    t.draw(|f| {
      draw(&state, f);
    })?;

    update(&mut state)?;

    if state.quit {
      break;
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  startup()?;
  let status = run();
  shutdown()?;
  status?;
  Ok(())
}
