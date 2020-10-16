use aldoc::{AldocError, parse, save_as_pdf};
use aldoc::{IntoLatex, IntoPrintable, Compiler};
use std::fs;
use std::path::PathBuf;
use clap::Clap;

/// A markup language compiler.
#[derive(Clap)]
#[clap(version = "0.1", author = "Aldo Acevedo <aldoacevedo1993@gmail.com>")]
struct Aldoc {
    /// Input .md file
    input: PathBuf,
    #[clap(subcommand)]
    subcommand: Subcommand,
}
#[derive(Clap)]
enum Subcommand {
    Compile(Compile),
    Print(Print),
}
/// Compiles the document to a PDF.
#[derive(Clap)]
struct Compile {
    /// PDF output path (defaults to the input file with a pdf extension).
    output: Option<PathBuf>,
    /// Determines if the output file will be overwritten
    #[clap(short, long)]
    force: bool,
}
/// Prints the document to STDOUT.
#[derive(Clap)]
struct Print {
    /// Should the document be printed in LaTeX format?
    #[clap(short, long)]
    latex: bool
}

fn main() -> Result<(), AldocError> {
    let aldoc: Aldoc = Aldoc::parse();
    let text = fs::read_to_string(&aldoc.input)?;

    if text.is_empty() {
        return Err(AldocError::EmptyDocument);
    }

    let document = parse(&text)?;

    match aldoc.subcommand {
        Subcommand::Compile(c) => {
            let output = if let Some(o) = c.output {
                o
            } else {
                let mut path = aldoc.input.clone();
                path.set_extension("pdf");
                path
            };

            save_as_pdf(&document, output, c.force)?;
        }
        Subcommand::Print(print) => {
            let text = if print.latex {
                IntoLatex.compile(&document)
            } else {
                IntoPrintable.compile(&document)
            };
            println!("{}", text);
        }
    }
    // TODO: print `Display` text instead of `Debug` text
    Ok(())
}

