use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_ast::ast::*;
use rustc_ast::visit::{FnCtxt, FnKind};
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for variables which are missing a `mut`.
    ///
    /// ### Why restrict this?
    ///
    /// Goes against the rules of evil rust.
    ///
    /// ### Example
    /// ```no_run
    /// let a = 4;
    /// ```
    /// Use instead:
    /// ```no_run
    /// let mut a = 4;
    /// ```
    #[clippy::version = "1.88.0"]
    pub MISSING_MUT,
    evil,
    "default lint description"
}

declare_lint_pass!(MissingMut => [MISSING_MUT]);

fn absorb_kind(cx: &EarlyContext<'_>, kind: &PatKind, message: &'static str, help: &'static str) {
    match kind {
        PatKind::Ident(mode, ident, _) => {
            // there are 6 binding modes
            // 4 of them are `mut`
            // 2 are not
            if *mode == BindingMode::NONE || *mode == BindingMode::REF {
                span_lint_and_sugg(
                    cx,
                    MISSING_MUT,
                    ident.span.shrink_to_lo(),
                    message,
                    help,
                    "mut ".to_string(),
                    Applicability::MachineApplicable,
                );
            }
        },
        PatKind::Slice(fields) | PatKind::TupleStruct(_, _, fields) | PatKind::Tuple(fields) | PatKind::Or(fields) => {
            for field in fields {
                absorb_kind(cx, &field.kind, message, help);
            }
        },
        PatKind::Struct(_, _, fields, _) => {
            for field in fields {
                absorb_kind(cx, &field.pat.kind, message, help);
            }
        },
        PatKind::Guard(field, _) | PatKind::Box(field) | PatKind::Deref(field) | PatKind::Paren(field) => {
            absorb_kind(cx, &field.kind, message, help);
        },
        PatKind::Ref(field, mutable) => {
            absorb_kind(cx, &field.kind, message, help);
            if mutable.is_not() {
                span_lint_and_sugg(
                    cx,
                    MISSING_MUT,
                    field.span.shrink_to_lo(),
                    "function parameter is not mutable",
                    "make this parameter mutable",
                    "mut ".to_string(),
                    Applicability::MachineApplicable,
                );
            }
        },
        _ => (),
    }
}

impl EarlyLintPass for MissingMut {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, f: FnKind<'_>, _: rustc_span::Span, _: NodeId) {
        if let FnKind::Fn(fn_cx, _, f) = f &&
        // functions inside `extern ... {}` block cannot have `mut` args, as they don't support patterns
         fn_cx != FnCtxt::Foreign
        {
            // function params
            for param in &f.sig.decl.inputs {
                absorb_kind(
                    cx,
                    &param.pat.kind,
                    "parameter is not mutable",
                    "make this parameter mutable",
                );
            }
        }
    }

    // local variables
    fn check_local(&mut self, cx: &EarlyContext<'_>, local: &Local) {
        absorb_kind(
            cx,
            &local.pat.kind,
            "variable is not mutable",
            "make this variable mutable",
        );
    }

    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        // static
        if let ItemKind::Static(stati) = &item.kind
            && stati.mutability == Mutability::Not
        {
            span_lint_and_sugg(
                cx,
                MISSING_MUT,
                stati.ident.span.shrink_to_lo(),
                "static is not mutable",
                "make this static mutable",
                "mut ".to_string(),
                Applicability::MachineApplicable,
            );
        }
    }
}
