//! ***(some of the following information may be subject to change)***
//! # Aldoc
//! 
//! *aldoc* is a markup language with the goal of providing the beauty and control 
//! of LaTeX documents with Markdown's pretty syntax, in other words, letting you 
//! write your documents without ever needing to touch LaTeX code.
//! 
//! It does so while also trying to solve some quirks that Markdown had since its 
//! creation: its rules were never clearly established, and as more features were 
//! needed for document formatting, variants began appearing, each with their own 
//! differences.
//! 
//! The different versions and editions of Markdown vary mildly in syntax, thus 
//! making it unreliable for posting on multiple platforms (GitHub Markdown, 
//! original HTML Markdown, Pandoc Markdown, etc.)
//! 
//! ## Status
//! 
//! This project is still in its infancy (pre-alpha), and major design decisions 
//! haven't been taken yet. The goals spoken of haven't been reached yet, and 
//! features are lacking, this shouldn't be used on its current state.
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
//! 3. The LaTeX code is compiled to PDF via Tectonic.
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
use nom::Err as NomError;
use nom::error::ErrorKind;
use std::io::Error as IoError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AldocError {
    #[error("Error reading file")]
    FileError(#[from] IoError),
    #[error("Document is empty")]
    EmptyDocument,
    #[error("Error parsing document")]
    ParseError(NomError<(String, ErrorKind)>),
    #[error("Error exporting to PDF: {0}")]
    PdfError(#[from] PdfError)
}
impl From<NomError<(&str, ErrorKind)>> for AldocError {
    fn from(e: NomError<(&str, ErrorKind)>) -> AldocError {
        AldocError::ParseError(e.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::AldocError;
    use crate::pdf::save_as_pdf;
    use crate::parse::parse;
    use std::path::PathBuf;

    fn quick_pdf<T>(p: T) -> Result<(), AldocError>
        where T: Into<PathBuf> 
    {
        let text = std::fs::read_to_string(p.into()).unwrap();
        let document = parse(&text)?;
        save_as_pdf(&document, "test/test.pdf", true)?; // some tests don't need to be saved to a pdf
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
