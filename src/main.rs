use anyhow::Result;

use std::path::{Component, Path, PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to scan for sources
    input: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verose: u8,

    /// Output original (raw) source paths in the binary (do not attempt to normalize paths)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    raw: bool,

    /// Only output files which exist on disk
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    exists: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input_file = cli.input;
    let input_file = PathBuf::from(input_file);
    let input_file = std::fs::File::open(input_file)?;
    let normalize = cli.raw;
    let exists = cli.exists;

    let files = compiledfiles::parse(input_file)?;
    for file in files {
        let mut path = file.path;
        if let Ok(cwd) = std::env::current_dir() {
            if normalize && path.is_absolute() && path.starts_with(&cwd) {
                path = path.strip_prefix(&cwd).unwrap().to_path_buf();
            }
        }
        if normalize  {
            path = normalize_path(&path);
        }
        if !exists || path.exists() {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// copied from https://github.com/rust-lang/cargo/blob/2e4cfc2b7d43328b207879228a2ca7d427d188bb/src/cargo/util/paths.rs#L65-L90
// both projects are MIT https://github.com/rust-lang/cargo/blob/master/LICENSE-MIT
// for std impl progress see rfc https://github.com/rust-lang/rfcs/issues/2208
// replace this once that lands
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}