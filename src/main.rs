use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use plexrs::pty::Pty;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::exit;
use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    let size = terminal::size().expect("no terminal size");
    println!("{:?}", size);
    // terminal::enable_raw_mode()?;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    loop {
        let mut foo = Pty::spawn("/bin/ls");
        if poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                }) => exit(0),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }

        let mut buf = Vec::new();
        foo.read_to_end(&mut buf)?;

        writer.write_all(&buf)?;
        writer.flush()?;
    }
}
