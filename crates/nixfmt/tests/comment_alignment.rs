use std::num::NonZeroUsize;

use nixfmt::config::Config;
use nixfmt::config::Indentation;
use pretty_assertions::assert_eq;

fn format(input: &str, config: Config) -> String {
    nixfmt::format::in_memory("test.nix".to_owned(), input.to_owned(), config).1
}

fn aligned_config() -> Config {
    Config { align_trailing_comments: true, ..Default::default() }
}

#[test]
fn omitted_trailing_comment_alignment_keeps_baseline_spacing() {
    assert_eq!(
        format("{\n  a = 1; # one\n  longer = 2; # two\n}", Config::default(),),
        "{\n  a = 1; # one\n  longer = 2; # two\n}\n",
    );
}

#[test]
fn disabled_trailing_comment_alignment_keeps_baseline_spacing() {
    let config =
        Config { align_trailing_comments: false, ..Default::default() };

    assert_eq!(
        format("{\n  a = 1; # one\n  longer = 2; # two\n}", config),
        "{\n  a = 1; # one\n  longer = 2; # two\n}\n",
    );
}

#[test]
fn aligns_a_trailing_comment_group() {
    assert_eq!(
        format("{\n  a = 1; # one\n  longer = 2; # two\n}", aligned_config(),),
        "{\n  a = 1;      # one\n  longer = 2; # two\n}\n",
    );
}

#[test]
fn alignment_groups_follow_formatted_line_boundaries() {
    let input = "{\n  a = 1; # one\n  longer = 2; # two\n\n  single = 1; # single\n  # standalone\n  after = 2; # single\n  nested = {\n    a = 1; # one\n    longer = 2; # two\n  };\n}";

    assert_eq!(
        format(input, aligned_config()),
        "{\n  a = 1;      # one\n  longer = 2; # two\n\n  single = 1; # single\n  # standalone\n  after = 2; # single\n  nested = {\n    a = 1;      # one\n    longer = 2; # two\n  };\n}\n",
    );
}

#[test]
fn blank_lines_end_alignment_groups() {
    let input = "{\n  a = 1; # a\n  bb = 2; # b\n\n  extremely_long = 3; # c\n  x = 4; # d\n}";

    assert_eq!(
        format(input, aligned_config()),
        "{\n  a = 1;  # a\n  bb = 2; # b\n\n  extremely_long = 3; # c\n  x = 4;              # d\n}\n",
    );
}

#[test]
fn lines_without_trailing_comments_end_alignment_groups() {
    let input = "{\n  a = 1; # a\n  bb = 2; # b\n  separator = 0;\n  extremely_long = 3; # c\n  x = 4; # d\n}";

    assert_eq!(
        format(input, aligned_config()),
        "{\n  a = 1;  # a\n  bb = 2; # b\n  separator = 0;\n  extremely_long = 3; # c\n  x = 4;              # d\n}\n",
    );
}

#[test]
fn alignment_applies_outside_attribute_sets() {
    assert_eq!(
        format("[\n  a # one\n  longer # two\n]", aligned_config()),
        "[\n  a      # one\n  longer # two\n]\n",
    );
}

#[test]
fn alignment_applies_to_let_bindings() {
    let input = "let\n  a = 1; # one\n  longer = 2; # two\nin\n  a";
    let baseline = format(input, Config::default());

    assert_eq!(
        format(input, aligned_config()),
        baseline.replacen("  a = 1; # one", "  a = 1;      # one", 1),
    );
}

#[test]
fn strings_and_block_comments_do_not_participate() {
    let input = "{\n  a = \"#\"; # one\n  longer = 2; # two\n  block = 1; /* block */\n  after = 2; # single\n}";
    let baseline = format(input, Config::default());

    assert_eq!(
        format(input, aligned_config()),
        baseline.replacen("  a = \"#\"; # one", "  a = \"#\";    # one", 1,),
    );
}

#[test]
fn alignment_uses_spaces_and_unicode_scalar_columns() {
    let config = Config { indentation: Indentation::Tabs, ..aligned_config() };

    assert_eq!(
        format("{\n  a = \"界\"; # one\n  longer = 2; # two\n}", config),
        "{\n\ta = \"界\";    # one\n\tlonger = 2; # two\n}\n",
    );
}

#[test]
fn maximum_width_rejects_new_alignment_overflow() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(22).unwrap()),
        ..aligned_config()
    };

    assert_eq!(
        format("{\n  a = 1; # longcomment\n  longer = 2; # x\n}", config,),
        "{\n  a = 1; # longcomment\n  longer = 2; # x\n}\n",
    );
}

#[test]
fn preexisting_comment_overflow_does_not_disable_alignment() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(19).unwrap()),
        ..aligned_config()
    };

    assert_eq!(
        format("{\n  a = 1; # longcomment\n  longer = 2; # x\n}", config,),
        "{\n  a = 1;      # longcomment\n  longer = 2; # x\n}\n",
    );
}

#[test]
fn omitted_maximum_width_does_not_limit_alignment_padding() {
    let output = format(
        "{\n  a = 1; # short\n  \"123456789012345678901234567890\" = 2; # long\n}",
        aligned_config(),
    );
    let comment_columns: Vec<usize> =
        output.lines().filter_map(|line| line.find('#')).collect();

    assert_eq!(comment_columns, vec![40, 40]);
}

#[test]
fn trailing_comment_alignment_is_idempotent() {
    let once =
        format("{\n  a = 1; # one\n  longer = 2; # two\n}", aligned_config());

    assert_eq!(format(&once, aligned_config()), once);
}

#[test]
fn discarded_layout_trials_do_not_leave_comment_offsets() {
    let input = "if # condition\n true then {\n a = \"界\"; # one\n longer = 2; # two\n } else null";
    let output = format(input, aligned_config());

    assert_eq!(
        output,
        "if # condition\n  true\nthen {\n  a = \"界\";    # one\n  longer = 2; # two\n}\nelse null\n",
    );
    assert_eq!(format(&output, aligned_config()), output);
}

#[test]
fn large_comment_groups_preserve_every_line() {
    let mut input = String::from("{\n");
    let mut expected = String::from("{\n");
    for i in 0..1000 {
        let key = format!("key_{i}");
        input.push_str(&format!("  {key} = 1; # {i}\n"));
        expected.push_str(&format!(
            "  {key} = 1;{}# {i}\n",
            " ".repeat(8 - key.len())
        ));
    }
    input.push_str("}\n");
    expected.push_str("}\n");

    assert_eq!(format(&input, aligned_config()), expected);
}
