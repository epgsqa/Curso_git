pub mod tui {
  use std::{io, thread, time::Duration};
  use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction}, 
    Terminal, Frame}; 
  use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
  };

  pub fn terminal() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
      ui(f);
      // let size = f.size();
      // let block = Block::default()
      //   .title("Block")
      //   .borders(Borders::ALL);
      // f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
  }

  fn ui<B: Backend>(f: &mut Frame<B>) {
    let chuncks = Layout::default()
      .direction(Direction::Vertical)
      .margin(1)
      .constraints([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10)
      ].as_ref())
      .split(f.size());
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
      f.render_widget(block, chuncks[1]);
  }
}