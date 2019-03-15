#[cfg(test)]
mod test {
    use ruspec::ruspec;
    const COMMAND: &str = "target/debug/tigr";
    const SANDBOX_PATH: &str = "sandbox";

    ruspec! {
        describe "init command" {
            before {
                use std::fs;
                use std::process::Command;
                use std::path::Path;
                use super::*;

                Command::new(COMMAND)
                    .arg("init")
                    .arg(SANDBOX_PATH)
                    .output()
                    .unwrap();
            }

            after { fs::remove_dir_all(Path::new(SANDBOX_PATH)).unwrap(); }

            it "should create .git dir" {
                let path = Path::new(SANDBOX_PATH);
                let gitdir = path.join(".git");
                assert!(path.exists());
                assert!(gitdir.exists());
            }

            it "should create .git container files" {
                let path = Path::new(SANDBOX_PATH);
                let gitdir = path.join(".git");
                assert!(gitdir.exists());
                assert!(gitdir.join("branches").exists());
                assert!(gitdir.join("objects").exists());
                assert!(gitdir.join("refs/tags").exists());
                assert!(gitdir.join("refs/heads").exists());
            }

            // 順番が安定しないのでコメントアウト
            // it "should valid file content" {
            //     let path = Path::new(SANDBOX_PATH);
            //     let config_file = path.join(".git/config");
            //
            //     let config = fs::read_to_string(config_file).unwrap();
            //     assert_eq!(config, include_str!("./support/example.config"));
            // }
        }
    }
}
