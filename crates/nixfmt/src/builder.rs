use std::cell::RefCell;
use std::collections::HashMap;

use crate::config::Config;
use crate::config::Indentation;

#[derive(PartialEq)]
pub(crate) enum Step {
    Comment(rnix::SyntaxToken),
    Dedent,
    Format(rnix::SyntaxElement),
    FormatWider(rnix::SyntaxElement),
    Indent,
    NewLine,
    Pad,
    Token(rnix::SyntaxKind, String),
    Whitespace,
}

#[derive(Clone)]
pub(crate) struct BuildCtx<'a> {
    pub config: Config,
    pub layout_cache: &'a LayoutCache,
    pub step_buffers: &'a RefCell<Vec<Vec<Step>>>,
    pub force_wide: bool,
    pub force_wide_success: bool,
    pub force_wide_width_exceeded: bool,
    pub indentation: usize,
    pub pos_new: crate::position::Position,
    pub pos_old: crate::position::Position,
    pub path: &'a str,
    pub vertical: bool,
    pub vertical_due_to_width: bool,
}

#[derive(Default)]
pub(crate) struct LayoutCache(RefCell<HashMap<LayoutKey, bool>>);

#[derive(PartialEq, Eq, Hash)]
struct LayoutKey {
    element: rnix::SyntaxElement,
    indentation: usize,
    pos_new: crate::position::Position,
    pos_old: crate::position::Position,
}

impl LayoutKey {
    fn new(build_ctx: &BuildCtx, node: &rnix::SyntaxNode) -> Self {
        Self {
            element: node.clone().into(),
            indentation: build_ctx.indentation,
            pos_new: build_ctx.pos_new.clone(),
            pos_old: build_ctx.pos_old.clone(),
        }
    }
}

impl BuildCtx<'_> {
    pub(crate) fn take_steps(&self) -> Vec<Step> {
        self.step_buffers.borrow_mut().pop().unwrap_or_default()
    }

    /// Compute the indent string for the current indentation level,
    /// respecting the configured indentation style.
    pub(crate) fn indent_str(&self) -> String {
        let unit = match self.config.indentation {
            Indentation::FourSpaces => "    ",
            Indentation::Tabs => "\t",
            Indentation::TwoSpaces => "  ",
        };
        unit.repeat(self.indentation)
    }
}

pub(crate) struct Output {
    pub text: String,
    pub comment_offsets: Vec<usize>,
    measure_only: bool,
}

pub(crate) fn build(
    build_ctx: &mut BuildCtx,
    element: rnix::SyntaxElement,
) -> Option<Output> {
    let mut output = Output {
        text: String::with_capacity(usize::from(element.text_range().len())),
        comment_offsets: Vec::new(),
        measure_only: false,
    };

    format(&mut output, build_ctx, &element);

    if build_ctx.force_wide && !build_ctx.force_wide_success {
        None
    } else {
        Some(output)
    }
}

fn build_step(
    builder: &mut Output,
    build_ctx: &mut BuildCtx,

    step: &crate::builder::Step,
) {
    if build_ctx.force_wide && !build_ctx.force_wide_success {
        return;
    }

    match step {
        crate::builder::Step::Comment(token) => {
            let text = token.text();
            if !text.contains('\n') {
                add_token(
                    builder,
                    build_ctx,
                    rnix::SyntaxKind::TOKEN_COMMENT,
                    text.trim_end(),
                );
            } else {
                let indent = build_ctx.indent_str();
                let mut formatted = String::with_capacity(text.len());
                for (index, line) in text.lines().enumerate() {
                    let line = line.trim_end();
                    if index > 0 {
                        formatted.push('\n');
                        if !line.is_empty() {
                            formatted.push_str(&indent);
                        }
                    }
                    formatted.push_str(line);
                }
                add_token(
                    builder,
                    build_ctx,
                    rnix::SyntaxKind::TOKEN_COMMENT,
                    &formatted,
                );
            }
        }
        crate::builder::Step::Dedent => {
            build_ctx.indentation -= 1;
        }
        crate::builder::Step::Format(element) => {
            format(builder, build_ctx, element);
        }
        crate::builder::Step::FormatWider(element) => {
            format_wider(builder, build_ctx, element);
        }
        crate::builder::Step::Indent => {
            build_ctx.indentation += 1;
        }
        crate::builder::Step::NewLine => {
            build_ctx.force_wide_success = false;

            add_token(
                builder,
                build_ctx,
                rnix::SyntaxKind::TOKEN_WHITESPACE,
                "\n",
            );
        }
        crate::builder::Step::Pad => {
            if build_ctx.indentation > 0 {
                add_token(
                    builder,
                    build_ctx,
                    rnix::SyntaxKind::TOKEN_WHITESPACE,
                    &match build_ctx.config.indentation {
                        Indentation::FourSpaces => "    ",
                        Indentation::Tabs => "\t",
                        Indentation::TwoSpaces => "  ",
                    }
                    .repeat(build_ctx.indentation),
                );
            }
        }
        crate::builder::Step::Token(kind, text) => {
            add_token(builder, build_ctx, *kind, text);
        }
        crate::builder::Step::Whitespace => {
            add_token(
                builder,
                build_ctx,
                rnix::SyntaxKind::TOKEN_WHITESPACE,
                " ",
            );
        }
    }
}

fn add_token(
    builder: &mut Output,
    build_ctx: &mut BuildCtx,
    kind: rnix::SyntaxKind,
    text: &str,
) {
    if !builder.measure_only {
        if build_ctx.config.align_trailing_comments
            && kind == rnix::SyntaxKind::TOKEN_COMMENT
            && text.starts_with('#')
        {
            builder.comment_offsets.push(builder.text.len());
        }
        builder.text.push_str(text);
    }
    build_ctx.pos_new.update(text);

    if build_ctx.force_wide
        && build_ctx
            .config
            .max_width
            .is_some_and(|max_width| build_ctx.pos_new.column > max_width.get())
    {
        build_ctx.force_wide_success = false;
        build_ctx.force_wide_width_exceeded = true;
    }
}

fn format(
    builder: &mut Output,
    build_ctx: &mut BuildCtx,
    element: &rnix::SyntaxElement,
) {
    #[cfg(test)]
    tests::FORMATTED_ELEMENTS.with(|count| count.set(count.get() + 1));

    let kind = element.kind();

    match element {
        rnix::SyntaxElement::Node(node) => {
            let rule = match kind {
                // a b
                rnix::SyntaxKind::NODE_APPLY => crate::rules::apply::rule,

                // assert a; b
                rnix::SyntaxKind::NODE_ASSERT => crate::rules::scoped::rule,

                // a.b.c
                rnix::SyntaxKind::NODE_ATTRPATH => crate::rules::default,

                // a = b;
                rnix::SyntaxKind::NODE_ATTRPATH_VALUE => {
                    crate::rules::key_value::rule
                }

                // { }
                rnix::SyntaxKind::NODE_ATTR_SET => crate::rules::attr_set::rule,

                // a $op b
                rnix::SyntaxKind::NODE_BIN_OP => crate::rules::bin_op::rule,

                // __loc__ (current position)
                rnix::SyntaxKind::NODE_CUR_POS => crate::rules::default,

                // ${a} (interpolation but for NODE_SELECT)
                rnix::SyntaxKind::NODE_DYNAMIC => crate::rules::dynamic::rule,

                //
                rnix::SyntaxKind::NODE_HAS_ATTR => crate::rules::default,

                // $identifier
                rnix::SyntaxKind::NODE_IDENT => crate::rules::default,
                rnix::SyntaxKind::NODE_IDENT_PARAM => crate::rules::default,

                // if a then b else c
                rnix::SyntaxKind::NODE_IF_ELSE => crate::rules::if_else::rule,

                // inherit NODE_INHERIT_FROM? b+ ;
                rnix::SyntaxKind::NODE_INHERIT => crate::rules::inherit::rule,

                // ( a )
                rnix::SyntaxKind::NODE_INHERIT_FROM => {
                    crate::rules::paren::rule
                }

                // ${a}
                rnix::SyntaxKind::NODE_INTERPOL => crate::rules::paren::rule,

                // a: b
                rnix::SyntaxKind::NODE_LAMBDA => crate::rules::lambda::rule,

                // let { }
                rnix::SyntaxKind::NODE_LEGACY_LET => crate::rules::default,

                // let NODE_KEY_VALUE* in b;
                rnix::SyntaxKind::NODE_LET_IN => crate::rules::let_in::rule,

                // [ ... ]
                rnix::SyntaxKind::NODE_LIST => crate::rules::list::rule,

                // 1 | true | null
                rnix::SyntaxKind::NODE_LITERAL => crate::rules::default,

                // ( a )
                rnix::SyntaxKind::NODE_PAREN => crate::rules::paren::rule,

                // a | a ? b
                rnix::SyntaxKind::NODE_PAT_BIND => crate::rules::pat_bind::rule,

                // NODE_PAT_BIND | TOKEN_ELLIPSIS
                rnix::SyntaxKind::NODE_PAT_ENTRY => {
                    crate::rules::pat_entry::rule
                }

                // /path/to/${a} (absolute path)
                rnix::SyntaxKind::NODE_PATH_ABS => crate::rules::default,

                // ~/ (home path)
                rnix::SyntaxKind::NODE_PATH_HOME => crate::rules::default,

                // /path/to/${a} (relative path)
                rnix::SyntaxKind::NODE_PATH_REL => crate::rules::default,

                // <path> (search path)
                rnix::SyntaxKind::NODE_PATH_SEARCH => crate::rules::default,

                // { NODE_PAT_ENTRY* }
                rnix::SyntaxKind::NODE_PATTERN => crate::rules::pattern::rule,

                // implementation detail of rowan
                rnix::SyntaxKind::NODE_ROOT => crate::rules::root::rule,

                // a.b | a.NODE_DYNAMIC
                rnix::SyntaxKind::NODE_SELECT => crate::rules::default,

                // "..." || ''...''
                rnix::SyntaxKind::NODE_STRING => crate::rules::string::rule,

                // !a
                rnix::SyntaxKind::NODE_UNARY_OP => crate::rules::default,

                // with a; b
                rnix::SyntaxKind::NODE_WITH => crate::rules::scoped::rule,
                kind => {
                    panic!(
                        "Missing rule for {:?} at: {}",
                        kind, build_ctx.path
                    );
                }
            };

            let mut steps = rule(build_ctx, node);
            for step in &steps {
                build_step(builder, build_ctx, step);
            }
            // Reuse capacity across siblings, releasing token references before
            // returning the buffer. Recursive rules borrow separate buffers.
            steps.clear();
            build_ctx.step_buffers.borrow_mut().push(steps);
        }
        rnix::SyntaxElement::Token(token) => {
            let text = token.text();
            add_token(builder, build_ctx, kind, text);
            build_ctx.pos_old.update(text);
        }
    }
}

fn format_wider(
    builder: &mut Output,
    build_ctx: &mut BuildCtx,
    element: &rnix::SyntaxElement,
) {
    let rnix::SyntaxElement::Node(node) = element else {
        format(builder, build_ctx, element);
        return;
    };

    let mut trial = BuildCtx {
        force_wide: true,
        force_wide_success: true,
        force_wide_width_exceeded: false,
        vertical: false,
        vertical_due_to_width: false,
        ..build_ctx.clone()
    };

    // Nested trials write into the same buffer. A successful subtree is kept
    // instead of being measured and then recursively rendered a second time.
    let checkpoint = builder.text.len();
    let comment_checkpoint = builder.comment_offsets.len();
    let key = LayoutKey::new(build_ctx, node);
    let rejected = build_ctx.layout_cache.0.borrow().get(&key).copied();
    if let Some(width_exceeded) = rejected {
        trial.force_wide_success = false;
        trial.force_wide_width_exceeded = width_exceeded;
    } else {
        format(builder, &mut trial, element);
        if !trial.force_wide_success {
            // Configuration is constant for this document. Positions and
            // indentation remain part of the key because width and comment
            // dedenting depend on them. Cache failures only, not output text.
            build_ctx
                .layout_cache
                .0
                .borrow_mut()
                .insert(key, trial.force_wide_width_exceeded);
        }
    }

    if trial.force_wide_success {
        build_ctx.pos_new = trial.pos_new;
    } else {
        builder.text.truncate(checkpoint);
        builder.comment_offsets.truncate(comment_checkpoint);
        let mut fallback = BuildCtx {
            force_wide_width_exceeded: false,
            vertical: true,
            vertical_due_to_width: trial.force_wide_width_exceeded,
            ..build_ctx.clone()
        };
        format(builder, &mut fallback, element);
        build_ctx.pos_new = fallback.pos_new;
        if build_ctx.force_wide {
            // Preserve the fallback's failure reason: a child's width overflow
            // may become a newline, which must not expand its parent's braces.
            build_ctx.force_wide_success = fallback.force_wide_success;
            build_ctx.force_wide_width_exceeded |=
                fallback.force_wide_width_exceeded;
        }
    }
}

fn single_line_status(
    build_ctx_old: &crate::builder::BuildCtx,
    element: rnix::SyntaxElement,
) -> (bool, bool) {
    let key = element.as_node().map(|node| LayoutKey::new(build_ctx_old, node));
    if let Some(width_exceeded) = key
        .as_ref()
        .and_then(|key| build_ctx_old.layout_cache.0.borrow().get(key).copied())
    {
        return (false, width_exceeded);
    }

    let mut build_ctx = crate::builder::BuildCtx {
        force_wide: true,
        force_wide_success: true,
        force_wide_width_exceeded: false,
        vertical: false,
        vertical_due_to_width: false,
        ..build_ctx_old.clone()
    };

    // Rules that only ask about layout do not need an output string or tree.
    let mut output = Output {
        text: String::new(),
        comment_offsets: Vec::new(),
        measure_only: true,
    };
    format(&mut output, &mut build_ctx, &element);

    if !build_ctx.force_wide_success {
        if let Some(key) = key {
            build_ctx
                .layout_cache
                .0
                .borrow_mut()
                .insert(key, build_ctx.force_wide_width_exceeded);
        }
    }

    (build_ctx.force_wide_success, build_ctx.force_wide_width_exceeded)
}

pub(crate) fn fits_in_single_line(
    build_ctx_old: &crate::builder::BuildCtx,
    element: rnix::SyntaxElement,
) -> bool {
    single_line_status(build_ctx_old, element).0
}

pub(crate) fn make_isolated_token(
    kind: rnix::SyntaxKind,
    text: &str,
) -> rnix::SyntaxToken {
    #[cfg(test)]
    tests::ISOLATED_TOKENS.with(|count| count.set(count.get() + 1));

    use rowan::Language;

    let mut builder = rowan::GreenNodeBuilder::new();
    builder.start_node(rnix::NixLanguage::kind_to_raw(
        rnix::SyntaxKind::NODE_ROOT,
    ));
    builder.token(rnix::NixLanguage::kind_to_raw(kind), text);
    builder.finish_node();

    let node = builder.finish();

    rnix::SyntaxNode::new_root(node).first_token().unwrap()
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    thread_local! {
        pub(super) static FORMATTED_ELEMENTS: Cell<usize> = const { Cell::new(0) };
        pub(super) static ISOLATED_TOKENS: Cell<usize> = const { Cell::new(0) };
    }

    #[test]
    fn unchanged_comments_reuse_source_tokens() {
        let input = "{\n  a = 1; # first\n  b = 2; # second\n}\n";
        ISOLATED_TOKENS.with(|count| count.set(0));
        let (status, output) = crate::format::in_memory(
            "comments.nix".to_owned(),
            input.to_owned(),
            Default::default(),
        );
        assert!(matches!(status, crate::format::Status::Changed(false)));
        assert_eq!(output, input);
        assert_eq!(ISOLATED_TOKENS.with(Cell::get), 0);
    }

    #[test]
    fn compact_nesting_does_not_repeat_layout_exponentially() {
        let measure = |depth| {
            let input =
                format!("{}1{}\n", "{ a = ".repeat(depth), "; }".repeat(depth));
            FORMATTED_ELEMENTS.with(|count| count.set(0));
            let (status, output) = crate::format::in_memory(
                "nested.nix".to_owned(),
                input.clone(),
                Default::default(),
            );
            assert!(matches!(status, crate::format::Status::Changed(false)));
            assert_eq!(output, input);
            FORMATTED_ELEMENTS.with(Cell::get)
        };

        let shallow = measure(8);
        let deeper = measure(12);
        // Count work rather than elapsed time so slow CI machines cannot hide regressions.
        assert!(
            deeper <= shallow * 3,
            "8 levels: {shallow}, 12 levels: {deeper}"
        );
        assert!(measure(64) <= shallow * 16);
    }

    #[test]
    fn rejected_layouts_do_not_repeat_work_exponentially() {
        let measure = |arguments| {
            let input =
                format!("(\n f\n{})", "(/* comment */ x)\n".repeat(arguments));
            FORMATTED_ELEMENTS.with(|count| count.set(0));
            let (status, _) = crate::format::in_memory(
                "application.nix".to_owned(),
                input,
                Default::default(),
            );
            assert!(matches!(status, crate::format::Status::Changed(_)));
            FORMATTED_ELEMENTS.with(Cell::get)
        };

        let shallow = measure(8);
        let deeper = measure(12);
        assert!(
            deeper <= shallow * 3,
            "8 arguments: {shallow}, 12 arguments: {deeper}"
        );
    }
}
