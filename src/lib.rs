use std::io::{Read, Write, BufReader, BufRead, BufWriter};
use std::str;
use std::error::Error;

pub trait ReadCrlfLine {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize, Box<dyn Error>>;
}

impl<T: Read> ReadCrlfLine for BufReader<T> {
    fn read_crlf_line(&mut self, buffer: &mut String) -> Result<usize, Box<dyn Error>> {
        let fill_buf = self.fill_buf()?;

        if fill_buf.is_empty() {
            return Ok(0);
        }

        let mut consumed = 0;
        // XXX: Spaghetti conditional
        while consumed < fill_buf.len() // Buffer is not finished
            && (consumed == fill_buf.len()-1 // If it's the end of the buffer, just finish reading
                                             // it
            || (fill_buf[consumed] != b'\r' || fill_buf[consumed+1] != b'\n')) // Ensure CRLF
                                                                               // hasn't been found
                                                                               // yet 
        {
            consumed += 1; 
        }

        buffer.push_str(str::from_utf8(&fill_buf[..consumed])?);

        if consumed < fill_buf.len() {
            // Found a CRLF before the buffer ends, so we manually consume it
            consumed += 2;
        }
        self.consume(consumed);

        return Ok(consumed);
    }
}

pub trait WriteCrlfLine {
    fn write_crlf_line(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>>;
}

impl<T: Write> WriteCrlfLine for BufWriter<T> {
    fn write_crlf_line(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
       self.write_all(buf)?;
       self.write(b"\r\n")?;
       self.flush()?;
       Ok(())
    } 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_single_line() {
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
    fn read_two_lines() {
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

    #[test]
    fn read_reads_to_end_when_theres_no_crlf() {
        let mut line = String::with_capacity(32);
        let mut buf_reader = BufReader::new("this is a line with no crlf".as_bytes());

        let result = buf_reader.read_crlf_line(&mut line);

        assert_eq!(line, "this is a line with no crlf");
        assert_eq!(result.unwrap(), 27);
    }

    #[test]
    fn read_multiline_without_crlf_as_single_line() {
        let mut line = String::with_capacity(128);
        let mut buf_reader = BufReader::new(
            "this is\nseveral lines\nseparated by newlines\nwithout carriage return".as_bytes()
        );

        let result = buf_reader.read_crlf_line(&mut line);

        assert_eq!(line, "this is\nseveral lines\nseparated by newlines\nwithout carriage return");
        assert_eq!(result.unwrap(), 67);
    }

    #[test]
    fn write_single_line() {
        let target: Vec<u8> = Vec::new();
        let mut buf_writer = BufWriter::new(target);

        let result = buf_writer.write_crlf_line("single line".as_bytes());

        assert!(result.is_ok());
        
        let bytes = buf_writer.into_inner().unwrap();
        assert_eq!("single line\r\n", String::from_utf8(bytes).unwrap())
    }

    #[test]
    fn write_multiple_lines() {
        let target: Vec<u8> = Vec::new();
        let mut buf_writer = BufWriter::new(target);

        buf_writer.write_crlf_line("first line".as_bytes()).unwrap();
        buf_writer.write_crlf_line("second line".as_bytes()).unwrap();

        let bytes = buf_writer.into_inner().unwrap();
        assert_eq!("first line\r\nsecond line\r\n", String::from_utf8(bytes).unwrap());
    }
}
