#[macro_export]
macro_rules! export_transformer {
	 ($a:expr) => {
		use swc_core::ecma::{
			ast::Program,
			visit::{as_folder, FoldWith},
		};
		use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

		#[plugin_transform]
		pub fn transformer(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
			program.fold_with(&mut as_folder($a))
		}
	 };
}