mod app;

use app::run;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal);
    ratatui::restore();
    app_result
}
