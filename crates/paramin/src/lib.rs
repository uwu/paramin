mod codegen;
pub mod measurement;
mod transforms;
mod ast_utils;

use swc_core::ecma::ast::Program;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub fn simple_transform(program: &mut Program) {
	transforms::confident::transform_confident(program);
}

#[plugin_transform]
pub fn transformer(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
	transforms::confident::transform_confident(&mut program);

	program
}
