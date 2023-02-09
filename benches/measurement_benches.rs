use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paramin::measurement::get_length;

use swc_core::common::Span;
use swc_core::ecma::ast;

#[inline(always)]
fn emit_ident(str: &str) -> ast::Ident {
	 ast::Ident {
		  span: Span::default(),
		  optional: false,
		  sym: str.into(),
	 }
}

fn bench_get_length_1_iter(c: &mut Criterion) {
	 let blank_span = Span::default();

	 let ast = ast::CallExpr {
		  span: blank_span,
		  type_args: None,
		  args: vec![],
		  callee: ast::Callee::Expr(Box::new(ast::Expr::Paren(ast::ParenExpr {
				span: blank_span,
				expr: Box::new(ast::Expr::Fn(ast::FnExpr {
					 ident: Some(emit_ident("test")),
					 function: Box::new(ast::Function {
						  span: blank_span,
						  decorators: vec![],
						  is_async: false,
						  is_generator: false,
						  return_type: None,
						  type_params: None,
						  params: vec![
								ast::Param {
									 span: blank_span,
									 decorators: vec![],
									 pat: ast::Pat::Ident(ast::BindingIdent {
										  type_ann: None,
										  id: emit_ident("foo"),
									 }),
								},
								ast::Param {
									 span: blank_span,
									 decorators: vec![],
									 pat: ast::Pat::Assign(ast::AssignPat {
										  span: blank_span,
										  type_ann: None,
										  left: Box::new(ast::Pat::Ident(ast::BindingIdent {
												type_ann: None,
												id: emit_ident("bar"),
										  })),
										  right: Box::new(ast::Expr::Array(ast::ArrayLit {
												span: blank_span,
												elems: vec![],
										  })),
									 }),
								},
						  ],

						  body: Some(ast::BlockStmt {
							span: blank_span,
							stmts: vec![
								ast::Stmt::Return(ast::ReturnStmt {
									span: blank_span,
									arg: Some(Box::new(
										ast::Expr::Cond(ast::CondExpr {
											span: blank_span,
											test: Box::new(ast::Expr::Bin(ast::BinExpr {
												span: blank_span,
												op: ast::BinaryOp::EqEqEq,
												left: Box::new(ast::Expr::Member(ast::MemberExpr {
													span: blank_span,
													obj: Box::new(ast::Expr::Lit(ast::Lit::Str(
														"ab".into()
													))),
													prop: ast::MemberProp::Computed(ast::ComputedPropName {
														span: blank_span,
														expr: Box::new(ast::Expr::Lit(0.into()))
													})
												})),
												right: Box::new(ast::Expr::Member(ast::MemberExpr {
													span: blank_span,
													obj: Box::new(ast::Expr::Ident(emit_ident("foo"))),
													prop: ast::MemberProp::Computed(ast::ComputedPropName {
														span: blank_span,
														expr: Box::new(ast::Expr::Lit(1.into()))
													})
												})),
											})),

											cons: Box::new(ast::Expr::Bin(ast::BinExpr {
												span: blank_span,
												op: ast::BinaryOp::NullishCoalescing,
												left: Box::new(ast::Expr::Member(ast::MemberExpr {
													span: blank_span,
													obj: Box::new(ast::Expr::Ident(emit_ident("bar"))),
													prop: ast::MemberProp::Computed(ast::ComputedPropName {
														span: blank_span,
														expr: Box::new(ast::Expr::Lit(0.into()))
													}),
												})),
												right: Box::new(ast::Expr::Lit(1.into()))
											})),

											alt: Box::new(ast::Expr::Lit(4.into()))
										})
									))
								})
							]
						  }),
					 }),
				})),
		  }))),
	 };

	 assert_eq!(get_length(&ast), 66);
	 c.bench_function("get_length", |b| b.iter(|| get_length(black_box(&ast))));
}

criterion_group!(measurement, bench_get_length_1_iter);
criterion_main!(measurement);
