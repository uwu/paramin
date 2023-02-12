use std::collections::HashMap;

use swc_core::ecma::{
	ast,
	atoms::JsWord,
	transforms::testing::test,
	visit::{VisitMut, VisitMutWith},
};

use crate::export_transformer;

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ$_";
const ALPHABET_LENGTH: usize = ALPHABET.len();

#[derive(Default)]
struct MangleVisitor {
	// names in use (some may be shadowable but not all)
	// if a name isnt in here, halt and catch fire
	pub defined_names: Vec<JsWord>,
	// backwards stack (5, 3 -> df)
	pub name_stack: Vec<usize>,
	// holds the renames so far in this scope
	pub renames: HashMap<JsWord, JsWord>,
}

impl MangleVisitor {
	fn halt_and_catch_fire(&self) {
		panic!("halt & catch fire in MangleVisitor");
	}

	fn sub_visitor(&self) -> MangleVisitor {
		MangleVisitor {
			defined_names: self.defined_names.clone(),
			..Default::default()
		}
	}

	fn next_name(&mut self) -> JsWord {
		if self.name_stack.is_empty() {
			self.name_stack.push(0);
			std::str::from_utf8(&[ALPHABET[0]][..]).unwrap().into()
		} else {
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

				// TODO: some of these can be safely shadowed! (perhaps look at some code from terser or something)
				let name = self.name_stack_to_str();
				if !self.defined_names.contains(&name) {
					return name;
				}
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
			match decl.name {
				ast::Pat::Ident(id) => self.defined_names.push(id.id.sym),
				ast::Pat::Array(_) => todo!(),
				ast::Pat::Object(_) => todo!(),
				ast::Pat::Rest(_) => self.halt_and_catch_fire(),
				ast::Pat::Assign(_) => self.halt_and_catch_fire(),
				ast::Pat::Invalid(_) => self.halt_and_catch_fire(),
				ast::Pat::Expr(_) => self.halt_and_catch_fire(),
			}
		}

		n.visit_mut_children_with(self);
	}

	fn visit_mut_ident(&mut self, n: &mut ast::Ident) {
		if !self.defined_names.contains(&n.sym) {
			return;
		}

		if let Some(replacement) = self.renames.get(&n.sym) {
			n.sym = replacement.clone();
		} else {
			let new = self.next_name();

			self.renames.insert(n.sym.clone(), new.clone());
			n.sym = new;
		}

		n.visit_mut_children_with(self);
	}

	fn visit_mut_function(&mut self, n: &mut ast::Function) {
		let mut v = self.sub_visitor();

		for p in n.params.clone() {
			match p.pat {
				ast::Pat::Ident(id) => v.defined_names.push(id.sym.clone()),
				ast::Pat::Array(_) => todo!(),
				ast::Pat::Object(_) => todo!(),
				ast::Pat::Assign(_) => todo!(),
				ast::Pat::Rest(_) => todo!(),
				ast::Pat::Invalid(_) => self.halt_and_catch_fire(),
				ast::Pat::Expr(_) => self.halt_and_catch_fire(),
			}
		}

		n.visit_mut_children_with(&mut v);
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
