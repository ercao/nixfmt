use std::num::NonZeroUsize;
use std::ops::Range;

struct Candidate {
    whitespace: Range<usize>,
    code_columns: usize,
    line_columns: usize,
    comment_column: usize,
    indentation: String,
}

pub(crate) fn align(
    formatted: String,
    comment_offsets: &[usize],
    max_width: Option<NonZeroUsize>,
) -> String {
    // The renderer records comment tokens, excluding hashes inside strings and
    // block comments without reparsing the formatted text.
    if comment_offsets.is_empty() {
        return formatted;
    }

    let mut edits = Vec::new();
    let mut group = Vec::new();
    let mut group_indentation = None;
    let mut comment_index = 0;
    let mut offset = 0;

    for line in formatted.split_inclusive('\n') {
        let line_end = offset + line.len();
        let comment_offset = comment_offsets
            .get(comment_index)
            .copied()
            .filter(|comment_offset| *comment_offset < line_end);

        let candidate = comment_offset.and_then(|comment_offset| {
            comment_index += 1;
            let line = line.strip_suffix('\n').unwrap_or(line);
            let comment_column =
                formatted[offset..comment_offset].chars().count();
            let prefix = &formatted[offset..comment_offset];
            let code = prefix
                .trim_end_matches(|character| matches!(character, ' ' | '\t'));

            if code.trim().is_empty() {
                return None;
            }

            let code_without_indentation =
                code.trim_start_matches(|character| {
                    matches!(character, ' ' | '\t')
                });
            let indentation =
                code[..code.len() - code_without_indentation.len()].to_owned();

            Some(Candidate {
                whitespace: offset + code.len()..comment_offset,
                code_columns: code.chars().count(),
                line_columns: line.chars().count(),
                comment_column,
                indentation,
            })
        });

        match candidate {
            Some(candidate)
                if group_indentation.as_deref()
                    == Some(candidate.indentation.as_str()) =>
            {
                group.push(candidate);
            }
            Some(candidate) => {
                align_group(&group, max_width, &mut edits);
                group.clear();
                group_indentation = Some(candidate.indentation.clone());
                group.push(candidate);
            }
            None => {
                align_group(&group, max_width, &mut edits);
                group.clear();
                group_indentation = None;
            }
        }

        offset = line_end;
    }

    align_group(&group, max_width, &mut edits);

    if edits.is_empty() {
        return formatted;
    }

    let extra: usize = edits
        .iter()
        .map(|(whitespace, width)| width.saturating_sub(whitespace.len()))
        .sum();
    let mut aligned = String::with_capacity(formatted.len() + extra);
    let mut offset = 0;
    // Copy each unchanged span once; in-place insertion would repeatedly move
    // the rest of the file for large groups of trailing comments.
    for (whitespace, width) in edits {
        aligned.push_str(&formatted[offset..whitespace.start]);
        aligned.extend(std::iter::repeat_n(' ', width));
        offset = whitespace.end;
    }
    aligned.push_str(&formatted[offset..]);
    aligned
}

fn align_group(
    group: &[Candidate],
    max_width: Option<NonZeroUsize>,
    edits: &mut Vec<(Range<usize>, usize)>,
) {
    if group.len() < 2 {
        return;
    }

    let target_column =
        group.iter().map(|candidate| candidate.code_columns).max().unwrap() + 1;

    if max_width.is_some_and(|max_width| {
        group.iter().any(|candidate| {
            let aligned_width = candidate.line_columns
                - candidate.comment_column
                + target_column;
            candidate.line_columns <= max_width.get()
                && aligned_width > max_width.get()
        })
    }) {
        return;
    }

    edits.extend(group.iter().map(|candidate| {
        (candidate.whitespace.clone(), target_column - candidate.code_columns)
    }));
}
