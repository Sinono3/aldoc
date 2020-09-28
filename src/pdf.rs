use crate::parse::*;
use crate::compiler::*;

use std::path::PathBuf;
use std::{fs, io};
use tectonic::{Error as TectonicError, latex_to_pdf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("File writing error {0}")]
    IoError(io::Error),
    #[error("Error while compiling with Tectonic {0}")]
    TectonicError(TectonicError),
    #[error("Output file {0} already exists")]
    FileExists(PathBuf)
}
impl From<std::io::Error> for PdfError {
    fn from(err: std::io::Error) -> PdfError {
        PdfError::IoError(err)
    }
}

impl From<TectonicError> for PdfError {
    fn from(err: TectonicError) -> PdfError {
        PdfError::TectonicError(err)
    }
}

/// Saves a document as a PDF with Tectonic.
pub fn save_as_pdf<T>(document: &Document, out: T) -> Result<(), PdfError> 
where T: Into<PathBuf> 
{
    let compiled = IntoLatex.compile(&document);
    let out = out.into(); // pdf file output

    if out.exists() {
        return Err(PdfError::FileExists(out));
    }
    let pdf: Vec<u8> = latex_to_pdf(compiled)?;
    fs::write(&out, pdf)?;
    Ok(())
}
