use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use predicates::prelude::predicate;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("eocarcinus")?;

    cmd.arg("--path").arg("path/to/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn missing_path_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("eocarcinus")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));

    Ok(())
}
