use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::ast::*;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::BytePos;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for usages of safe functions
    ///
    /// ### Why restrict this?
    ///
    /// Safe functions are prohibited in EVIL RUST
    ///
    /// ### Example
    /// ```no_run
    /// fn foo() {}
    /// ```
    /// Use instead:
    /// ```no_run
    /// unsafe foo() {}
    /// ```
    #[clippy::version = "1.88.0"]
    pub SAFE_FN,
    evil,
    "safe functions are not allowed in EVIL RUST"
}

declare_lint_pass!(SafeFn => [SAFE_FN]);

impl EarlyLintPass for SafeFn {
    fn check_fn(
        &mut self,
        cx: &EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        _: rustc_span::Span,
        _: NodeId,
    ) {
        if let rustc_ast::visit::FnKind::Fn(_, _, func) = fn_kind {
            if func.sig.header.safety == Safety::Default {
                let span = if let Extern::Implicit(span) | Extern::Explicit(_, span) = func.sig.header.ext {
                    // `ItemSafety` must come before the `extern`, if it exists
                    span.shrink_to_lo()
                } else {
                    func.ident
                        .span
                        .with_lo(func.ident.span.lo() - BytePos("fn ".len() as u32))
                        .shrink_to_lo()
                    // If `extern` does not exist, `ItemSafety` comes before the `fn`
                };
                span_lint_and_help(
                    cx,
                    SAFE_FN,
                    span,
                    "safe function",
                    Some(span),
                    // insert `unsafe`
                    "make this function unsafe: `unsafe` ".to_string(),
                );
            } else if let Safety::Safe(span) = func.sig.header.safety {
                span_lint_and_help(
                    cx,
                    SAFE_FN,
                    span,
                    "safe function",
                    Some(span),
                    // replace `safe` keyword with `unsafe`
                    "make this function unsafe: `unsafe fn`".to_string(),
                );
            }
        }
    }
}
