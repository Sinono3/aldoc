use regex::Regex;
use crate::indent::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    SyntaxError(SyntaxError)
}
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Skipped a heading level. Went from level {0} to level {1}")]
    SkippedHeadingLevel(usize, usize)
}
type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Part {
    Heading(String, usize),
    Paragraph(Paragraph),

    // unimplemented:
    // Image(PathBuf)            (png, jpg or svg?)
    // Table(csv?)
    // Diagram(custom support?)
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Paragraph {
    Text(String),
    List(Vec<Paragraph>)
}

struct ParseContext {
    current_heading: usize
}
fn parse_paragraph(part: &str) -> Paragraph {
    let mut buffer = String::new();
    let paragraph: Paragraph;

    if is_part_list(part) {
        let mut vec = Vec::new();
        let mut skip_to = 0;

        for (i, ch) in part.char_indices() {
            // used for skipping already processed blocks
            // like indented lists
            if i < skip_to {
                continue;
            }
            // appends normal elements to vec
            let left = part.len() - (i + 1);

            if left > 1 {
                if part[i..i+2] == *"- " { //ch == '-' {
                    if !buffer.is_empty() {
                        vec.push(parse_paragraph(&buffer.trim()));
                        buffer.clear();
                    }
                    continue;
                }
                // checks for indented lists
                // uses recursiveness
                if part[i..i+2] == *"\t-" {
                    if !buffer.is_empty() {
                        vec.push(parse_paragraph(&buffer.trim()));
                        buffer.clear();
                    }

                    let over = part[i..].find("\n-").map(|n| i + n + 1).unwrap_or(part.len());
                    let p = &part[i..over]; // until indented list is over
                    let squashed = squash_indent(p);
                    vec.push(parse_paragraph(&squashed));
                    skip_to = over;
                    continue;
                }
            }
            buffer.push(ch);
        }
        if !buffer.is_empty() {
            vec.push(parse_paragraph(&buffer.trim()));
            buffer.clear();
        }
        paragraph = Paragraph::List(vec);
    } else {
        for ch in part.chars() {
            if ch == '\t' { continue; }
            if ch == '\n' {
                buffer.push(' ');
                continue;
            }
            buffer.push(ch);
        }
        buffer = buffer.trim().to_string();
        paragraph = Paragraph::Text(buffer);
    } 
    return paragraph;
}

fn parse_part(context: &mut ParseContext, part: &str) -> ParseResult<Part> {
    if part.starts_with("#") {
        let mut level = 0;

        for ch in part.chars() {
            if ch != '#' {
                break;
            }
            level += 1;
        }
        if level - context.current_heading > 1 {
            return Err(ParseError::SyntaxError(SyntaxError::SkippedHeadingLevel(context.current_heading, level)));
        }
        context.current_heading = level;

        Ok(Part::Heading(part[level + 1..].to_string(), level))
    } else {
        Ok(Part::Paragraph(parse_paragraph(part)))
    }
}

/// An Aldoc document abstraction.
pub struct Document {
    pub parts: Vec<Part>
}

/// Parses raw Aldoc text into a document abstraction.
pub fn parse(text: &str) -> ParseResult<Document> {
    let mut context = ParseContext {
        current_heading: 0
    };
    let mut parts = Vec::new();
    let part_strings = into_parts(text);

    for part_str in part_strings {
        parts.push(parse_part(&mut context, &part_str)?);
    }
    Ok(Document { parts })
}

fn into_parts(text: &str) -> Vec<String> {
    let separator = "\n\n";
    text.split(separator).map(|s| s.to_string()).collect()
}
fn is_part_list(part: &str) -> bool {
    let regex = Regex::new(r"^\s*- ").unwrap(); // TODO: make regex lazy static
    regex.captures_iter(part).count() > 0
}
