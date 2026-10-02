use std::fmt::Write;
use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::Duration;
use std::time::Instant;

use nixfmt::config::Config;
use nixfmt::format::Status;

fn measure(name: &str, input: &str, config: Config) {
    let format = || {
        let (status, output) = nixfmt::format::in_memory(
            "benchmark.nix".to_owned(),
            black_box(input).to_owned(),
            config,
        );
        assert!(matches!(status, Status::Changed(_)));
        black_box(output);
    };

    format();
    let start = Instant::now();
    let mut iterations = 0;
    while start.elapsed() < Duration::from_millis(500) || iterations < 5 {
        format();
        iterations += 1;
    }
    let seconds = start.elapsed().as_secs_f64() / f64::from(iterations);
    println!(
        "{name:24} {:9.3} ms/iteration {:8.2} MiB/s ({iterations} iterations)",
        seconds * 1000.0,
        input.len() as f64 / seconds / (1024.0 * 1024.0),
    );
}

fn main() {
    let default = Config::default();
    let medium = include_str!("../tests/cases/default/idioms_nixos_1/in.nix");
    measure("real-world-12kb", medium, default);
    measure(
        "real-world-width100",
        medium,
        Config { max_width: NonZeroUsize::new(100), ..default },
    );
    measure(
        "parentheses",
        include_str!("../tests/cases/default/paren/in.nix"),
        default,
    );
    for depth in [16, 32, 64] {
        let input =
            format!("{}1{}\n", "{ a = ".repeat(depth), "; }".repeat(depth));
        measure(&format!("compact-depth-{depth}"), &input, default);
    }

    let mut plain = String::from("''\n");
    let mut interpolated = String::from("''\n");
    for i in 0..1000 {
        writeln!(plain, "  echo line_{i} with trailing spaces  ").unwrap();
        writeln!(interpolated, "  echo ${{{i}}} with trailing spaces  ")
            .unwrap();
    }
    plain.push_str("''\n");
    interpolated.push_str("''\n");
    measure("plain-string-1000", &plain, default);
    measure("interpolated-string-1000", &interpolated, default);

    let mut comments = String::from("{\n");
    for i in 0..50000 {
        let suffix = if i % 2 == 0 { "" } else { "_long" };
        writeln!(comments, "  key_{i}{suffix} = {i}; # comment {i}").unwrap();
    }
    comments.push_str("}\n");
    measure("comments-50000", &comments, default);
    measure(
        "comments-50000-aligned",
        &comments,
        Config { align_trailing_comments: true, ..default },
    );
}
