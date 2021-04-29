use anyhow::Result;

use std::path::{Component, Path, PathBuf};

fn main() -> Result<()> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Tool to list sources used to compile binary")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity")
        )
        .arg(
            clap::Arg::with_name("input")
                .help("File to scan for sources")
                .required(true)
                .index(1)
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let input_file = PathBuf::from(input_file);
    let input_file = std::fs::File::open(input_file)?;

    let files = compiledfiles::parse(input_file)?;
    for file in files {
        let mut path = file.path;
        if let Ok(cwd) = std::env::current_dir() {
            if path.is_absolute() && path.starts_with(&cwd) {
                path = path.strip_prefix(&cwd).unwrap().to_path_buf();
            }
        }
        path = normalize_path(&path);
        println!("{}", path.display());
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