use swc_core::{
	common::{sync::Lrc, Globals, Mark, SourceMap, GLOBALS},
	ecma::{
		ast::{EsVersion, Program},
		minifier::{
			optimize,
			option::{CompressOptions, ExtraOptions, MangleOptions, MinifyOptions},
		},
	},
};

pub fn transformer(program: &mut Program) {
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
				}),
			},
			&ExtraOptions {
				unresolved_mark: Mark::new(),
				top_level_mark: Mark::new(),
			},
		);
	});
}
