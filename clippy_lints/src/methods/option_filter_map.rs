use crate::utils::{match_qpath, paths};
use rustc_hir::{self, Expr, ExprKind};
use std::iter;

fn calls_fn<'tcx>(expr: &'tcx [Expr<'_>], fn_name: &str) -> bool {
    expr.get(1).map_or(false, |x| match &x.kind {
        ExprKind::Path(qp) => match_qpath(
            qp,
            &paths::OPTION
                .iter()
                .cloned()
                .chain(iter::once(fn_name))
                .collect::<Vec<&str>>(),
        ),
        _ => false,
    })
}

pub fn in_scope<'tcx>(filter_args: &'tcx [Expr<'_>], map_args: &'tcx [Expr<'_>]) -> bool {
    calls_fn(map_args, "unwrap") && calls_fn(filter_args, "is_some")
}
