use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::Constraint::{Fill, Length, Min};
use ratatui::prelude::Stylize;
use ratatui::DefaultTerminal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, List, Paragraph, Tabs},
    Frame,
};
use std::io;

struct AppState {
    selected_tab: usize,
    selected_item: usize,
    is_nav_focused: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            selected_tab: 1,
            selected_item: 1,
            is_nav_focused: true,
        }
    }

    fn toggle(&mut self) {
        if self.is_nav_focused {}
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
    let items = vec!["Create VM", "List VMs", "Edit VM"];

    let vertical = Layout::vertical([Length(3), Min(0), Length(1)]);
    let [nav_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let block = Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Navigation bar");

    let tabs = Tabs::new(titles)
        .block(block)
        .divider("|")
        .select(app_state.selected_tab);
    frame.render_widget(tabs, nav_area);

    let content_title = match app_state.selected_tab {
        0 => "Virtual Machines",
        1 => "System Hardening",
        2 => "Network Management",
        _ => unreachable!(),
    };

    let preview_title = match app_state.selected_tab {
        0 => items[0],
        1 => items[1],
        2 => items[2],
        _ => unreachable!(),
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(content_title)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .highlight_symbol(">>");
    frame.render_widget(list, left_area);

    let preview = Paragraph::new("yoo").block(
        Block::default()
            .title(preview_title)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
    );
    frame.render_widget(preview, right_area);
}
