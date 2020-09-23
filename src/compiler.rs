use crate::parse::{Document, Block, List, ListToken, ListItem};

// made this a trait just for more extensibility
// in the future
/// Compiles a document to a different format.
pub trait Compiler {
    fn compile(&mut self, document: &Document) -> String; 
}

/// Compiles into LaTeX.
pub struct IntoLatex;

impl IntoLatex {
    fn push_text(buf: &mut String, text: &str) {
        let mut in_bold = false;
        // should also check for italics

        for ch in text.chars() {
            if ch == '*' {
                let bf = if !in_bold {
                    r#"\textbf{"#
                } else {
                    "}"
                };
                in_bold = !in_bold;
                buf.push_str(bf);
                continue;
            }
            buf.push(ch);
        }
    }
    fn print_paragraph(buf: &mut String, paragraph: &str) {
        IntoLatex::push_text(buf, paragraph);
        buf.push_str("\n\n");
    }
    fn print_list(buf: &mut String, list: &List) {
        let environment = match list.token {
            ListToken::Dot => "itemize",
            ListToken::Numbered => "enumerate",
            _ => "itemize",
        };
        buf.push_str(r#"\begin{"#);
        buf.push_str(environment);
        buf.push_str("}");
        buf.push('\n');

        for item in &list.vec { 
            buf.push_str(r#"\item "#);
            IntoLatex::push_text(buf, &item.text);
            buf.push('\n');

            if let Some(list) = &item.list {
                IntoLatex::print_list(buf, &list);
            }
        }

        buf.push_str(r#"\end{"#);
        buf.push_str(environment);
        buf.push_str("}");
        buf.push_str("\n\n");
    }
    fn print_block(buf: &mut String, part: &Block) {
        match part {
            Block::Heading(level, title) => { 
                let latex_heading = match level {
                    // hmm, shouldn't section be a better 
                    // fit for level 0 instead of chapter?
                    0 => r#"\chapter{"#, 
                    1 => r#"\section{"#,
                    2 => r#"\subsection{"#,
                    3 => r#"\subsubsection{"#,
                    4 => r#"\paragraph{"#,
                    5 => r#"\subparagraph{"#,
                    _ => r#"\heading{"#,
                };
                buf.push_str(latex_heading);
                buf.push_str(&title);
                buf.push_str("}\n\n");
            },
            Block::Paragraph(p) => IntoLatex::print_paragraph(buf, &p),
            Block::List(list) => IntoLatex::print_list(buf, &list),
        }
    }
}
impl Compiler for IntoLatex {
    fn compile(&mut self, document: &Document) -> String {
        let mut string = String::new();

        string.push_str(
r#"\documentclass{article}

\begin{document}
"#);

        for block in &document.blocks {
            IntoLatex::print_block(&mut string, &block);
        }

        string.push_str(r#"\end{document}"#);
        string
    }
}

/// Compiles into terminal friendly text.
pub struct IntoPrintable;

impl IntoPrintable {
    fn print_paragraph(buf: &mut String, par: &str) {
        buf.push_str(par);
        buf.push_str("\n\n");
    }
    fn print_list(buf: &mut String, list: &List, indent: usize) {
        for _ in 0..indent {
            buf.push_str("  ");
        }
        let get_token = |i: usize| 
            match list.token {
                ListToken::Dot => String::from("-"),
                ListToken::Numbered => format!("{}.", i + 1),
                _ => String::from("-")
            };


        let iter = list.vec.iter().enumerate();
        for (index, item) in iter {
            buf.push_str(&get_token(index));
            buf.push(' ');
            buf.push_str(&item.text);
            buf.push('\n');

            if let Some(list) = &item.list {
                IntoPrintable::print_list(buf, &list, indent + 1);
            }
        }
        buf.push_str("\n\n");
    }
    fn print_block(buf: &mut String, part: &Block) {
        match part {
            Block::Heading(level, title) => { 
                for _ in 0..*level {
                    buf.push('#');
                }
                buf.push(' ');
                buf.push_str(&title);
                buf.push_str("\n\n");
            },
            Block::Paragraph(p) => Self::print_paragraph(buf, p),
            Block::List(list) => IntoPrintable::print_list(buf, &list, 0),
        }
    }
}
impl Compiler for IntoPrintable {
    fn compile(&mut self, document: &Document) -> String {
        let mut string = String::new();

        for part in &document.blocks {
            Self::print_block(&mut string, &part);
        }
        string
    }
}
