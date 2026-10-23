//! Golden tests: compile every example and every case in `tests/golden/cases`,
//! compare the generated JavaScript with a checked-in snapshot and, when an
//! expected `.out` file exists, run the JavaScript with Node and compare stdout.
//!
//! Layout:
//! - `examples/<name>.fratm` → `tests/golden/examples/<name>.js` (+ optional `.out`)
//! - `tests/golden/cases/<name>.fratm` → `<name>.js` (+ optional `.out`) next to it
//!
//! Regenerate snapshots after an intended change with:
//! `FRATM_UPDATE_GOLDEN=1 cargo test -p fratm-core --test golden`
//! Only `.js` files are written; a `.out` file is created by hand when a case
//! should also be executed, and then kept up to date by the same command.

use fratm_core::{compile, CompileOptions};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Case {
    name: String,
    source: PathBuf,
    expected_js: PathBuf,
    expected_out: PathBuf,
}

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fratm_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "fratm"))
        .collect();
    files.sort();
    files
}

fn collect_cases() -> Vec<Case> {
    let root = crate_dir();
    let examples = root.join("../../examples");
    let snapshots = root.join("tests/golden/examples");
    let cases = root.join("tests/golden/cases");

    let mut all = Vec::new();
    for source in fratm_files(&examples) {
        let stem = source.file_stem().unwrap().to_string_lossy().into_owned();
        all.push(Case {
            name: format!("examples/{stem}"),
            expected_js: snapshots.join(format!("{stem}.js")),
            expected_out: snapshots.join(format!("{stem}.out")),
            source,
        });
    }
    for source in fratm_files(&cases) {
        let stem = source.file_stem().unwrap().to_string_lossy().into_owned();
        all.push(Case {
            name: format!("cases/{stem}"),
            expected_js: source.with_extension("js"),
            expected_out: source.with_extension("out"),
            source,
        });
    }
    all
}

fn run_node(code: &str, name: &str) -> Result<String, String> {
    let dir = std::env::temp_dir().join(format!("fratm-golden-{}", std::process::id()));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let file = dir.join(format!("{}.cjs", name.replace('/', "_")));
    fs::write(&file, code).map_err(|e| e.to_string())?;
    let output = Command::new("node").arg(&file).output();
    let _ = fs::remove_file(&file);
    let output = output.map_err(|e| format!("cannot start node (is Node.js installed?): {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "node exited with {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"))
}

fn mismatch(what: &str, case: &Case, expected: &str, actual: &str) -> String {
    let diff = pretty_assertions::StrComparison::new(expected, actual);
    format!("{}: {what} differs from the snapshot\n{diff}", case.name)
}

#[test]
fn golden() {
    let update = std::env::var_os("FRATM_UPDATE_GOLDEN").is_some();
    let cases = collect_cases();
    assert!(!cases.is_empty(), "no golden cases found");

    let mut failures = Vec::new();
    for case in &cases {
        let source = fs::read_to_string(&case.source).unwrap();
        let code = match compile(&source, CompileOptions::default()) {
            Ok(result) => result.code,
            Err(e) => {
                failures.push(format!("{}: does not compile: {e}", case.name));
                continue;
            }
        };

        if update {
            fs::create_dir_all(case.expected_js.parent().unwrap()).unwrap();
            fs::write(&case.expected_js, &code).unwrap();
        } else {
            match fs::read_to_string(&case.expected_js) {
                Ok(expected) if expected == code => {}
                Ok(expected) => failures.push(mismatch("generated JS", case, &expected, &code)),
                Err(_) => failures.push(format!(
                    "{}: missing snapshot {} (run with FRATM_UPDATE_GOLDEN=1)",
                    case.name,
                    case.expected_js.display()
                )),
            }
        }

        if !case.expected_out.exists() {
            continue;
        }
        let stdout = match run_node(&code, &case.name) {
            Ok(stdout) => stdout,
            Err(e) => {
                failures.push(format!("{}: {e}", case.name));
                continue;
            }
        };
        if update {
            fs::write(&case.expected_out, &stdout).unwrap();
        } else {
            let expected = fs::read_to_string(&case.expected_out)
                .unwrap()
                .replace("\r\n", "\n");
            if expected != stdout {
                failures.push(mismatch("output", case, &expected, &stdout));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} golden cases failed:\n\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n\n")
    );
}
