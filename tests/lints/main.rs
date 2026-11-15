//! Snapshot tests of `scripts/lint.sh`, one per folder of `test_cases/lints/`.

mod repo;

use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use libtest_mimic::{Arguments, Failed, Trial};
use repo::TestRepo;

const CASES_FOLDER: &str = "test_cases/lints";
const EXPECTED_FILE: &str = "expected.log";

fn main() -> ExitCode {
    let args = Arguments::from_args();
    let folders = match case_folders() {
        Ok(folders) => folders,
        Err(error) => {
            eprintln!("{CASES_FOLDER}: {error}");
            return ExitCode::FAILURE;
        }
    };
    let trials = folders
        .into_iter()
        .map(|folder| {
            let name = format!("lints/{}", folder.file_name().unwrap_or_default().display());
            Trial::test(name, move || check_case(&folder))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit_code()
}

fn case_folders() -> std::io::Result<Vec<PathBuf>> {
    let mut folders = Vec::new();
    for entry in fs::read_dir(CASES_FOLDER)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && !entry.file_name().as_encoded_bytes().starts_with(b".") {
            folders.push(entry.path());
        }
    }
    folders.sort();
    Ok(folders)
}

fn check_case(case_folder: &Path) -> Result<(), Failed> {
    let actual = run_case(case_folder)?;
    let path = case_folder.join(EXPECTED_FILE);
    let expected = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    if actual != expected {
        return Err(format!("{}: unexpected output:\n{actual}", path.display()).into());
    }
    Ok(())
}

fn run_case(case_folder: &Path) -> Result<String, String> {
    let path_var = env::var_os("PATH").unwrap_or_default();
    let tmp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let repo = TestRepo::create(tmp.path(), &path_var, case_folder)?;
    let mut command = Command::new("bash");
    command
        .arg("scripts/lint.sh")
        .current_dir(&repo.path)
        .env_clear()
        .env("PATH", &path_var)
        .env("HOME", tmp.path())
        .env("LC_ALL", "C")
        .env("NO_COLOR", "1")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("ZIZMOR_OFFLINE", "true");
    let (output, exit_code) = run_with_shared_output(command)?;
    let output = String::from_utf8_lossy(&output).replace(&repo.base_sha, "<BASE_SHA>");
    Ok(format!("{output}\nexit code: {exit_code}\n"))
}

/// Runs the command with stdout and stderr written to the same pipe, to keep their write order.
fn run_with_shared_output(mut command: Command) -> Result<(Vec<u8>, i32), String> {
    let error = |e: std::io::Error| e.to_string();
    let (mut reader, writer) = std::io::pipe().map_err(error)?;
    command.stdout(writer.try_clone().map_err(error)?).stderr(writer);
    let mut child = command.spawn().map_err(error)?;
    // Drops the command's pipe writers, so that reading ends when the child exits.
    drop(command);
    let mut output = Vec::new();
    reader.read_to_end(&mut output).map_err(error)?;
    let status = child.wait().map_err(error)?;
    let exit_code = status.code().ok_or_else(|| format!("lint.sh killed: {status}"))?;
    Ok((output, exit_code))
}
