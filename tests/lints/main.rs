mod paths;
mod repo;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Error, Read};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

use libtest_mimic::{Arguments, Failed, Trial};
use repo::TestRepo;

const CASES_PARENT_FOLDER: &str = "test_cases";
const CASES_FOLDER: &str = "lints";
const EXPECTED_FILE: &str = "expected.log";

fn main() -> ExitCode {
    let args = Arguments::from_args();
    let root = env!("CARGO_MANIFEST_DIR");
    if let Err(error) = env::set_current_dir(root) {
        eprintln!("{root}: {error}");
        return ExitCode::FAILURE;
    }
    let cases_folder = Path::new(CASES_PARENT_FOLDER).join(CASES_FOLDER);
    let names = match case_folder_names(&cases_folder) {
        Ok(names) => names,
        Err(error) => {
            eprintln!("{}: {error}", cases_folder.display());
            return ExitCode::FAILURE;
        }
    };
    let trials = names
        .into_iter()
        .map(|name| {
            let folder = cases_folder.join(&name);
            let name = format!("{CASES_FOLDER}/{}", name.display());
            Trial::test(name, move || check_case(&folder))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit_code()
}

fn case_folder_names(cases_folder: &Path) -> Result<Vec<OsString>, Error> {
    let mut names = Vec::new();
    for entry in fs::read_dir(cases_folder)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && !entry.file_name().as_encoded_bytes().starts_with(b".") {
            names.push(entry.file_name());
        }
    }
    names.sort();
    if names.is_empty() {
        return Err(Error::other("no test case found"));
    }
    Ok(names)
}

fn check_case(case_folder: &Path) -> Result<(), Failed> {
    let actual = run_case(case_folder).map_err(paths::error(case_folder))?;
    let path = case_folder.join(EXPECTED_FILE);
    let expected = fs::read_to_string(&path).map_err(paths::error(&path))?;
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
        .stdin(Stdio::null())
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
