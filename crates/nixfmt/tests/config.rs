use std::num::NonZeroUsize;

use nixfmt::config::Config;
use pretty_assertions::assert_eq;

fn format(input: &str, config: Config) -> String {
    nixfmt::format::in_memory("test.nix".to_owned(), input.to_owned(), config).1
}

#[test]
fn compact_attribute_set_preserves_its_input_shape() {
    assert_eq!(
        format("{ foo = 1; bar = 2; }", Config::default()),
        "{ foo = 1; bar = 2; }\n"
    );
}

#[test]
fn compact_attribute_set_does_not_expand_for_nested_structure() {
    let once = format("{ a = if x then y else z; }", Config::default());

    assert_eq!(once, "{ a =\n  if x\n  then y\n  else z; }\n");
    assert_eq!(format(&once, Config::default()), once);
}

#[test]
fn newline_before_attribute_set_does_not_expand_its_contents() {
    assert_eq!(
        format("rec\n{ a = 1; }", Config::default()),
        "rec\n{ a = 1; }\n"
    );
}

#[test]
fn multiline_attribute_set_preserves_its_input_shape() {
    assert_eq!(
        format("{ foo = 1;\nbar = 2; }", Config::default()),
        "{\n  foo = 1;\n  bar = 2;\n}\n"
    );
}

#[test]
fn pattern_preserves_its_input_shape() {
    assert_eq!(
        format("{ foo, bar }: foo", Config::default()),
        "{ foo, bar }: foo\n"
    );
    assert_eq!(
        format("{ foo,\nbar }: foo", Config::default()),
        "{\n  foo,\n  bar,\n}:\nfoo\n"
    );
}

#[test]
fn space_around_brackets_keeps_single_empty_attribute_set_compact() {
    assert_eq!(format("[ { } ]", Config::default()), "[{}]\n");
}

#[test]
fn bracket_spacing_is_enabled_by_default_and_can_be_disabled() {
    let default = Config::default();
    let compact = Config { space_around_brackets: false, ..default };

    assert!(default.space_around_brackets);
    for (input, spaced, unspaced) in [
        ("[a b]", "[ a b ]\n", "[a b]\n"),
        ("{a = 1;}", "{ a = 1; }\n", "{a = 1;}\n"),
        ("{a}: a", "{ a }: a\n", "{a}: a\n"),
    ] {
        assert_eq!(format(input, default), spaced);
        assert_eq!(format(input, compact), unspaced);
    }
}

#[test]
fn omitted_maximum_width_keeps_baseline_layout() {
    assert_eq!(format("[ a b c ]", Config::default()), "[ a b c ]\n");
    assert_eq!(
        format("{ foo = [ a b ]; }", Config::default()),
        "{ foo = [ a b ]; }\n"
    );
}

#[test]
fn maximum_width_expands_breakable_syntax() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(5).unwrap()),
        ..Default::default()
    };

    assert_eq!(format("[ a b c ]", config), "[\n  a\n  b\n  c\n]\n");
}

#[test]
fn maximum_width_includes_indentation() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(12).unwrap()),
        ..Default::default()
    };

    assert_eq!(
        format("{ foo = [ a b ]; }", config),
        "{ foo = [\n  a\n  b\n]; }\n"
    );
}

#[test]
fn maximum_width_keeps_unbreakable_overflow() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(5).unwrap()),
        ..Default::default()
    };

    assert_eq!(format("\"abcdefghij\"", config), "\"abcdefghij\"\n");
}

#[test]
fn configured_formatting_is_idempotent() {
    let config = Config {
        max_width: Some(NonZeroUsize::new(12).unwrap()),
        ..Default::default()
    };
    let once = format("{ foo = [ a b ]; }", config);

    assert_eq!(format(&once, config), once);
}
