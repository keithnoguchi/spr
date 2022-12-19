use std::io::{BufRead, Result, Write};

pub fn grep<R, W>(target: &str, reader: &mut R, writer: &mut W) -> Result<()>
where
    R: BufRead + ?Sized,
    W: Write + ?Sized,
{
    for line in reader.lines() {
        let line = line?;
        if line.contains(target) {
            writer.write_fmt(format_args!("{}\n", line))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::grep;
    use std::io::{BufRead, Cursor, Result};

    #[test]
    fn test_grep() {
        let target = "line";
        let mut reader = Cursor::new("line one\nline two\nline three\n");
        let mut writer = Vec::new();

        let result = grep(target, &mut reader, &mut writer);
        assert!(result.is_ok());

        let got = Cursor::new(writer)
            .lines()
            .collect::<Result<Vec<String>>>()
            .unwrap();
        assert_eq!(got.len(), 3);
        assert_eq!(got[0], "line one");
        assert_eq!(got[1], "line two");
        assert_eq!(got[2], "line three");
    }
}
