use std::convert::TryFrom;
use crate::parse::{Document, Block, List, ListToken};
use numerals::roman::Roman;

static ALPHABET_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'B', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];
static ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

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
        // TODO: This is not foolproof
        // should also check for italics
        let mut in_bold = false;

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
        if in_bold {
            buf.push('}');
        }
    }
    fn print_paragraph(buf: &mut String, paragraph: &str) {
        IntoLatex::push_text(buf, paragraph);
        buf.push_str("\n");
    }
    fn print_list(buf: &mut String, list: &List) {
        let environment = match list.token {
            ListToken::Unnumbered => "itemize",
            _ => "enumerate",
        };
        buf.push_str(r#"\begin{"#);
        buf.push_str(environment);
        buf.push_str("}");

        let extra = match list.token {
            ListToken::Unnumbered => "",
            ListToken::Numbered => "",
            ListToken::Alphabetical(true)   => "[label=\\Alph*.]",
            ListToken::Alphabetical(false)  => "[label=\\alph*.]",
            ListToken::Roman(true)   => "[label=\\Roman*.]",
            ListToken::Roman(false)  => "[label=\\roman*.]",
        };
        buf.push_str(extra);

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
        buf.push('\n');
    }
    fn print_block(buf: &mut String, part: &Block) {
        match part {
            Block::Heading(level, title) => { 
                let latex_heading = match level {
                    // FIXME: hmm, shouldn't section be a better 
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
                buf.push_str("}\n");
            },
            Block::Paragraph(p) => IntoLatex::print_paragraph(buf, &p),
            Block::List(list) => IntoLatex::print_list(buf, &list),
        }
        buf.push_str("\n");
    }
}
impl Compiler for IntoLatex {
    fn compile(&mut self, document: &Document) -> String {
        let mut string = String::new();

        string.push_str(
r#"\documentclass{article}

\usepackage{enumitem}

\begin{document}
"#);

        for block in &document.blocks {
            IntoLatex::print_block(&mut string, &block);
        }

        string.push_str(r#"\end{document}"#);
        string
    }
}

fn roman(number: i16, uppercase: bool) -> String {
    let roman = Roman::from(number);

    if uppercase {
        format!("{:X}", roman)
    } else {
        format!("{:x}", roman)
    }
}
/// Compiles into terminal friendly text.
pub struct IntoPrintable;

impl IntoPrintable {
    fn print_paragraph(buf: &mut String, par: &str) {
        buf.push_str(par);
        buf.push_str("\n");
    }
    fn print_list(buf: &mut String, list: &List, indent: usize) {
        for _ in 0..indent {
            buf.push_str("  ");
        }
        // index starts on 1
        let get_token = |i: i16| 
            match list.token {
                ListToken::Unnumbered => String::from("-"),
                ListToken::Numbered => format!("{}.", i),
                ListToken::Alphabetical(u)   => {
                    // FIXME: Should this happen?
                    // What are alternatives to this?
                    
                    if i > 26 {
                        panic!("List bigger than alphabet size");
                    }
                    if u {
                        format!("{}.", ALPHABET_UPPER[(i - 1) as usize])
                    } else {
                        format!("{}.", ALPHABET_LOWER[(i - 1) as usize])
                    }
                }
                ListToken::Roman(u) => format!("{}.", roman(i, u)),
            };


        let iter = list.vec.iter().enumerate();
        for (index, item) in iter {
            // limitation on list size
            let index = i16::try_from(index).expect("List bigger than 32768 elements");

            buf.push_str(&get_token(index + 1));
            buf.push(' ');
            buf.push_str(&item.text);
            buf.push('\n');

            if let Some(list) = &item.list {
                IntoPrintable::print_list(buf, &list, indent + 1);
            }
        }
    }
    fn print_block(buf: &mut String, part: &Block) {
        match part {
            Block::Heading(level, title) => { 
                for _ in 0..*level {
                    buf.push('#');
                }
                buf.push(' ');
                buf.push_str(&title);
                buf.push_str("\n");
            },
            Block::Paragraph(p) => Self::print_paragraph(buf, p),
            Block::List(list) => IntoPrintable::print_list(buf, &list, 0),
        }
        buf.push_str("\n");
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
