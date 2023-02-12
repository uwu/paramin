#[macro_export]
macro_rules! export_transformer {
	($a:expr) => {
		use swc_core::ecma::ast::Program as ___macro_Program;

		#[inline(always)]
		pub fn transformer(program: &mut ___macro_Program) {
			program.visit_mut_with(&mut $a)
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
