use crate::repository::Repository;
use clap::ArgMatches;

pub fn exec(matches: Option<&ArgMatches>) -> Result<(), failure::Error> {
    let path = if let Some(matches) = matches {
        matches.value_of("path").unwrap_or(".")
    } else {
        "."
    };

    Repository::repository_create(path)?;

    Ok(())
}
