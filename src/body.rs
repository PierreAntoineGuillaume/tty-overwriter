use std::io::Write;

#[derive(Debug, Default)]
pub struct Body
where
{
    buffer: String,
}

impl Body
{
    pub fn overwrite<Writer>(&mut self, new_text: &str, write: &mut Writer)
        -> std::io::Result<()>
        where Writer: Write
    {
        write.write_all(new_text.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial() {
        let mut body = Body::default();
        let mut buf = Vec::new();
        body.overwrite("content", &mut buf).unwrap();

        let string = String::from_utf8(buf).unwrap();
        assert_eq!(string, "content");
    }
}
