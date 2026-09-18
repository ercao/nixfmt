pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = Vec::new();

    let pattern = crate::parsers::pattern::parse(build_ctx, node);

    let arguments_count = pattern.arguments.len();
    let vertical = build_ctx.vertical_due_to_width
        || node
            .children_with_tokens()
            .skip_while(|element| {
                element.kind() != rnix::SyntaxKind::TOKEN_L_BRACE
            })
            .skip(1)
            .take_while(|element| {
                element.kind() != rnix::SyntaxKind::TOKEN_R_BRACE
            })
            .any(|element| match element {
                rnix::SyntaxElement::Node(_) => false,
                rnix::SyntaxElement::Token(token) => {
                    token.text().contains('\n')
                }
            });

    // x @
    if let Some(element) = &pattern.initial_at {
        let element = element.clone();
        if vertical {
            steps.push(crate::builder::Step::FormatWider(element));
        } else {
            steps.push(crate::builder::Step::Format(element));
        }
    }

    // /**/
    if !pattern.comments_after_initial_at.is_empty() {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
        for text in pattern.comments_after_initial_at {
            steps.push(crate::builder::Step::Comment(text));
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        }
    } else if pattern.initial_at.is_some() {
        steps.push(crate::builder::Step::Whitespace);
    }

    // {
    steps.push(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_L_BRACE,
        "{".to_string(),
    ));
    if vertical {
        steps.push(crate::builder::Step::Indent);
    } else if arguments_count > 0 && build_ctx.config.space_around_brackets {
        steps.push(crate::builder::Step::Whitespace);
    }

    // arguments
    for (index, argument) in pattern.arguments.into_iter().enumerate() {
        if vertical {
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        } else if index > 0 {
            steps.push(crate::builder::Step::Whitespace);
        }

        // /**/
        if !argument.comments_before.is_empty() {
            for text in argument.comments_before {
                steps.push(crate::builder::Step::Comment(text));
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
            }
        }

        // argument
        let element = argument.item.unwrap();
        let element_kind = element.kind();
        if vertical {
            steps.push(crate::builder::Step::FormatWider(element));
        } else {
            steps.push(crate::builder::Step::Format(element));
        };

        // ,
        if vertical {
            if !matches!(element_kind, rnix::SyntaxKind::TOKEN_ELLIPSIS) {
                steps.push(crate::builder::Step::Token(
                    rnix::SyntaxKind::TOKEN_COMMA,
                    ",".to_string(),
                ));
            }
        } else if index + 1 < arguments_count {
            steps.push(crate::builder::Step::Token(
                rnix::SyntaxKind::TOKEN_COMMA,
                ",".to_string(),
            ));
        };

        // possible inline comment
        if let Some(text) = argument.comment_after {
            if text.starts_with('#') {
                steps.push(crate::builder::Step::Whitespace);
            } else {
                steps.push(crate::builder::Step::NewLine);
                steps.push(crate::builder::Step::Pad);
            }
            steps.push(crate::builder::Step::Comment(text));
        }
    }

    // /**/
    let has_comments_before_curly_b_close =
        !pattern.comments_before_curly_b_close.is_empty();
    for text in pattern.comments_before_curly_b_close {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
        steps.push(crate::builder::Step::Comment(text));
    }

    // }
    if vertical {
        steps.push(crate::builder::Step::Dedent);
        if arguments_count > 0 || has_comments_before_curly_b_close {
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        }
    } else if arguments_count > 0 && build_ctx.config.space_around_brackets {
        steps.push(crate::builder::Step::Whitespace);
    }
    steps.push(crate::builder::Step::Token(
        rnix::SyntaxKind::TOKEN_R_BRACE,
        "}".to_string(),
    ));

    // /**/
    if pattern.comments_before_end_at.is_empty() {
        if pattern.end_at.is_some() {
            steps.push(crate::builder::Step::Whitespace);
        }
    } else {
        steps.push(crate::builder::Step::NewLine);
        steps.push(crate::builder::Step::Pad);
        for text in pattern.comments_before_end_at {
            steps.push(crate::builder::Step::Comment(text));
            steps.push(crate::builder::Step::NewLine);
            steps.push(crate::builder::Step::Pad);
        }
    }

    // @ x
    if let Some(element) = pattern.end_at {
        if vertical {
            steps.push(crate::builder::Step::FormatWider(element));
        } else {
            steps.push(crate::builder::Step::Format(element));
        }
    }

    steps
}
