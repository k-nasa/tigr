mod commands;
mod repository;

use commands::build_app;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), failure::Error> {
    let matches = build_app().get_matches();

    match matches.subcommand() {
        ("init", arg) => commands::init::exec(arg)?,
        _ => build_app().print_help()?,
    }

    Ok(())
}
