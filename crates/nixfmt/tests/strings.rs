use std::num::NonZeroUsize;

use nixfmt::config::{Config, Indentation};
use nixfmt::format::Status;

#[test]
fn indented_strings_preserve_whitespace_escapes_and_interpolations() {
    let cases = [
        (
            "''\n  first  \n  second\t \n''",
            Indentation::TwoSpaces,
            None,
            "''\n  first  \n  second\t \n''\n",
        ),
        (
            "''\n\tvalue  \n  界\t \n''",
            Indentation::Tabs,
            Some(20),
            "''\n  \tvalue  \n    界\t \n''\n",
        ),
        (
            "''\n  ''${literal}\n  quote ''' end\n''",
            Indentation::TwoSpaces,
            None,
            "''\n  ''${literal}\n  quote ''' end\n''\n",
        ),
        (
            "''\n \t \n   \n''",
            Indentation::TwoSpaces,
            None,
            "''\n \t \n   \n''\n",
        ),
        (
            "''\n  a\n  ${\n    if true then 1 else 2\n  }\n''",
            Indentation::Tabs,
            Some(20),
            "''\n  a\n  ${\n\t\tif true\n\t\tthen 1\n\t\telse 2\n\t}\n''\n",
        ),
        (
            "''\n  a\n  ${\n    if true then 1 else 2\n  }\n''",
            Indentation::TwoSpaces,
            Some(12),
            "''\n  a\n  ${\n    if true\n    then 1\n    else 2\n  }\n''\n",
        ),
    ];
    for (input, indentation, width, expected) in cases {
        let config = Config {
            indentation,
            max_width: width.and_then(NonZeroUsize::new),
            ..Default::default()
        };
        let (status, output) = nixfmt::format::in_memory(
            "string.nix".to_owned(),
            input.to_owned(),
            config,
        );
        assert!(matches!(status, Status::Changed(_)));
        assert_eq!(output, expected);
    }
}

#[test]
fn literal_text_is_never_an_interpolation_marker() {
    let input = concat!(
        "''4d13159079d76c1398db5f3ab0c62325",
        "f884b545e63226f7ec8aad96c52e13e8",
        "6b219abc9462c41b87e47344752e9940",
        "abf9353565f69a5db5c672b89372b84c''\n",
    );
    let (status, output) = nixfmt::format::in_memory(
        "literal.nix".to_owned(),
        input.to_owned(),
        Config::default(),
    );
    assert!(matches!(status, Status::Changed(false)));
    assert_eq!(output, input);
}
