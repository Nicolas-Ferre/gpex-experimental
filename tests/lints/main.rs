mod paths;
mod repo;

use std::env;
use std::fs;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use libtest_mimic::{Arguments, Failed, Trial};
use repo::TestRepo;

const CASES_PARENT_FOLDER: &str = "test_cases";
const CASES_FOLDER: &str = "lints";
const EXPECTED_FILE: &str = "expected.log";

fn main() -> ExitCode {
    let args = Arguments::from_args();
    let cases_folder = Path::new(CASES_PARENT_FOLDER).join(CASES_FOLDER);
    let folders = match case_folders(&cases_folder) {
        Ok(folders) => folders,
        Err(error) => {
            eprintln!("{}: {error}", cases_folder.display());
            return ExitCode::FAILURE;
        }
    };
    let trials = folders
        .into_iter()
        .map(|folder| {
            let name = format!(
                "{CASES_FOLDER}/{}",
                folder.file_name().unwrap_or_default().display()
            );
            Trial::test(name, move || check_case(&folder))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit_code()
}

/// Returns paths relative to [`paths::ROOT`], so that messages show short paths.
fn case_folders(cases_folder: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut folders = Vec::new();
    for entry in fs::read_dir(paths::absolute(cases_folder))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && !entry.file_name().as_encoded_bytes().starts_with(b".") {
            folders.push(cases_folder.join(entry.file_name()));
        }
    }
    folders.sort();
    if folders.is_empty() {
        return Err(Error::other("no test case found"));
    }
    Ok(folders)
}

fn check_case(case_folder: &Path) -> Result<(), Failed> {
    let actual = run_case(case_folder).map_err(paths::error(case_folder))?;
    let path = case_folder.join(EXPECTED_FILE);
    let expected = fs::read_to_string(paths::absolute(&path)).map_err(paths::error(&path))?;
    if actual != expected {
        return Err(format!("{}: unexpected output:\n{actual}", path.display()).into());
    }
    Ok(())
}

fn run_case(case_folder: &Path) -> Result<String, String> {
    let path_var = env::var_os("PATH").unwrap_or_default();
    let tmp = tempfile::tempdir().map_err(|e| format!("temp directory: {e}"))?;
    let repo = TestRepo::create(tmp.path(), &path_var, case_folder)?;
    let mut command = repo::isolated_command("bash", &path_var, tmp.path());
    command
        .arg("scripts/lint.sh")
        .current_dir(&repo.path)
        .env("NO_COLOR", "1")
        .env("ZIZMOR_OFFLINE", "true");
    let (output, exit_code) = run_with_shared_output(command)?;
    let output = String::from_utf8_lossy(&output).replace(&repo.base_sha, "<BASE_SHA>");
    Ok(format!("{output}\nexit code: {exit_code}\n"))
}

/// Keeps the write order of stdout and stderr.
fn run_with_shared_output(mut command: Command) -> Result<(Vec<u8>, i32), String> {
    let error = |e: Error| format!("bash scripts/lint.sh: {e}");
    let (mut reader, writer) = io::pipe().map_err(error)?;
    command
        .stdout(writer.try_clone().map_err(error)?)
        .stderr(writer);
    let mut child = command.spawn().map_err(error)?;
    // `command` holds pipe writers, which would keep reading from ending after the child exits.
    drop(command);
    let mut output = Vec::new();
    let read = reader.read_to_end(&mut output);
    // Waits even if reading failed, so that lint.sh does not outlive its temp directory.
    let status = child.wait().map_err(error)?;
    read.map_err(error)?;
    let exit_code = status
        .code()
        .ok_or_else(|| format!("lint.sh killed: {status}"))?;
    Ok((output, exit_code))
}
