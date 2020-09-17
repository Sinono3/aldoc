use crate::parse::*;
use crate::compiler::*;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("Couldn't call `latexmk`: {0}")]
    IoError(io::Error),
    #[error("LatexMk error: {0}")]
    LatexMkError(String)
}
impl From<std::io::Error> for PdfError {
    fn from(err: std::io::Error) -> PdfError {
        PdfError::IoError(err)
    }
}

fn latexmk_pdf(path: &Path) -> Result<(), PdfError> {
    let status = Command::new("latexmk")
        .arg("-silent")
        .arg("-cd")
        .arg("-pdf")
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(PdfError::LatexMkError(format!("Error compiling LaTeX to PDF")));
    }
    Ok(())
}
fn latexmk_cleanup(path: &Path) -> Result<(), PdfError> {
    let status = Command::new("latexmk")
        .arg("-silent")
        .arg("-cd")
        .arg("-c")
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(PdfError::LatexMkError(format!("Error cleaning up work directory")));
    }
    Ok(())
}

/// Saves a document as a PDF with LatexMk.
pub fn save_as_pdf<T>(document: &Document, out: T) -> Result<(), PdfError> 
where T: Into<PathBuf> 
{
    let compiled = IntoLatex.compile(&document);
    let out = out.into().with_extension("tex");
    // testing: println!("{}", compiled);

    fs::write(&out, compiled)?;
    latexmk_pdf(&out)?;
    latexmk_cleanup(&out)?;
    fs::remove_file(&out)?;
    Ok(())
}
