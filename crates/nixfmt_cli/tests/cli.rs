use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use pretty_assertions::assert_eq;

static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct TestCase {
    args: &'static [&'static str],
    stdin: Option<&'static str>,
}

const CASES: &[TestCase] = &[
    TestCase { args: &["--help"], stdin: None },
    TestCase { args: &["--version"], stdin: None },
    //
    TestCase { args: &[], stdin: None },
    // changed
    TestCase { args: &[], stdin: Some("[]") },
    TestCase { args: &["--quiet"], stdin: Some("[]") },
    TestCase { args: &["--quiet", "--quiet"], stdin: Some("[]") },
    // check unchanged
    TestCase { args: &["--check"], stdin: Some("[]\n") },
    TestCase { args: &["--check", "--quiet"], stdin: Some("[]\n") },
    TestCase { args: &["--check", "--quiet", "--quiet"], stdin: Some("[]\n") },
    // check changed
    TestCase { args: &["--check"], stdin: Some("[\t]") },
    TestCase { args: &["--check", "--quiet"], stdin: Some("[\t]") },
    TestCase { args: &["--check", "--quiet", "--quiet"], stdin: Some("[\t]") },
    // check error
    TestCase { args: &[], stdin: Some("[") },
    TestCase { args: &["--quiet"], stdin: Some("[") },
    TestCase { args: &["--quiet", "--quiet"], stdin: Some("[") },
    // nothing to format
    TestCase { args: &[".", "--exclude", "."], stdin: None },
    TestCase { args: &[".", "--exclude", ".", "--quiet"], stdin: None },
    TestCase {
        args: &["--exclude", ".", "--quiet", "--quiet", "--", "."],
        stdin: None,
    },
    //
    TestCase { args: &["--check", "tests/inputs/changed.nix"], stdin: None },
    TestCase {
        args: &["--check", "tests/inputs/changed.nix", "--quiet"],
        stdin: None,
    },
    TestCase {
        args: &["-c", "tests/inputs/changed.nix", "-e", "tests/changed.nix"],
        stdin: None,
    },
    TestCase {
        args: &[
            "-c",
            "tests/inputs/changed.nix",
            "-q",
            "-e",
            "tests/changed.nix",
        ],
        stdin: None,
    },
    TestCase {
        args: &["--check", "tests/inputs/changed.nix", "-qq"],
        stdin: None,
    },
    TestCase { args: &["-c", "tests/inputs/unchanged.nix"], stdin: None },
    TestCase {
        args: &["--check", "tests/inputs/unchanged.nix", "-q"],
        stdin: None,
    },
    TestCase {
        args: &["--check", "tests/inputs/unchanged.nix", "-qq"],
        stdin: None,
    },
    TestCase { args: &["--check", "tests/inputs/error.nix"], stdin: None },
    TestCase {
        args: &["--check", "tests/inputs/error.nix", "-q"],
        stdin: None,
    },
    TestCase {
        args: &["--check", "tests/inputs/error.nix", "-qq"],
        stdin: None,
    },
    TestCase {
        args: &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--config",
            "../../.nixfmt.toml",
            "--threads",
            "1",
        ],
        stdin: None,
    },
    TestCase {
        args: &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--config",
            "tests/configs/empty_config.toml",
            "-t",
            "1",
        ],
        stdin: None,
    },
    TestCase {
        args: &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--config",
            "tests/configs/wrong_key.toml",
        ],
        stdin: None,
    },
    TestCase {
        args: &[
            "--check",
            "tests/inputs/unchanged.nix",
            "--config",
            "tests/configs/zero_width.toml",
        ],
        stdin: None,
    },
];

#[test]
fn cases() {
    let should_update = std::env::var("UPDATE").is_ok();

    let output_path = PathBuf::new().join("tests").join("output.txt");

    let mut output_got = String::new();

    for case in CASES {
        output_got.push_str("===\n");
        output_got.push_str(&format!("args: {:?}\n", case.args));

        let mut child = Command::new("cargo")
            .env("NIXFMT_THREADS", "1")
            .args(["run", "--quiet", "--"])
            .args(case.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("command failed");

        if let Some(stdin) = case.stdin {
            output_got.push_str(&format!("stdin: {:?}\n", stdin));

            child
                .stdin
                .take()
                .unwrap()
                .write_all(stdin.as_bytes())
                .expect("unable to write to child stdin");
        }

        let output = child.wait_with_output().expect("child command failed");

        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");
        if !stdout.is_empty() {
            output_got
                .push_str(&format!("stdout:\n{}\n", indent_and_clean(&stdout)));
        }

        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8");
        if !stderr.is_empty() {
            output_got
                .push_str(&format!("stderr:\n{}\n", indent_and_clean(&stderr)));
        }

        output_got
            .push_str(&format!("exit code: {:?}\n", output.status.code()));
    }

    if should_update {
        std::fs::File::create(&output_path)
            .unwrap()
            .write_all(output_got.as_bytes())
            .unwrap();
    }

    let output_expected = std::fs::read_to_string(&output_path).unwrap();

    assert_eq!(output_expected, output_got);
}

#[test]
fn discovers_dot_nixfmt_toml_in_current_directory() {
    let temp_dir = temp_dir();
    std::fs::write(temp_dir.join(".nixfmt.toml"), "max_width = 5\n").unwrap();

    let output = run_in(&temp_dir, "[ a b c ]");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "[\n  a\n  b\n  c\n]\n"
    );
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn does_not_discover_legacy_nixfmt_toml() {
    let temp_dir = temp_dir();
    std::fs::write(temp_dir.join("nixfmt.toml"), "max_width = 5\n").unwrap();

    let output = run_in(&temp_dir, "[ a b c ]");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[ a b c ]\n");
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn bracket_spacing_defaults_in_config_and_can_be_disabled() {
    for (config, expected) in [
        ("", "[ a b ]\n"),
        ("indentation = \"TwoSpaces\"\n", "[ a b ]\n"),
        ("space_around_brackets = false\n", "[a b]\n"),
    ] {
        let temp_dir = temp_dir();
        std::fs::write(temp_dir.join(".nixfmt.toml"), config).unwrap();

        let output = run_in(&temp_dir, "[a b]");

        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
        std::fs::remove_dir_all(temp_dir).unwrap();
    }
}

#[test]
fn rejects_zero_maximum_width() {
    let temp_dir = temp_dir();
    std::fs::write(temp_dir.join(".nixfmt.toml"), "max_width = 0\n").unwrap();

    let output = run_in(&temp_dir, "[]");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Errors found in config"));
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn rejects_experimental_config_option() {
    let output = Command::new(env!("CARGO_BIN_EXE_nixfmt"))
        .args(["--experimental-config", "config.toml"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Found argument '--experimental-config'"));
}

#[test]
fn discovers_trailing_comment_alignment_configuration() {
    let temp_dir = temp_dir();
    std::fs::write(
        temp_dir.join(".nixfmt.toml"),
        "align_trailing_comments = true\n",
    )
    .unwrap();

    let output = run_in(&temp_dir, "{\n  a = 1; # one\n  longer = 2; # two\n}");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "{\n  a = 1;      # one\n  longer = 2; # two\n}\n",
    );
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn loads_trailing_comment_alignment_from_explicit_config() {
    let temp_dir = temp_dir();
    std::fs::write(
        temp_dir.join("alignment.toml"),
        "align_trailing_comments = true\n",
    )
    .unwrap();

    let output = run_with_args_in(
        &temp_dir,
        "{\n  a = 1; # one\n  longer = 2; # two\n}",
        &["--config", "alignment.toml"],
    );

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "{\n  a = 1;      # one\n  longer = 2; # two\n}\n",
    );
    std::fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn single_file_uses_one_worker_even_when_more_are_requested() {
    let dir = temp_dir();
    let path = dir.join("one.nix");
    std::fs::write(&path, "[]").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nixfmt"))
        .current_dir(&dir)
        .args(["--threads", "8"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("using 1 thread."));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "[]\n");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn sequential_and_parallel_batches_preserve_check_and_write_behavior() {
    for threads in ["1", "2"] {
        let dir = temp_dir();
        let path = dir.join("one.nix");
        let other = dir.join("two.nix");
        std::fs::write(&path, "[a b]").unwrap();
        std::fs::write(&other, "{a=1;}").unwrap();
        let run = |check| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_nixfmt"));
            command
                .current_dir(&dir)
                .args(["--quiet", "--threads", threads])
                .arg(&dir);
            if check {
                command.arg("--check");
            }
            command.output().unwrap()
        };
        assert_eq!(run(true).status.code(), Some(2));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "[a b]");
        assert!(run(false).status.success());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "[ a b ]\n");
        assert_eq!(std::fs::read_to_string(&other).unwrap(), "{ a = 1; }\n");
        assert!(run(true).status.success());
        std::fs::remove_dir_all(dir).unwrap();
    }
}

fn temp_dir() -> PathBuf {
    let unique = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir()
        .join(format!("nixfmt-cli-{}-{unique}", std::process::id()));
    std::fs::create_dir(&path).unwrap();
    path
}

fn run_in(current_dir: &std::path::Path, stdin: &str) -> std::process::Output {
    run_with_args_in(current_dir, stdin, &[])
}

fn run_with_args_in(
    current_dir: &std::path::Path,
    stdin: &str,
    args: &[&str],
) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_nixfmt"))
        .current_dir(current_dir)
        .args(args)
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.take().unwrap().write_all(stdin.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

fn indent_and_clean(data: &str) -> String {
    data.lines().fold(String::new(), |mut output, line| {
        if line.is_empty() {
            let _ = writeln!(output);
        } else {
            let _ = writeln!(output, "  {}", line);
        }
        output
    })
}
