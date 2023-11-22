use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Dateipfad über die Konsole einlesen
    let path = env::args().nth(1).ok_or("Expected a path to a .md file")?;
    let content = fs::read_to_string(path)?;

    let parser = Parser::new(content);
    let parsed = parser.parse_all();

    let mut out = File::create("out.html")?;

    write_as_html(&mut out, &parsed)?;
    Ok(())
}

// Datenstrukturen definieren

enum MarkdownElem {
    Heading(Heading),
    UnorderedList(UnorderedList),
    PlainText(String),
}

struct UnorderedList {
    items: Vec<String>,
}

enum Heading {
    H1(String),
    H2(String),
    H3(String),
}

struct Parser {
    offset: usize,
    markdown: String,
    elems: Vec<MarkdownElem>,
}

impl Parser {
    fn new(markdown: String) -> Parser {
        Parser {
            offset: 0,
            markdown: markdown,
            elems: Vec::new(),
        }
    }

    fn parse_all(mut self) -> Vec<MarkdownElem> {
        while self.offset < self.markdown.len() {
            let current = self.markdown.chars().nth(self.offset).unwrap();
            if current == '#' {
                self.parse_heading();
            } else if current == '-' {
                self.parse_list();
            } else {
                self.parse_plain_text();
            }
        }
        self.elems
    }

    fn parse_heading(&mut self) {
        let count_head = self
            .markdown
            .chars()
            .skip(self.offset)
            .take_while(|c| *c == '#')
            .count()
            .min(3);
        self.offset += count_head;
        let line = self.parse_line();
        match line {
            Some(heading_text) => {
                let level = match count_head {
                    1 => Heading::H1(heading_text),
                    2 => Heading::H2(heading_text),
                    3 => Heading::H3(heading_text),
                    _ => Heading::H3(heading_text),
                };
                self.elems.push(MarkdownElem::Heading(level));
            }
            None => {}
        }
    }

    fn parse_list(&mut self) {
        let mut items = vec![];
        while self.offset < self.markdown.len()
            && self.markdown.chars().nth(self.offset).unwrap() == '-'
        {
            self.offset += 1;
            let line = self.parse_line();
            match line {
                Some(text) => {
                    items.push(text);
                }
                None => {}
            }
        }
        self.elems
            .push(MarkdownElem::UnorderedList(UnorderedList { items }));
    }

    fn parse_plain_text(&mut self) {
        let line = self.parse_line();
        match line {
            Some(text) => {
                self.elems.push(MarkdownElem::PlainText(text));
            }
            None => {}
        }
    }

    fn parse_line(&mut self) -> Option<String> {
        let line: String = self
            .markdown
            .chars()
            .skip(self.offset)
            .take_while(|&c| c != '\n')
            .collect();
        self.offset += line.len() + 1;
        if line.trim().is_empty() {
            return None;
        }
        return Some(line);
    }
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
            MarkdownElem::PlainText(text) => {
                write_plain_text_html(out, text)?;
            }
        }
    }
    // den prelude schliessen
    html_end(out)?;
    Ok(())
}

fn write_plain_text_html(out: &mut File, text: &String) -> Result<(), io::Error> {
    write!(out, "<p>")?;
    write_inner_text_html(out, text)?;
    writeln!(out, "</p>")?;
    Ok(())
}

fn write_list_html(out: &mut File, list: &UnorderedList) -> Result<(), io::Error> {
    writeln!(out, "<ul>")?;
    for t in &list.items {
        write!(out, "<li>")?;
        write_inner_text_html(out, t)?;
        writeln!(out, "</li>")?;
    }
    writeln!(out, "</ul>")?;
    Ok(())
}

fn write_inner_text_html(out: &mut File, t: &String) -> Result<(), io::Error> {
    write!(out, "{}", t)?;
    Ok(())
}

fn write_heading_html(out: &mut File, heading: &Heading) -> Result<(), io::Error> {
    match heading {
        Heading::H1(text) => {
            write!(out, "<h1>")?;
            write_inner_text_html(out, text)?;
            writeln!(out, "</h1>")?;
        }
        Heading::H2(text) => {
            write!(out, "<h2>")?;
            write_inner_text_html(out, text)?;
            writeln!(out, "</h2>")?;
        }
        Heading::H3(text) => {
            write!(out, "<h3>")?;
            write_inner_text_html(out, text)?;
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
