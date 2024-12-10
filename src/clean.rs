use {
    crate::{
        args::Args,
        cleanfile::{Cleanfile, Framework},
    },
    std::{
        path::{Path, PathBuf},
        process::{Command, Stdio},
    },
};

pub fn read_and_clean(path: &Path, args: Args) {
    let cleanfile = read(path);
    clean(cleanfile, path.to_path_buf(), args);
}

pub fn read(path: &Path) -> Cleanfile {
    serde_yaml::from_reader::<_, Cleanfile>(std::fs::File::open(path).unwrap()).unwrap()
    // TODO if parsing the version fails, then ignore it (but only in recursive mode)
}

pub fn clean(cleanfile: Cleanfile, cleanfile_path: PathBuf, args: Args) {
    if let Some(frameworks) = cleanfile.frameworks {
        for framework in frameworks {
            match framework {
                Framework::Cargo => {
                    let path = cleanfile_path.parent().unwrap();
                    if !path.join("Cargo.toml").exists() {
                        panic!(
                            "Requested to clean a cargo project, but no Cargo.toml found at {}",
                            path.display()
                        );
                    }
                    println!("Cleaning cargo project at {}", path.display());
                    if !args.dry_run {
                        assert!(
                            Command::new("cargo")
                                .arg("clean")
                                .current_dir(path)
                                .stdout(Stdio::inherit())
                                .stderr(Stdio::inherit())
                                .status()
                                .expect("Failed to clean cargo project")
                                .success(),
                            "Failed to clean cargo project"
                        );
                    }
                }
            }
        }
    }

    if cleanfile.docker_prune_all {
        println!("Pruning all Docker containers, images, and volumes");
        if !args.dry_run {
            assert!(
                Command::new("docker")
                    .arg("system")
                    .arg("prune")
                    .arg("--all")
                    .arg("--force")
                    .arg("--volumes")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .expect("Failed to prune docker containers and images")
                    .success(),
                "Failed to prune docker containers and images"
            );
        }
    }

    recurse(
        cleanfile.recurse_depth,
        cleanfile_path.parent().unwrap(),
        &args,
    );
}

fn recurse(depth: u8, path: &Path, args: &Args) {
    // println!("Recursing into {} (depth={depth})", path.display());
    if depth == 0 {
        return;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            // println!("Entry {} is a directory", entry.path().display());

            let cleanfile_path = entry.path().join("cleanfile");
            if cleanfile_path.exists() && cleanfile_path.is_file() {
                println!("{}", cleanfile_path.display());
                read_and_clean(&cleanfile_path, args.clone());
            }

            recurse(depth - 1, &entry.path(), args);
        }
    }
}
