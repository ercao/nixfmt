pub(crate) mod apply;
pub(crate) mod attr_set;
pub(crate) mod bin_op;
pub(crate) mod dynamic;
pub(crate) mod if_else;
pub(crate) mod inherit;
pub(crate) mod key_value;
pub(crate) mod lambda;
pub(crate) mod let_in;
pub(crate) mod list;
pub(crate) mod paren;
pub(crate) mod pat_bind;
pub(crate) mod pat_entry;
pub(crate) mod pattern;
pub(crate) mod root;
pub(crate) mod scoped;
pub(crate) mod string;

pub(crate) fn default(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> Vec<crate::builder::Step> {
    let mut steps = build_ctx.take_steps();
    steps.extend(node.children_with_tokens().map(crate::builder::Step::Format));
    steps
}
