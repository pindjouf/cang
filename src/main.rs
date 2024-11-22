use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::io;
use std::io::stdout;

fn main() -> io::Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(Paragraph::new("yoooooo, press 'q' to quit."), area);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
