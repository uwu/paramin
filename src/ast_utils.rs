use swc_core::ecma::{
	ast,
	atoms::JsWord,
	codegen::Node,
	visit::{Visit, VisitWith},
};

use crate::codegen::emit_to_str;

// TODO: consider if tree walking is enough faster than stringifying to redesign this func
#[allow(dead_code)] // TODO: remove when not needed anymore
pub fn asts_are_equal(node1: &impl Node, node2: &impl Node) -> bool {
	emit_to_str(node1) == emit_to_str(node2)
}

pub struct IdentUseVisitor {
	found: Vec<JsWord>,
	ignore_list: Vec<JsWord>,
}

impl Visit for IdentUseVisitor {
	fn visit_ident(&mut self, n: &ast::Ident) {
		if !self.ignore_list.contains(&n.sym) && !self.found.contains(&n.sym) {
			self.found.push(n.sym.clone());
		}
	}

	fn visit_function(&mut self, n: &ast::Function) {
		let old_ignore_list = self.ignore_list.clone();

		let mut shadows = vec![];
		extract_fn_shadows(n, &mut shadows);

		for shadow in shadows {
			if !self.ignore_list.contains(&shadow) {
				self.ignore_list.push(shadow.clone());
			}
		}

		n.visit_children_with(self);

		self.ignore_list = old_ignore_list;
	}

	fn visit_arrow_expr(&mut self, n: &ast::ArrowExpr) {
		self.visit_function(&arrow_to_function(n));
	}
}

pub fn extract_ast_idents(
	node: &impl VisitWith<IdentUseVisitor>,
	target: &mut Vec<JsWord>
) {
	let mut visitor = IdentUseVisitor {
		found: vec![],
		ignore_list: vec![],
	};

	node.visit_with(&mut visitor);

	target.append(&mut visitor.found);
}

// TODO: extract_pat_idents
pub fn extract_fn_shadows(function: &ast::Function, target: &mut Vec<JsWord>) {
	for p in function.params.clone() {
		match p.pat {
			ast::Pat::Ident(id) => target.push(id.sym.clone()),
			ast::Pat::Array(_) => todo!(),
			ast::Pat::Object(_) => todo!(),
			ast::Pat::Assign(_) => todo!(),
			ast::Pat::Rest(_) => todo!(),
			ast::Pat::Invalid(_) => panic!("function params should not contain Invalid"),
			ast::Pat::Expr(_) => panic!("function params should not contain Expr"),
		}
	}
}

// swc api :bleh:
pub fn arrow_to_function(function: &ast::ArrowExpr) -> ast::Function {
	let mut params = Vec::with_capacity(function.params.len());
	for pat in function.params.clone() {
		params.push(ast::Param {
			span: Default::default(),
			pat,
			decorators: vec![],
		});
	}

	let body = match function.body.clone() {
		ast::BlockStmtOrExpr::BlockStmt(s) => s,
		ast::BlockStmtOrExpr::Expr(e) => ast::BlockStmt {
			span: Default::default(),
			stmts: vec![ast::Stmt::Expr(ast::ExprStmt {
				span: Default::default(),
				expr: e,
			})],
		},
	};

	ast::Function {
		span: function.span,
		decorators: vec![],
		is_async: function.is_async,
		is_generator: function.is_generator,
		type_params: function.type_params.clone(),
		return_type: function.return_type.clone(),
		params,
		body: Some(body),
	}
}

pub fn function_to_arrow(function: &ast::Function) -> ast::ArrowExpr {
	let mut params = Vec::with_capacity(function.params.len());
	for param in function.params.clone() {
		params.push(param.pat);
	}

	let body = match function.body.clone() {
    Some(b) => b,
    None => ast::BlockStmt {
		span: Default::default(),
		stmts: vec![]
	 },
};

	ast::ArrowExpr {
		span: function.span,
		is_async: function.is_async,
		is_generator: function.is_generator,
		type_params: function.type_params.clone(),
		return_type: function.return_type.clone(),
		params,
		body: ast::BlockStmtOrExpr::BlockStmt(body),
	}
}

#[test]
fn test_extract_fn_shadows() {
	let param = |name: &str| ast::Param {
		decorators: vec![],
		span: Default::default(),
		pat: ast::Pat::Ident(ast::BindingIdent {
			type_ann: None,
			id: ast::Ident {
				span: Default::default(),
				optional: false,
				sym: name.into(),
			},
		}),
	};

	let f = ast::Function {
		decorators: vec![],
		is_async: false,
		is_generator: false,
		return_type: None,
		span: Default::default(),
		type_params: None,
		body: None,

		params: vec![param("foo"), param("bar")],
	};

	let mut shadows = vec![];
	extract_fn_shadows(&f, &mut shadows);

	let expected: Vec<JsWord> = vec!["foo".into(), "bar".into()];

	assert_eq!(shadows, expected);
}

#[test]
fn test_extract_ast_idents() {
	let param = |name: &str| {
		ast::Pat::Ident(ast::BindingIdent {
			type_ann: None,
			id: ast::Ident {
				span: Default::default(),
				optional: false,
				sym: name.into(),
			},
		})
	};

	let ident = |name: &str| {
		Box::new(ast::Expr::Ident(ast::Ident {
			span: Default::default(),
			optional: false,
			sym: name.into(),
		}))
	};

	let ast = ast::BlockStmt {
		span: Default::default(),
		stmts: vec![
			ast::Stmt::Expr(ast::ExprStmt {
				span: Default::default(),
				expr: Box::new(ast::Expr::Seq(ast::SeqExpr {
					span: Default::default(),
					exprs: vec![ident("foo"), ident("bar")],
				})),
			}),
			ast::Stmt::Expr(ast::ExprStmt {
				span: Default::default(),
				expr: Box::new(ast::Expr::Arrow(ast::ArrowExpr {
					is_async: false,
					is_generator: false,
					span: Default::default(),
					return_type: None,
					type_params: None,
					params: vec![param("baz")],
					body: ast::BlockStmtOrExpr::Expr(Box::new(ast::Expr::Ident(ast::Ident {
						optional: false,
						span: Default::default(),
						sym: "baz".into(),
					}))),
				})),
			}),
		],
	};

	let mut results = vec![];
	extract_ast_idents(&ast, &mut results);
	let expected: Vec<JsWord> = vec!["foo".into(), "bar".into()];

	assert_eq!(results, expected);
}
