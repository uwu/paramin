mod codegen;
pub mod measurement;
mod transforms;

use swc_core::ecma::ast::Program;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[plugin_transform]
pub fn transformer(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
	transforms::confident::transform_all_confident(&mut program);

	program
}
