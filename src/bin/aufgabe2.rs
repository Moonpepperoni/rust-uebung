use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Dateipfad über die Konsole einlesen
    let path = env::args().nth(1).ok_or("Expected a path to a .md file")?;
    let content = fs::read_to_string(path)?;

    let parsed = parse(&content);

    let mut out = File::create("out.html")?;

    write_as_html(&mut out, &parsed)?;
    Ok(())
}

// Datenstrukturen definieren

enum MarkdownElem <'a> {
    Heading(Heading <'a>),
    UnorderedList(UnorderedList <'a>),
    RichText(RichText <'a>),
}

struct RichText<'a> {
    content: Vec<ModifiedText<'a>>,
}

enum ModifiedText<'a> {
    Plain(&'a str),
    Italic(&'a str),
}

struct UnorderedList <'a> {
    items: Vec<RichText<'a>>,
}

enum Heading <'a> {
    H1(RichText<'a>),
    H2(RichText<'a>),
    H3(RichText<'a>),
}

fn parse(markdown: &str) -> Vec<MarkdownElem> {
    let mut elems = vec![];
    let mut offset: usize = 0;
    while offset < markdown.len() {
        // read entire line and decide what to do
        let end_pos = markdown
            .chars()
            .skip(offset)
            .position(|c| c == '\n')
            .unwrap_or_else(|| markdown.len() - 1);
        let line = &markdown[offset..offset + end_pos + 1];
        let (read, elem) = if line.trim().is_empty() {
            offset += end_pos + 1;
            continue;
        } else if line.starts_with("#") {
            let (read, elem) = parse_heading(markdown, offset);
            (read, MarkdownElem::Heading(elem))
        } else if line.starts_with("- ") {
            let (read, elem) = parse_unordered_list(markdown, offset);
            (read, MarkdownElem::UnorderedList(elem))
        } else {
            let (read, elem) = parse_rich_text(markdown, offset);
            (read, MarkdownElem::RichText(elem))
        };
        offset += read;
        elems.push(elem);
    }
    elems
}

fn parse_rich_text(markdown: &str, offset: usize) -> (usize, RichText) {
    let mut richt_text_elems = vec![];
    let mut read = 0;
    let mut is_italic = false;
    let mut parse_begin = offset;
    for (i, c) in markdown.char_indices().skip(offset) {
        read += 1;
        match (is_italic, c) {
            (false, '*') => {
                if i != parse_begin {
                    richt_text_elems
                        .push(ModifiedText::Plain(&markdown[parse_begin..i]));
                }
                is_italic = true;
                parse_begin = i;
            }
            (true, '*') => {
                richt_text_elems.push(ModifiedText::Italic(
                    &markdown[parse_begin + 1..i],
                ));
                parse_begin = i + 1;
                is_italic = false;
            }
            (_, '\n') => {
                richt_text_elems.push(ModifiedText::Plain(&markdown[parse_begin..i]));
                break;
            }
            (_, _) if i == markdown.len() - 1 => {
                richt_text_elems.push(ModifiedText::Plain(
                    &markdown[parse_begin..i + 1],
                ));
            }
            (_, _) => {}
        }
    }
    (
        read,
        RichText {
            content: richt_text_elems,
        },
    )
}

fn parse_heading(markdown: &str, offset: usize) -> (usize, Heading) {
    let count_head = markdown
        .chars()
        .skip(offset)
        .take_while(|c| *c == '#')
        .count()
        .min(3);
    let mut read = count_head;
    let (rich_read, heading_text) = parse_rich_text(markdown, offset + count_head);
    read += rich_read;
    let level = match count_head {
        1 => Heading::H1(heading_text),
        2 => Heading::H2(heading_text),
        3 => Heading::H3(heading_text),
        _ => Heading::H3(heading_text),
    };
    (read, level)
}

fn parse_unordered_list(markdown: &str, r: usize) -> (usize, UnorderedList) {
    let mut items = vec![];
    let i = r;
    let mut read = 0;
    while i + read < markdown.len() {
        let c = markdown.chars().nth(i + read).unwrap();
        let (text_read, text) = match c {
            '-' => parse_rich_text(markdown, i + read + 1),
            _ => break,
        };
        read += text_read + 1;
        items.push(text);
    }
    (read, UnorderedList { items })
}

// Schreiben des HTMLs

fn write_as_html(out: &mut File, parsed: &Vec<MarkdownElem>) -> Result<(), io::Error> {
    // DEN HTML Prelude starten
    html_start(out)?;
    for me in parsed {
        match me {
            MarkdownElem::Heading(heading) => {
                write_heading_html(out, heading)?;
            }
            MarkdownElem::UnorderedList(list) => {
                write_list_html(out, list)?;
            }
            MarkdownElem::RichText(text) => {
                write_standalone_rich_html(out, text)?;
            }
        }
    }
    // den prelude schliessen
    html_end(out)?;
    Ok(())
}

fn write_standalone_rich_html(out: &mut File, text: &RichText) -> Result<(), io::Error> {
    write!(out, "<p>")?;
    write_inner_rich_html(out, text)?;
    writeln!(out, "</p>")?;
    Ok(())
}

fn write_list_html(out: &mut File, list: &UnorderedList) -> Result<(), io::Error> {
    writeln!(out, "<ul>")?;
    for t in &list.items {
        write!(out, "<li>")?;
        write_inner_rich_html(out, t)?;
        writeln!(out, "</li>")?;
    }
    writeln!(out, "</ul>")?;
    Ok(())
}

fn write_inner_rich_html(out: &mut File, t: &RichText) -> Result<(), io::Error> {
    for t in &t.content {
        match t {
            ModifiedText::Italic(content) => write!(out, "<em>{}</em>", content)?,
            ModifiedText::Plain(content) => write!(out, "{}", content)?,
        };
    }
    Ok(())
}

fn write_heading_html(out: &mut File, heading: &Heading) -> Result<(), io::Error> {
    match heading {
        Heading::H1(text) => {
            write!(out, "<h1>")?;
            write_inner_rich_html(out, text)?;
            writeln!(out, "</h1>")?;
        }
        Heading::H2(text) => {
            write!(out, "<h2>")?;
            write_inner_rich_html(out, text)?;
            writeln!(out, "</h2>")?;
        }
        Heading::H3(text) => {
            write!(out, "<h3>")?;
            write_inner_rich_html(out, text)?;
            writeln!(out, "</h3>")?;
        }
    }
    Ok(())
}

// Nicht verändern
fn html_start(f: &mut File) -> Result<(), io::Error> {
    writeln!(
        f,
        "<!DOCTYPE html>
  <html lang=\"en\">
  
  <head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Document</title>
  </head>
  
  <body>"
    )?;
    Ok(())
}

// Nicht verändern
fn html_end(f: &mut File) -> Result<(), io::Error> {
    writeln!(
        f,
        "</body>
  </html>"
    )?;

    Ok(())
}
