use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::paths;

/// Copied from the real repository. Must follow `lint.sh` dependencies.
const BASE_FILES: &[&str] = &["scripts/lint.sh", ".editorconfig"];
const BASE_WORKFLOW: &str = include_str!("base-workflow.yml");
const BASE_WORKFLOW_PATH: &str = ".github/workflows/base.yml";

/// Base commit on `origin/main`, HEAD commit adding the case input files.
pub struct TestRepo {
    pub path: PathBuf,
    pub base_sha: String,
}

impl TestRepo {
    pub fn create(tmp: &Path, path_var: &OsStr, case_folder: &Path) -> Result<Self, String> {
        let git = Git {
            tmp,
            path_var,
            repo: tmp.join("repo"),
        };
        fs::create_dir(&git.repo).map_err(paths::error(&git.repo))?;
        git.run(&["init", "--quiet", "--initial-branch=main"])?;
        for file in BASE_FILES {
            copy_file(Path::new(file), &git.repo.join(file))?;
        }
        write_file(&git.repo.join(BASE_WORKFLOW_PATH), BASE_WORKFLOW)?;
        git.commit("base")?;
        git.run(&["update-ref", "refs/remotes/origin/main", "HEAD"])?;
        let base_sha = git.run(&["rev-parse", "HEAD"])?.trim().to_string();
        copy_inputs(case_folder, &git.repo)?;
        git.commit("head")?;
        Ok(Self {
            path: git.repo,
            base_sha,
        })
    }
}

pub fn isolated_command(program: &str, path_var: &OsStr, home: &Path) -> Command {
    let mut command = Command::new(program);
    command
        .env_clear()
        .env("PATH", path_var)
        .env("HOME", home)
        .env("LC_ALL", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

struct Git<'a> {
    tmp: &'a Path,
    path_var: &'a OsStr,
    repo: PathBuf,
}

impl Git<'_> {
    fn commit(&self, message: &str) -> Result<(), String> {
        self.run(&["add", "--all"])?;
        self.run(&[
            "-c",
            "user.name=GPEx Test",
            "-c",
            "user.email=test@gpex.invalid",
            "commit",
            "--quiet",
            "--allow-empty",
            "--message",
            message,
        ])?;
        Ok(())
    }

    fn run(&self, args: &[&str]) -> Result<String, String> {
        let output = isolated_command("git", self.path_var, self.tmp)
            .args(args)
            .current_dir(&self.repo)
            .output()
            .map_err(|e| format!("git {}: {e}", args.join(" ")))?;
        if !output.status.success() {
            return Err(format!(
                "git {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

fn copy_inputs(case_folder: &Path, repo: &Path) -> Result<(), String> {
    for entry in fs::read_dir(case_folder).map_err(paths::error(case_folder))? {
        let name = entry.map_err(paths::error(case_folder))?.file_name();
        if !name.as_encoded_bytes().starts_with(b"expected") {
            copy_entry(&case_folder.join(&name), &repo.join(name))?;
        }
    }
    Ok(())
}

fn copy_entry(source: &Path, target: &Path) -> Result<(), String> {
    if source.file_name() == Some(OsStr::new(".DS_Store")) {
        return Ok(());
    }
    if !source.is_dir() {
        return copy_file(source, target);
    }
    fs::create_dir_all(target).map_err(paths::error(target))?;
    for entry in fs::read_dir(source).map_err(paths::error(source))? {
        let name = entry.map_err(paths::error(source))?.file_name();
        copy_entry(&source.join(&name), &target.join(name))?;
    }
    Ok(())
}

fn copy_file(source: &Path, target: &Path) -> Result<(), String> {
    create_parent(target)?;
    fs::copy(source, target).map_err(paths::error(source))?;
    Ok(())
}

fn write_file(target: &Path, content: &str) -> Result<(), String> {
    create_parent(target)?;
    fs::write(target, content).map_err(paths::error(target))
}

fn create_parent(target: &Path) -> Result<(), String> {
    let parent = target.parent().expect("target path has a parent");
    fs::create_dir_all(parent).map_err(paths::error(parent))
}
