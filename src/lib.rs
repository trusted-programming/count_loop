#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_lint::{EarlyContext, EarlyLintPass};

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub COUNT_LOOP,
    Warn,
    "description goes here"
}

impl<'tcx> EarlyLintPass for CountLoop {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &rustc_ast::Expr) {
        if let rustc_ast::ExprKind::ForLoop {
            pat: _,
            iter: _,
            body: _,
            label: _,
            kind: _,
        } = &expr.kind
        {
            span_lint_and_help(
                cx,
                COUNT_LOOP,
                expr.span,
                &format!("found for loop, code: 213423"),
                None,
                "",
            );
        }
        if let rustc_ast::ExprKind::MethodCall(method_call) = &expr.kind {
            let iters = vec!["iter", "into_iter", "iter_mut"];
            if iters.contains(&method_call.seg.ident.as_str()) {
                span_lint_and_help(
                    cx,
                    COUNT_LOOP,
                    expr.span,
                    &format!("found iterator, code: 213932"),
                    None,
                    "",
                );
            }
        }
        if let rustc_ast::ExprKind::MethodCall(method_call) = &expr.kind {
            let iters = vec!["par_iter", "into_par_iter", "par_iter_mut"];
            if iters.contains(&method_call.seg.ident.as_str()) {
                span_lint_and_help(
                    cx,
                    COUNT_LOOP,
                    expr.span,
                    &format!("found par iterator, code: 213312"),
                    None,
                    "",
                );
            }
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
