use std::fs::{self, File};
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // TODO den Dateipfad über die Konsole einlesen

    // TODO Markdown-File vollständig in einen String einlesen

    // TODO Den String Stück für Stück durchgehen und in Datenstrukturen aus Rust einlesen

    // TODO Eine NEUE html-Datei öffnen

    // TODO Jede Datenstruktur aus dem Stream durchgehen und den zugehörigen HTML Code in die geöffnete Datei schreiben

    // TODO vor dem Schreiben einmal html_start auf dem file ausführen

    // TODO am Ende html_end auf dem File ausführen

    Ok(())
}

// TODO Datenstrukturen definieren

// Nicht verändern
fn html_start(f: &mut File) -> Result<(), Error> {
    write!(
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
fn html_end(f: &mut File) -> Result<(), Error> {
    write!(
        f,
        "</body>
  </html>"
    )?;

    Ok(())
}
