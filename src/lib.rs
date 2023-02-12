mod codegen;
pub mod measurement;
mod transforms;

use swc_core::ecma::ast::Program;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[plugin_transform]
pub fn transformer(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
	program
}
