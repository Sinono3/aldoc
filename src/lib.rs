mod indent;
mod parse;
mod compiler;
mod pdf;

pub use crate::{
    pdf::{PdfError, save_as_pdf},
    compiler::{Compiler, IntoLatex, IntoPrintable},
    parse::{Document, parse}
};

use parse::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AldocError {
    #[error("Error parsing document: {0}")]
    ParseError(ParseError),
    #[error("Error compiling document")]
    CompilationError, // unimplemented
    #[error("Error exporting to PDF: {0}")]
    PdfError(PdfError)
}
impl From<ParseError> for AldocError {
    fn from(e: ParseError) -> AldocError {
        AldocError::ParseError(e)
    }
}
impl From<PdfError> for AldocError {
    fn from(e: PdfError) -> AldocError {
        AldocError::PdfError(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::AldocError;
    use crate::pdf::save_as_pdf;
    use crate::parse::parse;
    use std::path::PathBuf;

    // This function is for tests
    fn quick_pdf<T>(p: T) -> Result<(), AldocError>
        where T: Into<PathBuf> 
    {
        let text = std::fs::read_to_string(p.into()).unwrap();
        let document = parse(&text)?;
        save_as_pdf(&document, "test/test.pdf")?; // some tests don't need to be saved to a pdf
        Ok(())
    }
    fn quick_parse<T>(p: T) -> Result<(), AldocError>
        where T: Into<PathBuf> 
    {
        let text = std::fs::read_to_string(p.into()).unwrap();
        parse(&text)?;
        Ok(())
    }
    #[test]
    fn hyphen() {
        assert!(quick_parse("docs/hyphen_in_text.ald").is_ok());
    }
    #[test]
    fn incorrect_headings() {
        assert!(quick_parse("docs/incorrect_headings.ald").is_err());
    }
}
