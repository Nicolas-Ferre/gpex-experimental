use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Real repository files of the base commit.
/// Must follow `lint.sh` dependencies.
const BASE_FILES: [&str; 2] = ["scripts/lint.sh", ".editorconfig"];
const BASE_WORKFLOW: &str = include_str!("base-workflow.yml");
const BASE_WORKFLOW_PATH: &str = ".github/workflows/base.yml";

/// Test repository whose base commit is `origin/main` and HEAD commit adds the case input files.
pub struct TestRepo {
    pub path: PathBuf,
    pub base_sha: String,
}

impl TestRepo {
    pub fn create(tmp: &Path, path_var: &OsStr, case_folder: &Path) -> Result<Self, String> {
        let git = Git { tmp, path_var, repo: tmp.join("repo") };
        fs::create_dir(&git.repo).map_err(|e| e.to_string())?;
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
        Ok(Self { path: git.repo, base_sha })
    }
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
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.repo)
            .env_clear()
            .env("PATH", self.path_var)
            .env("HOME", self.tmp)
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_NOSYSTEM", "1")
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
    for (name, source) in sorted_entries(case_folder)? {
        if !name.to_string_lossy().starts_with("expected") {
            copy_entry(&source, &repo.join(name))?;
        }
    }
    Ok(())
}

fn copy_entry(source: &Path, target: &Path) -> Result<(), String> {
    if !source.is_dir() {
        return copy_file(source, target);
    }
    fs::create_dir_all(target).map_err(|e| format!("{}: {e}", target.display()))?;
    for (name, child) in sorted_entries(source)? {
        copy_entry(&child, &target.join(name))?;
    }
    Ok(())
}

fn sorted_entries(folder: &Path) -> Result<Vec<(OsString, PathBuf)>, String> {
    let error = |e: std::io::Error| format!("{}: {e}", folder.display());
    let mut entries = Vec::new();
    for entry in fs::read_dir(folder).map_err(error)? {
        let entry = entry.map_err(error)?;
        entries.push((entry.file_name(), entry.path()));
    }
    entries.sort();
    Ok(entries)
}

fn copy_file(source: &Path, target: &Path) -> Result<(), String> {
    create_parent(target)?;
    fs::copy(source, target).map_err(|e| format!("{}: {e}", source.display()))?;
    Ok(())
}

fn write_file(target: &Path, content: &str) -> Result<(), String> {
    create_parent(target)?;
    fs::write(target, content).map_err(|e| format!("{}: {e}", target.display()))
}

fn create_parent(target: &Path) -> Result<(), String> {
    let parent = target.parent().expect("target path has a parent");
    fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))
}
