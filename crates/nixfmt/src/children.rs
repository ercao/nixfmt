pub(crate) struct Children {
    children: Vec<rnix::SyntaxElement>,
    current_index: usize,
    has_comments: bool,
    has_newlines: bool,
}

pub(crate) enum Trivia {
    Comment(rnix::SyntaxToken),
    // Spaces are discarded by Children; retain only the number of line breaks.
    Whitespace(usize),
}

impl Children {
    pub fn new(
        build_ctx: &crate::builder::BuildCtx,
        node: &rnix::SyntaxNode,
    ) -> Children {
        let mut children: Vec<rnix::SyntaxElement> = Vec::new();

        // Only block comments need their original column. Advance this second
        // iterator lazily, without flattening preceding subtrees into strings.
        let mut position_children = node.children_with_tokens();
        let mut pos = build_ctx.pos_old.clone();
        let mut has_comments = false;
        let mut has_newlines = false;

        for child in node.children_with_tokens() {
            match child {
                rnix::SyntaxElement::Node(node) => match node.kind() {
                    rnix::SyntaxKind::NODE_PAREN => {
                        let mut simplified = node.clone();

                        while matches!(
                            simplified.kind(),
                            rnix::SyntaxKind::NODE_PAREN
                        ) {
                            let mut children =
                                crate::annotated_children::annotated(
                                    build_ctx,
                                    &simplified,
                                );

                            let opener = children.next().unwrap();
                            let expression = children.next().unwrap();
                            let closer = children.next().unwrap();

                            if !opener.has_inline_comment
                                && !opener.has_comments
                                && !expression.has_inline_comment
                                && !expression.has_comments
                                && !closer.has_inline_comment
                                && !closer.has_comments
                                && matches!(
                                    expression.element.kind(),
                                    rnix::SyntaxKind::NODE_ATTR_SET
                                        | rnix::SyntaxKind::NODE_IDENT
                                        | rnix::SyntaxKind::NODE_LIST
                                        | rnix::SyntaxKind::NODE_LITERAL
                                        | rnix::SyntaxKind::NODE_PAREN
                                        | rnix::SyntaxKind::NODE_INTERPOL
                                        | rnix::SyntaxKind::NODE_STRING
                                )
                            {
                                simplified =
                                    expression.element.into_node().unwrap();
                            } else {
                                break;
                            }
                        }

                        children.push(simplified.into());
                    }
                    _ => {
                        children.push(node.clone().into());
                    }
                },

                rnix::SyntaxElement::Token(token) => match token.kind() {
                    rnix::SyntaxKind::TOKEN_COMMENT => {
                        has_comments = true;
                        if token.text().starts_with('#') {
                            children.push(token.clone().into());
                        } else {
                            let current = token.clone().into();
                            for preceding in position_children.by_ref() {
                                if preceding == current {
                                    break;
                                }
                                match preceding {
                                    rnix::SyntaxElement::Node(node) => {
                                        node.text().for_each_chunk(|text| {
                                            pos.update(text)
                                        });
                                    }
                                    rnix::SyntaxElement::Token(token) => {
                                        pos.update(token.text())
                                    }
                                }
                            }
                            let text = dedent_comment(&pos, token.text());
                            pos.update(token.text());
                            if text == token.text() {
                                children.push(token.clone().into());
                            } else {
                                children.push(
                                    crate::builder::make_isolated_token(
                                        rnix::SyntaxKind::TOKEN_COMMENT,
                                        &text,
                                    )
                                    .into(),
                                );
                            }
                        }
                    }
                    rnix::SyntaxKind::TOKEN_WHITESPACE => {
                        if crate::utils::has_newlines(token.text()) {
                            has_newlines = true;
                            children.push(token.clone().into());
                        }
                    }
                    _ => {
                        children.push(token.clone().into());
                    }
                },
            }
        }

        Children { children, current_index: 0, has_comments, has_newlines }
    }

    pub fn get(&mut self, index: usize) -> Option<rnix::SyntaxElement> {
        if index + 1 > self.children.len() {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    pub fn get_next(&mut self) -> Option<rnix::SyntaxElement> {
        let child = self.get(self.current_index);
        self.move_next();
        child
    }

    pub fn get_remaining(&mut self) -> Vec<rnix::SyntaxElement> {
        let remaining = &self.children[self.current_index..self.children.len()];
        self.current_index = self.children.len();
        remaining.to_vec()
    }

    pub fn has_next(&self) -> bool {
        self.current_index < self.children.len()
    }

    pub fn peek_next(&mut self) -> Option<rnix::SyntaxElement> {
        self.get(self.current_index)
    }

    pub fn move_next(&mut self) {
        self.current_index += 1
    }

    pub fn move_prev(&mut self) {
        self.current_index -= 1
    }

    pub fn has_comments(&self) -> bool {
        self.has_comments
    }

    pub fn has_newlines(&self) -> bool {
        self.has_newlines
    }

    pub fn drain_trivia<F: FnMut(Trivia)>(&mut self, mut callback: F) {
        while let Some(child) = self.peek_next() {
            match child.kind() {
                rnix::SyntaxKind::TOKEN_COMMENT => {
                    callback(Trivia::Comment(child.into_token().unwrap()));
                    self.move_next();
                }
                rnix::SyntaxKind::TOKEN_WHITESPACE => {
                    callback(Trivia::Whitespace(crate::utils::count_newlines(
                        child.as_token().unwrap().text(),
                    )));
                    self.move_next();
                }
                _ => {
                    break;
                }
            }
        }
    }
}

fn dedent_comment(pos: &crate::position::Position, text: &str) -> String {
    if text.starts_with('#') {
        text.to_string()
    } else {
        let is_doc = text.starts_with("/**") && !text.starts_with("/**/");
        let mut lines: Vec<String> = if is_doc {
            text[3..text.len() - 2]
                .lines()
                .map(|line| line.to_string())
                .collect()
        } else {
            text[2..text.len() - 2]
                .lines()
                .map(|line| line.to_string())
                .collect()
        };

        // If all lines are whitespace just return a compact comment
        if lines.iter().all(|line| line.trim().is_empty()) {
            if is_doc {
                return "/***/".to_string();
            } else {
                return "/**/".to_string();
            }
        }

        // Make sure it starts with empty line
        if lines.len() == 1 {
            lines.insert(0, "".to_string());
            lines[1] = format!("{0:<1$}{2}", "", pos.column + 2, lines[1]);
        } else if lines[0].trim().is_empty() {
            lines[0] = "".to_string();
        } else {
            lines.insert(0, format!("{0:<1$}", "", pos.column + 1));
            lines[1] = format!("{0:<1$}{2}", "", pos.column + 2, lines[1]);
        }

        // Make sure it ends with empty line
        let len = lines.len();
        if len == 2 {
            lines.push(format!("{0:<1$}", "", pos.column + 1));
        } else if lines[len - 1].trim().is_empty() {
            lines[len - 1] = format!("{0:<1$}", "", pos.column + 1)
        } else {
            lines.push(format!("{0:<1$}", "", pos.column + 1));
        }

        // Compute the distance to the first character from the left
        let mut indentation: usize = usize::MAX;
        for (index, line) in lines.iter().enumerate() {
            if index != 0 && index + 1 != lines.len() {
                let line = line.trim_end();

                if !line.is_empty() {
                    indentation = usize::min(
                        indentation,
                        line.len() - line.trim_start().len(),
                    );
                }
            }
        }
        if indentation == usize::MAX {
            indentation = pos.column;
        };

        // Re-align everything with respect to the vertical
        lines = lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 || index + 1 == lines.len() {
                    line.to_string()
                } else if pos.column >= indentation {
                    format!(
                        "{0:<1$}{2}",
                        "",
                        pos.column - indentation + 1,
                        line,
                    )
                } else if line.len() >= indentation - pos.column {
                    line[indentation - pos.column - 1..line.len()].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        // Dedent everything as much as possible so that upstream components
        // can indent as they see convenient
        lines = lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == 0 {
                    line.to_string()
                } else if line.len() > pos.column {
                    line[pos.column + 1..line.len()].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();
        if is_doc {
            format!("/**{}*/", lines.join("\n"))
        } else {
            format!("/*{}*/", lines.join("\n"))
        }
    }
}
