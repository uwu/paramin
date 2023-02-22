use std::collections::HashMap;

use swc_core::ecma::{
	ast,
	atoms::JsWord,
	transforms::testing::test,
	visit::{VisitMut, VisitMutWith},
};

use crate::{
	ast_utils::{
		arrow_to_function, extract_fn_shadows, extract_pat_idents, function_to_arrow,
		get_ast_idents,
	},
	export_transformer,
};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$_";
const ALPHABET_LENGTH: usize = ALPHABET.len();

#[derive(Default)]
struct MangleVisitor {
	// names that are now in use
	pub in_use_names: Vec<JsWord>,
	// source names in scope (that have been renamed)
	// only rename names with one of these!
	pub source_names: Vec<JsWord>,
	// backwards stack (5, 3 -> df)
	pub name_stack: Vec<usize>,
	// holds the renames so far in this scope
	pub renames: HashMap<JsWord, JsWord>,
}

impl MangleVisitor {
	fn sub_visitor(&self) -> MangleVisitor {
		MangleVisitor {
			//in_use_names: self.in_use_names.clone(),
			source_names: self.source_names.clone(),
			renames: self.renames.clone(),
			..Default::default()
		}
	}

	fn next_name(&mut self) -> JsWord {
		if self.name_stack.is_empty() {
			self.name_stack.push(0);

			let name = self.name_stack_to_str();
			if !self.in_use_names.contains(&name) {
				return name;
			}
		}

		loop {
			let mut okay = false;
			for i in 0..self.name_stack.len() {
				self.name_stack[i] += 1;

				if self.name_stack[i] < ALPHABET_LENGTH {
					okay = true;
					break;
				}

				self.name_stack[i] -= ALPHABET_LENGTH;
			}

			if !okay {
				self.name_stack.push(0);
			}

			let name = self.name_stack_to_str();
			if !self.in_use_names.contains(&name) {
				return name;
			}
		}
	}

	fn name_stack_to_str(&self) -> JsWord {
		let mut chars = Vec::with_capacity(self.name_stack.len());

		for c in (0..self.name_stack.len()).rev() {
			chars.push(ALPHABET[self.name_stack[c]]);
		}

		String::from_utf8(chars).unwrap().into()
	}
}

impl VisitMut for MangleVisitor {
	fn visit_mut_var_decl(&mut self, n: &mut ast::VarDecl) {
		for decl in n.decls.clone() {
			extract_pat_idents(&decl.name, &mut self.source_names);
		}

		n.visit_mut_children_with(self);
	}

	fn visit_mut_ident(&mut self, n: &mut ast::Ident) {
		if !self.source_names.contains(&n.sym) {
			return;
		}

		if let Some(replacement) = self.renames.get(&n.sym) {
			n.sym = replacement.clone();
		} else {
			let new = self.next_name();

			self.renames.insert(n.sym.clone(), new.clone());
			self.in_use_names.push(new.clone());
			n.sym = new;
		}

		n.visit_mut_children_with(self);
	}

	fn visit_mut_function(&mut self, n: &mut ast::Function) {
		let mut v = self.sub_visitor();

		extract_fn_shadows(n, &mut v.in_use_names);

		for used in get_ast_idents(n) {
			if let Some(replaced) = self.renames.get(&used) {
				v.in_use_names.push(replaced.clone())
			}
		}

		n.visit_mut_children_with(&mut v);
	}

	fn visit_mut_arrow_expr(&mut self, n: &mut ast::ArrowExpr) {
		// i love swc
		let mut func = arrow_to_function(n);
		self.visit_mut_function(&mut func);
		let new_arrow = function_to_arrow(&func);
		n.body = new_arrow.body;
		n.params = new_arrow.params;
	}
}

export_transformer!(MangleVisitor::default());

test!(
	Default::default(),
	|_| {
		use swc_core::ecma::visit::as_folder;
		as_folder(MangleVisitor::default())
	},
	mangle_test,
	r#"let foo, bar = 5; console.log(foo ?? bar + 7)"#,
	r#"let a, b = 5; console.log(a ?? b + 7)"#
);

test!(
	Default::default(),
	|_| {
		use swc_core::ecma::visit::as_folder;
		as_folder(MangleVisitor::default())
	},
	mangle_awareness_test,
	r#"let b,c,a, d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,$,_;

	(() => {
		let foo;
		console.log(b, foo);
	})();"#,
	r#"let a,b,c, d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,$,_;

	(() => {
		let b;
		console.log(a, b);
	})();"#
);
