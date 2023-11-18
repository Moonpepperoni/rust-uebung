use std::env;

fn main() {
    if let Some(name) = env::args().nth(1) {
        println!("Hallo {}! Ich hoffe du hast jetzt richtig Lust bekommen Rust einmal selbst auszuprobieren. Bist du bereit die Aufgaben zu starten?", name);
    } else {
        println!("Gib bitte einen Namen an, sodass ich dich persönlich ansprechen kann.")
    }
}
