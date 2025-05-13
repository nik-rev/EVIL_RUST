use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::is_no_std_crate;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::DUMMY_SP;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Lints if the `#![no_std]` attribute is missing
    ///
    /// ### Why restrict this?
    ///
    /// Evil rust requires `#![no_std]` attribute to be present
    ///
    /// ### Example
    /// ```no_run
    /// fn foo() {}
    /// ```
    /// Use instead:
    /// ```no_run
    /// #![no_std]
    ///
    /// fn foo() {}
    /// ```
    #[clippy::version = "1.88.0"]
    pub MISSING_NO_STD,
    restriction,
    "default lint description"
}

declare_lint_pass!(StdUsed => [MISSING_NO_STD]);

impl LateLintPass<'_> for StdUsed {
    fn check_crate(&mut self, cx: &LateContext<'_>) {
        if !is_no_std_crate(cx) {
            span_lint_and_help(
                cx,
                MISSING_NO_STD,
                DUMMY_SP,
                "missing `#![no_std]`",
                None,
                "make this crate `#![no_std]`",
            );
        }
    }
}
