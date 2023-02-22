mod mangle;
use mangle::transformer as transform_mangle;

use swc_core::{
	common::{sync::Lrc, Globals, Mark, SourceMap, GLOBALS},
	ecma::{
		ast::{Program, EsVersion},
		minifier::{
			optimize,
			option::{ExtraOptions, MinifyOptions, CompressOptions, MangleOptions},
		},
	},
};

pub fn transform_confident(p: &mut Program) {
	// we leave SWC's minifier enabled, but ours helps to have too
	// compiling typescript (11MB)
	// SWC only: 3.3MB
	// my mangler only: 4.4MB
	// both manglers: 3.2MB!

	transform_mangle(p);
	transform_swc(p);
}

fn transform_swc(program: &mut Program) {
	let globals = Globals::new();
	GLOBALS.set(&globals, || {
		*program = optimize(
			program.clone(),
			Lrc::new(SourceMap::default()),
			None,
			None,
			&MinifyOptions {
				mangle: Some(MangleOptions::default()),
				enclose: false,
				rename: false,
				wrap: false,
				compress: Some(CompressOptions {
					ecma: EsVersion::EsNext,
					hoist_vars: true,
					hoist_props: true,
					hoist_fns: true,
					module: true,
					negate_iife: false,
					..Default::default()
				})
			},
			&ExtraOptions {
				unresolved_mark: Mark::new(),
				top_level_mark: Mark::new(),
			},
		);
	});
}
