use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO den Dateipfad über die Konsole einlesen.
    let path = ???;

    // Markdown-File vollständig in einen String einlesen
    let markdown = fs::read_to_string(path)?;

    // TODO Den String Stück für Stück durchgehen und in Datenstrukturen aus Rust einlesen
    let elements = parse(???)?;

    // Eine NEUE html-Datei öffnen
    let mut html = File::create("out.html")?;

    // TODO Jede Datenstruktur aus dem Stream durchgehen und den zugehörigen HTML Code in die geöffnete Datei schreiben
    write_as_html(&mut html, elements)?;
    Ok(())
}

// TODO Datenstrukturen definieren

// Überlegen Sie sich eine sinnvolle Struktur
// Welche Elemente können vorkommen?
// Evtl müssen Sie structs und enums verschachteln
enum MarkdownElem {

}


// Gehen Sie die Eingabe-Datei Zeile für Zeile durch
// Anhand der ersten paar Zeichen einer Zeile kann man erkennen, um welches Markdown-Element es sich handelt
// Schreiben Sie dann jeweils eine passende Funktion, die ein Markdown-Element in eine Rust-Datenstruktur speichert
fn parse(markdown: &str) -> Vec<MarkdownElem> {
  let mut elems = vec![];
  
  elems
}


// Schreiben des HTMLs
fn write_as_html(out: &mut File, parsed: &Vec<MarkdownElem>) -> Result<(), io::Error> {
  // DEN HTML Prelude starten
  html_start(out)?;
  // Gehen Sie hier jedes MarkdownElement durch und generieren Sie in der Datei den passenden HTML-Code


  // den prelude schliessen
  html_end(out)?;
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
