use aldoc::{AldocError, parse, save_as_pdf};
use std::fs;
use std::path::PathBuf;
use clap::Clap;

/// A markup language compiler.
#[derive(Clap)]
#[clap(version = "1.0", author = "Aldo Acevedo <aldoacevedo1993@gmail.com>")]
struct Aldoc {
    /// Input .ald file
    input: PathBuf,
    /// PDF output path
    output: Option<PathBuf>,
}

fn main() -> Result<(), AldocError> {
    let aldoc: Aldoc = Aldoc::parse();

    let text = fs::read_to_string(&aldoc.input).unwrap();
    let document = parse(&text).unwrap();
    let output = if let Some(o) = aldoc.output{
        o
    } else {
        let mut path = aldoc.input.clone();
        path.set_extension("pdf");
        path
    };
    println!("{:?}", &output);

    save_as_pdf(&document, output)?;
    Ok(())
}

