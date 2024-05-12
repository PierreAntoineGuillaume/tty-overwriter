use crate::prelude::AnsiSeq;
use std::io::Write;

#[derive(Debug, Default)]
pub struct Body {
    buffer: Vec<usize>,
}

const CLEAR_TIL_EOL: AnsiSeq = AnsiSeq::ClearCursorToEndOfLine;
const CLEAR_TIL_EOF: AnsiSeq = AnsiSeq::ClearCursorToEndOfScreen;
const JUMP_AT_BEGINNING: AnsiSeq = AnsiSeq::AbsoluteMove { horizontal: 0 };

impl Body {
    pub fn overwrite<T, Writer>(&mut self, new_text: &T, write: &mut Writer, available_width: usize) -> std::io::Result<()>
    where
        Writer: Write,
        T: ToString,
    {
        let new_text = new_text.to_string();
        let mut next_buffer = if self.buffer.is_empty() {
            vec![]
        } else {
            Vec::with_capacity(self.buffer.capacity())
        };
        let mut symbols = vec![];

        if new_text.is_empty() {
            write!(symbols, "{CLEAR_TIL_EOL}")?;
            next_buffer.push(0);
        } else {
            for (line_no, line) in new_text.lines().enumerate() {
                next_buffer.push(line.len());
                if line_no > 0 {
                    symbols.push(b'\n');
                }
                symbols.write_all(line.as_bytes())?;
                write!(symbols, "{CLEAR_TIL_EOL}")?;
            }
        }

        match self.guess_previous_body_height(available_width) {
            1 => {
                write!(write, "{JUMP_AT_BEGINNING}")?;
            }
            lines if lines > 1 => {
                let movement = AnsiSeq::MoveLines {
                    down: 0,
                    up: (lines - 1) as u16,
                };
                write!(write, "{JUMP_AT_BEGINNING}{movement}")?;
            }
            _ => {}
        }

        write!(symbols, "{CLEAR_TIL_EOF}")?;
        write.write_all(&symbols)?;
        self.buffer = next_buffer;
        Ok(())
    }

    fn guess_previous_body_height(&self, available_width: usize) -> usize {
        let mut lines = 0;
        for line in &self.buffer {
            lines+= line/available_width + 1;
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fmt::Write;

    #[test]
    fn initial() -> Result<(), Box<dyn Error>> {
        let mut body = Body::default();
        let mut buf = Vec::new();
        body.overwrite(&"content", &mut buf, 80)?;

        let string = String::from_utf8(buf.clone())?;
        assert_eq!("content\\e[0K\\e[0J", string);

        body.overwrite(&"content + content", &mut buf, 80)?;
        let string = String::from_utf8(buf.clone())?;
        assert_eq!("content\\e[0K\\e[0J\\e[0Gcontent + content\\e[0K\\e[0J", string);

        body.overwrite(&"none", &mut buf, 80)?;
        let string = String::from_utf8(buf.clone())?;

        let mut expected = String::new();
        expected.write_str("content")?; // first content
        expected.write_str("\\e[0K")?; // clear til EOL
        expected.write_str("\\e[0J")?; // clear til EOF
        expected.write_str("\\e[0G")?; // go far left
        expected.write_str("content + content")?; // second content
        expected.write_str("\\e[0K")?; // clear til EOL
        expected.write_str("\\e[0J")?; // clear til EOF
        expected.write_str("\\e[0G")?; // go far left
        expected.write_str("none")?; // third content
        expected.write_str("\\e[0K")?; // clear til EOL
        expected.write_str("\\e[0J")?; // clear til EOF

        assert_eq!(expected, string);

        body.overwrite(&"", &mut buf, 80)?;

        expected.write_str("\\e[0G")?; // go far left
        expected.write_str("\\e[0K")?; // clear til EOL
        expected.write_str("\\e[0J")?; // clear til EOF

        let string = String::from_utf8(buf.clone())?;

        assert_eq!(expected, string,);

        Ok(())
    }
}
