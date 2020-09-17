use crate::parse::{Document, Part, Paragraph};

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
    fn print_paragraph(buf: &mut String, par: &Paragraph, in_list: bool) {
        match par {
            Paragraph::Text(t) => if in_list {
                buf.push_str(r#"\item "#);
                Self::push_text(buf, t);
                buf.push('\n');
            } else {
                Self::push_text(buf, t);
                buf.push_str("\n\n");
            },
            Paragraph::List(l) => {
                buf.push_str(r#"\begin{itemize}"#);
                buf.push('\n');
                for e in l { 
                    Self::print_paragraph(buf, e, true);
                }
                buf.push_str(r#"\end{itemize}"#);
                buf.push_str("\n\n");
            }
        }
    }
    fn print_part(buf: &mut String, part: &Part) {
        match part {
            Part::Heading(h, i) => { 
                let latex_heading = match i {
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
                buf.push_str(&h);
                buf.push_str("}\n\n");
            },
            Part::Paragraph(p) => Self::print_paragraph(buf, p, false)
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

        for part in &document.parts {
            Self::print_part(&mut string, &part);
        }

        string.push_str(r#"\end{document}"#);
        string
    }
}

/// Compiles into terminal friendly text.
pub struct IntoPrintable;

impl IntoPrintable {
    fn print_paragraph(buf: &mut String, par: &Paragraph, indent: u32, in_list: bool) {
        for _ in 0..indent {
            buf.push_str("  ");
        }
        match par {
            Paragraph::Text(t) => if in_list {
                buf.push_str("- ");
                buf.push_str(&t);
                buf.push('\n');
            } else {
                buf.push_str(&t);
                buf.push_str("\n\n");
            },
            Paragraph::List(l) => {
                for e in l { 
                    Self::print_paragraph(buf, e, indent+1, true);
                }
                buf.push('\n');
            }
        }
    }
    fn print_part(buf: &mut String, part: &Part) {
        match part {
            Part::Heading(h, i) => { 
                for _ in 0..*i {
                    buf.push('#');
                }
                buf.push(' ');
                buf.push_str(&h);
                buf.push_str("\n\n");
            },
            Part::Paragraph(p) => Self::print_paragraph(buf, p, 0, false)
        }
    }
}
impl Compiler for IntoPrintable {
    fn compile(&mut self, document: &Document) -> String {
        let mut string = String::new();

        for part in &document.parts {
            Self::print_part(&mut string, &part);
        }
        string
    }
}
