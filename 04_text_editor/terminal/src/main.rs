use std::env;
use std::process;
use texteditor::config::Config;

pub mod text_editor;

fn main() -> Result<(), std::io::Error> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = texteditor::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
