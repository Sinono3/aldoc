use crate::parse::*;
use crate::compiler::*;

use std::path::PathBuf;
use std::{fs, io};
use tectonic::{Error as TectonicError, latex_to_pdf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("File writing error {0}")]
    IoError(#[from] io::Error),
    #[error("Tectonic compilation error: {0}")]
    TectonicError(#[from] TectonicError),
    #[error("Output file {0} already exists")]
    FileExists(PathBuf)
}

/// Compiles a document to binary PDF data via Tectonic.
pub fn compile_to_pdf(document: &Document) -> Result<Vec<u8>, TectonicError> {
    let compiled = IntoLatex.compile(&document);
    Ok(latex_to_pdf(compiled)?)
}
/// Exports a document to a PDF file via Tectonic.
pub fn save_as_pdf<T>(document: &Document, out: T, overwrite: bool) -> Result<(), PdfError> 
where T: Into<PathBuf> 
{
    let out = out.into(); // pdf file output

    if !overwrite && out.exists() {
        return Err(PdfError::FileExists(out));
    }

    let pdf = compile_to_pdf(&document)?;
    fs::write(&out, pdf)?;
    Ok(())
}
