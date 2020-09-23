use aldoc::{AldocError, parse, save_as_pdf};
use aldoc::{IntoLatex, IntoPrintable, Compiler};
use std::fs;
use std::path::PathBuf;
use clap::Clap;

/// A markup language compiler.
#[derive(Clap)]
#[clap(version = "0.1", author = "Aldo Acevedo <aldoacevedo1993@gmail.com>")]
struct Aldoc {
    /// Input .ald file
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

    let text = fs::read_to_string(&aldoc.input).unwrap();
    let document = parse(&text).unwrap();


    match aldoc.subcommand {
        Subcommand::Compile(c) => {
            let output = if let Some(o) = c.output {
                o
            } else {
                let mut path = aldoc.input.clone();
                path.set_extension("pdf");
                path
            };
            println!("{:?}", &output);

            save_as_pdf(&document, output)?;
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
    Ok(())
}

