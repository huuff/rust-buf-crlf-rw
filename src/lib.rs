use std::io::{Read, BufReader, BufRead, Result};
use std::str;

pub trait ReadCrlfLine {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize>;
}

impl<T: Read> ReadCrlfLine for BufReader<T> {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize> {
        let fill_buf = self.fill_buf()?;

        if fill_buf.is_empty() {
            return Ok(0);
        }

        let mut consumed = 0;
        while fill_buf[consumed] != b'\r' && fill_buf[consumed+1] != b'\n' {
           consumed += 1; 
        }

        // TODO: Please do not unwrap
        buffer.push_str(str::from_utf8(&fill_buf[..consumed]).unwrap());

        consumed += 2;
        self.consume(consumed);

        return Ok(consumed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        // ARRANGE
        let mut line = String::with_capacity(15);
        let mut buf_reader = BufReader::new(
            "This is a text\r\nwith two lines".as_bytes()
        );

        // ACT
        let result = buf_reader.read_crlf_line(&mut line);

        // ASSERT
        assert_eq!(line, "This is a text");
        assert_eq!(result.unwrap(), 16);
    }

    #[test]
    fn two_lines() {
        // ARRANGE
        let mut line = String::with_capacity(32);
        let mut buf_reader = BufReader::new(
            "This is a text\r\nwith three lines\r\nseparated by crlf".as_bytes()
        );

        let first_result = buf_reader.read_crlf_line(&mut line);
        assert_eq!(line, "This is a text");
        assert_eq!(first_result.unwrap(), 16);

        line.clear();

        let second_result = buf_reader.read_crlf_line(&mut line);
        assert_eq!(line, "with three lines");
        assert_eq!(second_result.unwrap(), 18);
    }
}
