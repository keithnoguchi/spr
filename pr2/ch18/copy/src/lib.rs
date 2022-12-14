use std::io::{self, ErrorKind, Read, Write};

const BUFSIZ: usize = 8 * 1024;

pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut buf = [0; BUFSIZ];
    let mut written = 0;

    loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(n) => n,
            Err(e) if e.kind() == ErrorKind::Interrupted || e.kind() == ErrorKind::WouldBlock => {
                continue
            }
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..n])?;
        written += n as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::copy;
    use std::io::Cursor;

    #[test]
    fn test_copy() {
        let mut reader = Cursor::new(vec![0xab; 16 * 1024]);
        let mut writer = vec![0u8; 0];

        assert_eq!(writer.len(), 0);
        copy(&mut reader, &mut writer).unwrap();
        assert_eq!(writer.len(), 16 * 1024);

        assert_eq!(writer, reader.into_inner());
    }
}
