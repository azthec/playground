use anyhow::Result;
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use ratatui::{
  prelude::{Alignment, Constraint, CrosstermBackend, Frame, Layout, Terminal},
  style::Stylize,
  widgets::{block, Block, Borders, Clear, Paragraph},
};
use std::io::stderr;
use tictactoe::widgets::Board;

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
  let [left, right] = Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(45)])
    .margin(2)
    .areas(frame.size());
  let game_block = Block::default()
    .title(block::Title::from("Tictactoe").alignment(Alignment::Left))
    .borders(Borders::ALL)
    .light_yellow();
  let help_block = Block::default()
    .title(block::Title::from("Help").alignment(Alignment::Left))
    .borders(Borders::ALL)
    .light_yellow();
  frame.render_widget(Clear, frame.size());
  frame.render_widget(game_block, left);
  frame.render_widget(help_block, right);

  let [game_area, _] = Layout::vertical([Constraint::Percentage(100), Constraint::Min(20)])
    .margin(1)
    .areas(left);
  frame.render_widget(
    Board {
      content: "eeeeee".to_string(),
    },
    game_area,
  );
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
