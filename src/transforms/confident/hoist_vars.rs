use swc_core::ecma::{
	ast,
	visit::{VisitMut, VisitMutWith},
};

use crate::{export_transformer, test};

fn collect_stmts(stmts: &mut Vec<ast::Stmt>) {
	let mut all_decls = vec![];
	let mut new_stmts = Vec::with_capacity(stmts.len());

	for stmt in &mut *stmts {
		if let ast::Stmt::Decl(ast::Decl::Var(v)) = stmt {
			all_decls.append(&mut v.decls.clone());
		} else {
			new_stmts.push(stmt.clone());
		}
	}

	if !all_decls.is_empty() {
		new_stmts.insert(
			0,
			ast::Stmt::Decl(ast::Decl::Var(Box::new(ast::VarDecl {
				span: Default::default(),
				declare: false,
				kind: ast::VarDeclKind::Var,
				decls: all_decls,
			}))),
		);
	}

	new_stmts.shrink_to_fit();

	stmts.clear();
	stmts.append(&mut new_stmts);
}

fn collect_module_items(moditems: &mut Vec<ast::ModuleItem>) {
	let mut all_decls = vec![];
	let mut new_moditems = Vec::with_capacity(moditems.len());

	for moditem in &mut *moditems {
		if let ast::ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(v))) = moditem {
			all_decls.append(&mut v.decls.clone());
		} else {
			new_moditems.push(moditem.clone());
		}
	}

	if !all_decls.is_empty() {
		new_moditems.insert(
			0,
			ast::ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(Box::new(ast::VarDecl {
				span: Default::default(),
				declare: false,
				kind: ast::VarDeclKind::Var,
				decls: all_decls,
			})))),
		);
	}

	new_moditems.shrink_to_fit();

	moditems.clear();
	moditems.append(&mut new_moditems);
}

#[derive(Default)]
struct HoistVarsVistor;

impl VisitMut for HoistVarsVistor {
	fn visit_mut_block_stmt(&mut self, n: &mut ast::BlockStmt) {
		collect_stmts(&mut n.stmts);
		n.visit_mut_children_with(self);
	}

	fn visit_mut_script(&mut self, n: &mut ast::Script) {
		collect_stmts(&mut n.body);
		n.visit_mut_children_with(self);
	}

	fn visit_mut_module(&mut self, n: &mut ast::Module) {
		collect_module_items(&mut n.body);
		n.visit_mut_children_with(self);
	}
}

export_transformer!(HoistVarsVistor);

test!(
	HoistVarsVistor,
	hoist_vars_test,
	r#"let a = 5, b = 6;
	console.log(a);
	const x = y;
	console.log(`${b}${x}`);"#,
	r#"var a = 5, b = 6, x = y;
	console.log(a);
	console.log(`${b}${x}`);"#
);
