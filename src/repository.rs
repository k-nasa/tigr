use failure::{bail, Error};
use ini::Ini;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: Ini,
}

impl Repository {
    pub fn new(path: &Path, force: bool) -> Result<Self, Error> {
        let worktree = path.to_path_buf();
        let gitdir = path.join(".git");

        if !(force || gitdir.is_dir()) {
            bail!("Not a Git repository {}", worktree.display())
        }

        let config_file = gitdir.join("config");

        let conf = if config_file.exists() {
            Ini::load_from_file(config_file).unwrap()
        } else {
            bail!("Configuration file missing")
        };

        if !force {
            let core_section = conf.section(Some("core".to_owned())).unwrap();
            let version = core_section.get("repositoryformatversion");

            if version != Some(&String::from("0")) {
                bail!("Unsupported repositoryformatversion {:?}", version)
            }
        }

        Ok(Repository {
            worktree,
            gitdir,
            conf: conf,
        })
    }
}
