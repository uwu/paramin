mod mangle;
mod hoist_vars;
pub use mangle::transformer as transform_mangle;
pub use hoist_vars::transformer as transform_hoist_vars;
use swc_core::ecma::ast::Program;

// runs all confident transforms in optimal order
pub fn transform_all_confident(p: Program) -> Program {
	transform_hoist_vars(
		transform_mangle(p)
	)
}