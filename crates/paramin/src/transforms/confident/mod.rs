mod mangle;
mod swc;
use mangle::transformer as transform_mangle;
use swc::transformer as transform_swc;

use swc_core::ecma::ast::Program;

pub fn transform_confident(p: &mut Program) {
	// we leave SWC's minifier enabled, but ours helps to have too
	// compiling typescript (11MB)
	// SWC only: 3.3MB
	// my mangler only: 4.4MB
	// both manglers: 3.2MB!

	transform_mangle(p);
	transform_swc(p);
}
