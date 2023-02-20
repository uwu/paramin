mod mangle;
use mangle::transformer as transform_mangle;

use swc_core::{
	common::{sync::Lrc, Mark, SourceMap},
	ecma::{
		ast::Program,
		minifier::{
			optimize,
			option::{ExtraOptions, MinifyOptions},
		},
	},
};

pub fn transform_confident(p: &mut Program) {
	transform_mangle(p);
	transform_swc(p);
}

fn transform_swc(program: &mut Program) {
	*program = optimize(
		program.clone(),
		Lrc::new(SourceMap::default()),
		None,
		None,
		&MinifyOptions {
			// disable mangler as we will use our own
			mangle: None,
			..Default::default()
		},
		&ExtraOptions {
			unresolved_mark: Mark::new(),
			top_level_mark: Mark::new(),
		},
	);
}
