//! # Aldoc
//!
//! `aldoc` is a markup language, which takes heavy inspiration from 
//! Markdown. Its goal is to provide the simple syntax that Markdown 
//! (Pandoc's version, specifically) has, but without the quirks that 
//! it brings with it.
//!
//! ## Package
//!
//! This package's goal is to provide an abstraction for the language
//! and also a way to compile the documents to PDF. To do that the
//! following processes takes place.
//! 
//! 1. The aldoc source is parsed into a Rust abstraction.
//! 2. The abstraction is compiled to LaTeX.
//! 3. The LaTeX code is compiled to PDF via LatexMk.
//!
//! **LatexMK is required in order to use the compiler.**
//!
//! ## Syntax 
//! 
//! The syntax of aldoc is still *WIP*, I have not decided yet which syntax will be
//! most ergonomic, but still, the one used for testing temporarily is the 
//! following:
//! 
//! - Paragraphs are spaced with a blank line between them. **This example cannot be
//! shown on the Rust doc because it doesn't let me (yet again Markdown!)**
//! - Unnumbered lists can be written with the `-` or the `+` character.
//! 	```
//! 	- Alement
//! 	- Belement
//! 	- Celement
//! 	```
//! - Enumerated lists can be written:
//! 	- With numbers:
//! 		```
//! 		1. Alement
//! 		2. Belement
//! 		3. Celement
//! 		```
//! 	The symbol after the number (terminator), can be any of `.`, `-`, or `)`.
//! - Bold text is written with asterisks around it.
//! 	```
//! 	Normal text is written *until the asterisks come around*.
//! 	```
//!

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
    #[error("Error parsing document")]
    ParseError(String),
    #[error("Error compiling document")]
    CompilationError, // unimplemented
    #[error("Error exporting to PDF: {0}")]
    PdfError(PdfError)
}
impl<'s> From<ParseError<'s>> for AldocError {
    fn from(e: ParseError) -> AldocError {
        AldocError::ParseError(e.to_string())
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
}
