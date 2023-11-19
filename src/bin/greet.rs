use std::env;

fn main() {
    if let Some(name) = env::args().nth(1) {
        println!(
            "Hallo {}, ich hoffe du hast nach dem Vortrag Lust auf Rust bekommen.",
            name
        );
        println!("Bist du bereit selbst Hand anzulegen?");
    } else {
        println!("Gib bitte deinen Namen als Konsolen-Argument ein, damit es losgehen kann!");
    }
}
