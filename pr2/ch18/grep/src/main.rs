use grep::grep;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let mut args = env::args().skip(1);

    // get target string
    let target = match args.next() {
        Some(target) => target,
        None => {
            eprintln!("grep TARGET [FILE]...");
            process::exit(1);
        }
    };

    // no pipe support yet...
    let mut writer = io::stdout();

    let files: Vec<_> = args.map(PathBuf::from).collect();
    if files.is_empty() {
        let mut reader = io::stdin().lock();
        if let Err(e) = grep(&target, &mut reader, &mut writer) {
            eprintln!("grep: {e}");
            process::exit(1);
        }
    } else if let Err(e) = files
        .into_iter()
        .map(|path| grep_file(&target, &path, &mut writer))
        .collect::<io::Result<Vec<()>>>()
    {
        eprintln!("grep: {e}");
        process::exit(1);
    }
}

// i wish i can convert this to the adaptor abvoe...
fn grep_file<W>(target: &str, path: &Path, writer: &mut W) -> io::Result<()>
where
    W: Write + ?Sized,
{
    let mut reader = BufReader::new(File::open(path)?);
    grep(target, &mut reader, writer)
}
