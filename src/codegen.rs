use std::io::Write;
use swc_core::common::sync::Lrc;

use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Config;
use swc_core::{
	common::SourceMap,
	ecma::codegen::{Emitter, Node},
};

pub fn emit_to_writer(node: &impl Node, writer: impl Write) {
	let srcmap = Lrc::new(SourceMap::default());

	let mut emitter = Emitter {
		cm: srcmap.clone(),
		comments: None,
		wr: JsWriter::new(srcmap, "\n", writer, None),
		cfg: Config {
			minify: true,
			..Config::default()
		},
	};

	node.emit_with(&mut emitter).unwrap();
}

pub fn emit_to_str(node: &impl Node) -> String {
	let mut vec = Vec::new();
	emit_to_writer(node, &mut vec);

	String::from_utf8(vec).unwrap()
}

// TODO: consider if tree walking is enough faster than stringifying to redesign this func
pub fn test_asts_equal(node1: &impl Node, node2: &impl Node) -> bool {
	emit_to_str(node1) == emit_to_str(node2)
}

#[test]
fn test_emit_to_str() {
	use swc_core::ecma::ast;

	let negate = |n: ast::Expr| {
		ast::Expr::Unary(ast::UnaryExpr {
			span: Default::default(),
			op: ast::UnaryOp::Bang,
			arg: Box::new(n),
		})
	};

	let node = ast::CondExpr {
		span: Default::default(),
		test: Box::new(negate(negate(ast::Expr::Lit(5.into())))),
		cons: Box::new(ast::Expr::Lit(ast::Lit::Str(ast::Str {
			raw: Some("testing testing 123".into()),
			value: "testing testing 123".into(),
			span: Default::default(),
		}))),
		alt: Box::new(negate(ast::Expr::Lit(ast::Lit::Str(ast::Str {
			raw: Some("".into()),
			value: "".into(),
			span: Default::default(),
		})))),
	};

	let res = emit_to_str(&node);

	assert_eq!(res, "!!5?\"testing testing 123\":!\"\"");
}
