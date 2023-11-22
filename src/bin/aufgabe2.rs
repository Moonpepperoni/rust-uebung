use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO den Dateipfad über die Konsole einlesen.
    let path = ???;

    // Markdown-File vollständig in einen String einlesen
    let markdown = fs::read_to_string(path)?;

    // TODO Einen neuen Parser mit dem markdown erstellen
    let parser = Parser::new(???);
    let parsed = parser.parse_all();

    // Eine NEUE html-Datei öffnen
    let mut html = File::create("out.html")?;
    

    // TODO Jede Datenstruktur aus dem Stream durchgehen und den zugehörigen HTML Code in die geöffnete Datei schreiben
    write_as_html(&mut html, parsed)?;
    Ok(())
}




// TODO Datenstrukturen definieren

// Welche Elemente fehlen noch?
enum MarkdownElem {
    Heading(Heading),
    UnorderedList(UnorderedList),
    ???,
}

struct UnorderedList {
    items: Vec<String>,
}

enum Heading {
    ???,
    ???,
    ???,
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
            // schau den current_char an und ruf die richtige methode auf
        }
        return self.elems;
    }

    fn parse_heading(&mut self) {
      // lese ein wie viele # es gibt
        let count_head = self
            .markdown
            .chars()
            .skip(self.offset)
            .take_while(|c| *c == '#')
            .count()
            .min(3);
        self.offset += count_head;
        // lies rest der Zeile und speichere Heading
        ???
    }

    // diese methode ist schon fertig
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
        
    }

    // manche Zeilen sind leer daher Option
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
        // entscheide basierend auf dem markdownelement wie geschrieben wird
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

// TODO ergänze die Funktionssignatur
fn write_list_html(???) -> Result<(), io::Error> {
    // schreibe die Liste zu html
    Ok(())
}

fn write_inner_text_html(out: &mut File, t: &String) -> Result<(), io::Error> {
    write!(out, "{}", t)?;
    Ok(())
}

fn write_heading_html(out: &mut File, heading: &Heading) -> Result<(), io::Error> {
    match heading {
        // entscheide wie man ein Heading in html übersetzt
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

