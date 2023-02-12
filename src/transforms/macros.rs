#[macro_export]
macro_rules! export_transformer {
	($a:expr) => {
		use swc_core::ecma::{
			ast::Program as ___macro_Program,
			visit::{as_folder as ___macro__as_folder, FoldWith as ___macro__FoldWith},
		};

		#[inline(always)]
		pub fn transformer(program: ___macro_Program) -> ___macro_Program {
			program.fold_with(&mut ___macro__as_folder($a))
		}
	};
}

#[macro_export]
macro_rules! test {
	($a:ty, $b:ident, $c:expr, $d:expr) => {
		use swc_core::ecma::transforms::testing::test as ___macro_test;

		___macro_test!(
			Default::default(),
			|_| {
				use swc_core::ecma::visit::as_folder;
				as_folder::<$a>(Default::default())
			},
			$b,
			$c,
			$d
		);
	};
}
