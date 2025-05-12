use clippy_utils::diagnostics::{span_lint, span_lint_and_help};
use rustc_ast::ast::*;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for references
    ///
    /// ### Why restrict this?
    ///
    /// References are not allowed
    ///
    /// ### Example
    /// ```no_run
    /// let a = 4;
    /// let b = &4;
    /// ```
    /// Use instead:
    /// ```no_run
    /// let a = 4;
    /// let b = &raw const 4;
    /// ```
    #[clippy::version = "1.88.0"]
    pub REFERENCE_USED,
    restriction,
    "default lint description"
}

declare_lint_pass!(ReferenceUsed => [REFERENCE_USED]);

impl EarlyLintPass for ReferenceUsed {
    fn check_pat(&mut self, cx: &EarlyContext<'_>, pat: &Pat) {
        if let PatKind::Ident(
            BindingMode::REF | BindingMode::REF_MUT | BindingMode::MUT_REF | BindingMode::MUT_REF_MUT,
            _,
            _,
        ) = &pat.kind
            && !pat.span.from_expansion()
        {
            span_lint(
                cx,
                REFERENCE_USED,
                pat.span,
                "`ref` is not allowed, as it binds by reference",
            );
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        if let ExprKind::AddrOf(BorrowKind::Ref, mutability, _) = &expr.kind {
            span_lint_and_help(
                cx,
                REFERENCE_USED,
                expr.span,
                "references are not allowed",
                None,
                format!(
                    "use a raw borrowing instead: `{}`",
                    if mutability.is_mut() { "&raw mut" } else { "&raw const" }
                ),
            );
        }
    }

    fn check_ty(&mut self, cx: &EarlyContext<'_>, ty: &Ty) {
        if let TyKind::Ref(_, mutability) = &ty.kind {
            span_lint_and_help(
                cx,
                REFERENCE_USED,
                ty.span,
                "references are not allowed",
                None,
                format!(
                    "use a raw pointer instead: `{}`",
                    if mutability.mutbl.is_mut() { "*raw" } else { "*const" }
                ),
            );
        }
    }
}
