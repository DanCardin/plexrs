// use os_pipe::pipe;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::process;

pub struct Pty {
    // child: process::Child,
    writer: Box<dyn Write>,
    reader: Box<dyn Read>,
}

impl Pty {
    pub fn spawn(shell: &str) -> Pty {
        // let (mut reader, writer) = pipe().unwrap();
        // let writer_clone = writer.try_clone().unwrap();

        let mut child = process::Command::new(&shell)
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn subprocess");

        let input = child.stdin.take().unwrap();
        let writer = Box::new(BufWriter::new(input));

        let output = child.stdout.take().unwrap();
        let reader = Box::new(BufReader::new(output));

        Self {
            // child,
            writer,
            reader,
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
