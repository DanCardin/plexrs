use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::process::{Command, Stdio};

pub struct Pty {
    // pub child: Child,
    reader: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,
}

impl Pty {
    pub fn spawn(shell: &str) -> Pty {
        let mut child = Command::new(&shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn subprocess");

        Self {
            writer: Box::new(BufWriter::new(child.stdin.take().unwrap())),
            reader: Box::new(BufReader::new(child.stdout.take().unwrap())),
        }
    }
}

impl Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl Write for Pty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
