use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Part {
    Heading(String),
    Paragraph(Paragraph),

    // unimplmented:
    // Image(PathBuf)            (png, jpg or svg?)
    // Table(csv?)
    // Diagram(custom support?)
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum Paragraph {
    Text(String),
    List(Vec<Paragraph>)
}
fn into_parts(text: &str) -> Vec<String> {
    let separator = "\n\n";
    text.split(separator).map(|s| s.to_string()).collect()
}
fn is_part_list(part: &str) -> bool {
    let regex = Regex::new(r"^\s*- ").unwrap();
    regex.captures_iter(part).count() > 0
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
            if ch == '-' {
                if !buffer.is_empty() {
                    vec.push(parse_paragraph(&buffer.trim()));
                    buffer.clear();
                }

                //let line = &part[i..part[i..].find('\n').map(|n| i + n).unwrap_or(part.len())];
                //println!("element {}: {}", vec.len(), line);
                continue;
            }
            // checks for indented lists
            // uses recursiveness
            if ch == '\t' {//part[i..i+2] == *"\t-" {
                if !buffer.is_empty() {
                    vec.push(parse_paragraph(&buffer.trim()));
                    buffer.clear();
                }

                let over = part[i..].find("\n-").map(|n| i + n + 1).unwrap_or(part.len());

                let p = &part[i..over]; // until indented list is over
                let flattened = remove_indent(p, base_indent(p));
                println!("{}", flattened);

                vec.push(parse_paragraph(&flattened));
                skip_to = over;
                println!("Skipping to: {}", skip_to);
                continue;
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
fn parse(text: &str, start_indent: u32) -> Vec<Paragraph> {
    let mut paragraphs = Vec::new();

    let parts = into_parts(text);
    //println!("{:?}", parts);
    
    for part in parts {
        let paragraph = parse_paragraph(&part);
        paragraphs.push(paragraph);
    }
    paragraphs
}
fn remove_indent(text: &str, amount: u32) -> String {
    let mut string = String::new();

    for line in text.lines() {
        let mut to_remove = amount;
        
        for c in line.chars() {
            if c == '\t' && to_remove > 0 {
                to_remove -= 1;
                continue;
            }
            string.push(c);
        }
        string.push('\n');
    }
    return string;
}
fn base_indent(text: &str) -> u32 {
    let mut base_indent = 100; // TODO: wtf, why 100

    for line in text.lines() {
        let mut line_indent = 0;

        for c in line.chars().filter(|c| *c == '\t') {
            line_indent += 1;
        }

        if base_indent > line_indent {
            base_indent = line_indent;
        }
    }
    return base_indent;
}
use std::fs;

fn print_paragraph(par: &Paragraph, indent: u32, in_list: bool) {
    for _ in 0..indent {
        print!("  ");
    }
    match par {
        Paragraph::Text(t) => if in_list {
            println!("- {}", t);
        } else {
            println!("{}\n", t);
        },
        Paragraph::List(l) => {
            for e in l { 
                print_paragraph(e, indent+1, true);
            }
            println!();
        }
    }
}
fn main() {
    let text = fs::read_to_string("docs/document_test2.ald").unwrap();
    
    println!("\nText:===========================================================\n{}\n", text);
    //println!("base indent {}", base_indent(&text));

    println!("\nParsing...======================================================\n");
    let parts = parse(&text, 0);
    println!("\nFinished=========================================================\n");

    for paragraph in parts {
        print_paragraph(&paragraph, 0, false);
    }
}
