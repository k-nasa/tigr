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
        } else if !force {
            bail!("Configuration file missing")
        } else {
            Ini::new()
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

    /// Create a new repository at path
    pub fn repository_create(path: &str) -> Result<(), Error> {
        let repository = Self::new(Path::new(path), true)?;

        if repository.worktree.exists() {
            if !repository.worktree.is_dir() {
                bail!("{} is not a directory!", repository.worktree.display())
            }
        } else {
            fs::create_dir_all(repository.worktree.clone())?
        }

        repository.create_dir("branches")?;
        repository.create_dir("objects")?;
        repository.create_dir("refs/tags")?;
        repository.create_dir("refs/heads")?;

        fs::write(
            repository.gitdir.join("description"),
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )?;

        fs::write(repository.gitdir.join("HEAD"), "ref: refs/heads/master\n")?;

        let conf = Self::default_config();
        conf.write_to_file(repository.gitdir.join("config"))?;

        Ok(())
    }

    fn create_dir(&self, path: &str) -> Result<(), Error> {
        let path = self.gitdir.join(path);

        if path.exists() {
            if path.is_dir() {
                return Ok(());
            }
            bail!("Not a directory {}", path.display())
        }

        fs::create_dir_all(path)?;

        Ok(())
    }

    fn default_config() -> Ini {
        let mut conf = Ini::new();
        conf.with_section(Some("core".to_owned()))
            .set("repositoryformatversion", "0")
            .set("filemode", "false")
            .set("bare", "false");

        conf
    }
}
