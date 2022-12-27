use std::fs;
use std::io::{self, Result};
use std::path::Path;

pub fn copy_dir_to<P, Q>(src: P, dst: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry in src.read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

fn copy_to<P, Q>(src: P, src_type: &fs::FileType, dst: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("don't know how to copy: {}", src.display(),),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::copy_dir_to;

    #[test]
    fn test_copy_dir_to() {
        let src = "src";
        let dst = "tmp";

        assert!(copy_dir_to(src, dst).is_ok());
    }
}
