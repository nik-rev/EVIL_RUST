use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::HasSession;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why restrict this?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.88.0"]
    pub MISSING_PUB,
    restriction,
    "default lint description"
}

declare_lint_pass!(MissingPub => [MISSING_PUB]);

impl EarlyLintPass for MissingPub {
    fn check_ty(&mut self, _: &EarlyContext<'_>, _: &rustc_ast::Ty) {}

    fn check_expr(&mut self, _: &EarlyContext<'_>, _: &rustc_ast::Expr) {}

    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &rustc_ast::Item) {
        if !item.vis.kind.is_pub() && item.vis.span.is_visible(cx.sess().source_map()) {
            span_lint_and_sugg(
                cx,
                MISSING_PUB,
                item.vis.span,
                "item is not public",
                "make this item public",
                "pub ".to_string(),
                rustc_errors::Applicability::MachineApplicable,
            );
        }
    }
}
