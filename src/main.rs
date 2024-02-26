use std::io::{stdout, Write};

use crossterm:: {cursor, terminal, ExecutableCommand, QueueableCommand};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut cx = 0;
    let mut cy = 0;

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;
    }

    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
