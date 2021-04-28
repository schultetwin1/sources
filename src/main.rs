use anyhow::Result;

use std::path::PathBuf;

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
        println!("{}", file.path.display());
    }
    Ok(())
}
