use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};
use std::io;

struct AppState {
    selected_tab: usize,
}

impl AppState {
    fn new() -> Self {
        Self { selected_tab: 1 }
    }

    fn next(&mut self) {
        if self.selected_tab < 2 {
            self.selected_tab += 1;
        } else {
            self.selected_tab = 0;
        };
    }

    fn previous(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = 2;
        };
    }
}

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|frame| {
            render(frame, &app_state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('l') | KeyCode::Right => app_state.next(),
                KeyCode::Char('h') | KeyCode::Left => app_state.previous(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let area = frame.area();
    let titles = vec!["VMs", "System", "Network"];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let block = Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Navigation bar");

    let tabs = Tabs::new(titles)
        .block(block)
        .divider("|")
        .select(app_state.selected_tab);
    frame.render_widget(tabs, layout[0]);

    let content_title = match app_state.selected_tab {
        0 => "Virtual Machines",
        1 => "System Hardening",
        2 => "Network Management",
        _ => unreachable!(),
    };

    let main = Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title(content_title);
    frame.render_widget(main, layout[1]);
}
