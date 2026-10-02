use crate::config::Indentation;

/// Returns the indentation unit to use inside `''..''` string content.
///
/// Nix only strips leading **spaces** when evaluating indented strings.
/// We must never use tabs in string content, even when the surrounding code
/// uses tab indentation. This ensures string values are preserved correctly.
fn string_content_indent_unit(indentation: Indentation) -> &'static str {
    match indentation {
        Indentation::FourSpaces => "    ",
        Indentation::Tabs | Indentation::TwoSpaces => "  ",
    }
}

pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = build_ctx.take_steps();

    let mut children = crate::children::Children::new(build_ctx, node);

    let child = children.get_next().unwrap();
    let child_token = child.clone().into_token().unwrap();
    let text = child_token.text();
    steps.push(crate::builder::Step::Format(child));

    if text == "\"" {
        while let Some(child) = children.get_next() {
            if build_ctx.vertical {
                steps.push(crate::builder::Step::FormatWider(child));
            } else {
                steps.push(crate::builder::Step::Format(child));
            }
        }
    } else {
        let elements = children.get_remaining();
        let (closing, elements) = elements.split_last().unwrap();
        let mut content =
            String::with_capacity(usize::from(node.text_range().len()));
        let mut interpolations = Vec::new();
        for element in elements {
            if element.kind() == rnix::SyntaxKind::TOKEN_STRING_CONTENT {
                content.push_str(element.as_token().unwrap().text());
            } else {
                // A non-whitespace marker participates in indentation detection.
                // Its recorded offset, not its text, identifies the interpolation.
                interpolations.push((content.len(), element));
                content.push('x');
            }
        }

        let mut lines: Vec<&str> = content.split('\n').collect();
        // Trailing spaces and tabs belong to the string value. Only the final
        // whitespace-only line is formatting before the closing delimiter.
        if let Some(last) = lines.last_mut() {
            if last.trim().is_empty() {
                *last = "";
            }
        }
        // Nix strips leading spaces, not tabs or other Unicode whitespace.
        let indentation = lines
            .iter()
            .filter(|line| !line.trim_end().is_empty())
            .map(|line| line.bytes().take_while(|byte| *byte == b' ').count())
            .min()
            .unwrap_or(0);
        let multiline = lines.len() > 1;
        let unit = string_content_indent_unit(build_ctx.config.indentation);
        let content_pad = if multiline {
            unit.repeat(build_ctx.indentation)
        } else {
            String::new()
        };
        let mut interpolations = interpolations.into_iter().peekable();
        let mut offset = 0;

        for (index, original) in lines.iter().enumerate() {
            // Preserve the existing treatment of short whitespace-only lines;
            // char_indices keeps slicing valid for tabs and Unicode content.
            let skipped = original
                .char_indices()
                .nth(indentation)
                .map_or(0, |(offset, _)| offset);
            let line = &original[skipped..];
            let line_end = offset + original.len();
            if !line.is_empty() || index + 1 == lines.len() {
                if !content_pad.is_empty() {
                    steps.push(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_WHITESPACE,
                        content_pad.clone(),
                    ));
                }
                let mut portion =
                    String::with_capacity(unit.len() + line.len());
                if multiline && !line.trim().is_empty() {
                    portion.push_str(unit);
                }
                let mut cursor = offset + skipped;
                while interpolations
                    .peek()
                    .is_some_and(|(position, _)| *position < line_end)
                {
                    let (position, element) = interpolations.next().unwrap();
                    portion.push_str(&content[cursor..position]);
                    steps.push(crate::builder::Step::Token(
                        rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                        std::mem::take(&mut portion),
                    ));
                    steps.push(crate::builder::Step::Indent);
                    steps.push(crate::builder::Step::FormatWider(
                        element.clone(),
                    ));
                    steps.push(crate::builder::Step::Dedent);
                    cursor = position + 1;
                }
                portion.push_str(&content[cursor..line_end]);
                steps.push(crate::builder::Step::Token(
                    rnix::SyntaxKind::TOKEN_STRING_CONTENT,
                    portion,
                ));
            }
            if index + 1 < lines.len() {
                steps.push(crate::builder::Step::NewLine);
            }
            offset = line_end + 1;
        }
        steps.push(crate::builder::Step::FormatWider(closing.clone()));
    }

    steps
}
