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
    terminal::enable_raw_mode()?;
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
        println!("lkjasdlfjas");

        // foo.write(b"ls\n").expect("error writing to child stdin");
        // foo.flush().expect("error flushing child stdin");

        let mut buf = Vec::new();
        foo.read(&mut buf)?;

        let stdout = io::stdout();
        let handle = stdout.lock();
        let mut writer = BufWriter::new(handle);
        writer.write_all(&buf)?;
        writer.flush()?;
    }
}
