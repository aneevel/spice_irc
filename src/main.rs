use std::io;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor::{self, position},
    event::{self, Event, KeyCode, KeyEvent, read, DisableMouseCapture, EnableMouseCapture},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

const MENU: &str = r#"Spice IRC Client

    Controls:

    - 'q' - quit application
    - 'c' - proceed to main application

    "#;

fn run<W>(w: &mut W) -> io::Result<()>
where 
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;


    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        for line in MENU.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match read_char()? {
            'c' => proceed(w)?,
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }
            _ => {}
        };
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> io::Result<(u16, u16)> {
    terminal::size()
}

pub fn proceed<W>(w: &mut W) -> std::io::Result<()>
where 
    W: io::Write,
{
    queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;
    execute!(w, EnableMouseCapture)?;

    loop {
        let event = read()?;

        println!("Event::{:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }

    execute!(w, DisableMouseCapture)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
