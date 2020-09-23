//! ***(some of the following information may be subject to change)***
//! # Aldoc
//! 
//! *aldoc* is a markup language, which takes heavy inspiration from Markdown. Its
//! main goal is to provide the beauty and control of LaTeX documents with 
//! Markdown's pretty syntax.
//! 
//! Another one of its goals is to remove the quirks that Markdown brings with its 
//! original design, such as:
//! 
//! - Different versions and editions of Markdown which mildly in syntax, thus 
//! making it unreliable for posting on multiple platforms (GitHub Markdown, 
//! original HTML Markdown, Pandoc Markdown, etc.)
//! - Markdown was not intended for use outside of small documents, such as
//! small notes or READMEs (this one), which led to decisions that impacted the
//! ergonomics in the syntax (pandoc filters) and ended up in the creation of the 
//! different variants.
//! 
//! ## Syntax 
//! 
//! The syntax of aldoc is still *WIP*: what syntax will be the most beneficious 
//! has not yet been decided, but still, the one used for testing temporarily is 
//! the following:
//! 
//! - Paragraphs are spaced with a blank line between them. (this example 
//! cannot be shown on the Rustdoc)
//! - Unnumbered lists can be written with the `-` or the `+` character.
//! 	```
//! 	- Alement
//! 	- Belement
//! 	- Celement
//! 	```
//! - Enumerated lists can be written in many ways. Aldoc's design allow you to use
//! any combination of enumerator (`1`, `a`, `III`) and symbol (`.`, `)`, `-`).
//! 	- With numbers:
//! 		```
//! 		1. Alement
//! 		2. Belement
//! 		3. Celement
//! 		```
//! 	- With letters (uppercase or lowercase):
//! 		```
//! 		a) Alement
//! 		b) Belement
//! 		c) Celement
//! 		```
//! 	- With roman numbers (uppercase or lowercase):
//! 		```
//! 		I- Alement
//! 		II- Belement
//! 		III- Celement
//! 		```
//! - Bold text is written with asterisks around it.
//! 	```
//! 	Normal text is written *until the asterisks come around*.
//! 	```
//! ## Tool
//! 
//! As a tool, library and Cargo package, it provides an abstraction for the 
//! language and also a way to compile the documents to PDF. To do that the 
//! following processes takes place:
//! 
//! 1. The aldoc source is parsed into a Rust abstraction.
//! 2. The abstraction is compiled to LaTeX.
//! 3. The LaTeX code is compiled to PDF via LatexMk (this step is planned to 
//! change)
//! 
//! 
//! ### Usage
//! 
//! To actually compile the document, you only need to provide it with the input
//! file path (.ald) and the output pdf path, like this:
//! 
//! ```shell
//! $ aldoc doc.ald compile out.pdf
//! ```
//! 
//! You may even omit the output file, in which case, aldoc will output a pdf
//! with the same name as the document.
//! 
//! ```shell
//! $ aldoc doc.ald compile # outputs pdf as "doc.pdf"
//! ```

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
