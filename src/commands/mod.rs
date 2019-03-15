use clap::{crate_description, crate_name, crate_version, Arg, SubCommand};

pub mod init;

pub type App = clap::App<'static, 'static>;

pub fn build_app() -> App {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize a new, empty repository."))
        .arg(Arg::with_name("path").help("Where to create the repository."))
}
